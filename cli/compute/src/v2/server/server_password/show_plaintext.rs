// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Show plaintext server password command

use clap::Args;
use dialoguer::Password;
use std::path::{Path, PathBuf};
use tracing::info;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_cli_core::output::OutputProcessor;
use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::server_password::get;
use openstack_types::compute::v2::server::server_password::response;

use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use ssh_key::PrivateKey;

/// Retrieve and decrypt the administrative password for a server.
///
/// The password is encrypted with the SSH public key injected at boot time.
/// Provide the matching SSH private key to obtain the plaintext password.
///
/// Supports OpenSSH (`BEGIN OPENSSH PRIVATE KEY`), PEM RSA
/// (`BEGIN RSA PRIVATE KEY`), and PKCS#8 (`BEGIN PRIVATE KEY`,
/// `BEGIN ENCRYPTED PRIVATE KEY`) key formats. Passphrase-protected
/// OpenSSH and PKCS#8 keys trigger an interactive prompt.
///
/// To retrieve the raw (encrypted) password instead, use
/// `osc compute server password show`.
#[derive(Args)]
#[command(about = "Show Server Password (decrypted)")]
pub struct ShowPlaintextCommand {
    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Path to the SSH private key used to decrypt the password.
    /// Supports OpenSSH, PEM RSA, and PKCS#8 key formats.
    /// If the key is passphrase-protected you will be prompted interactively.
    #[arg(
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::FilePath,
        help_heading = "Decryption"
    )]
    private_key: PathBuf,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/os-server-password API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}

impl ShowPlaintextCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ServerPassword (plaintext)");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.server/server_password"),
            Some("get"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();
        ep_builder.server_id(&self.path.server_id);
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: serde_json::Value = ep.query_async(client).await?;

        let encrypted_b64 = data.get("password").and_then(|v| v.as_str()).unwrap_or("");

        if encrypted_b64.is_empty() {
            return Err(OpenStackCliError::InputParameters(
                "no password is set for this server".into(),
            ));
        }

        let plaintext = decrypt_password(&self.private_key, encrypted_b64)?;
        let decrypted = serde_json::json!({ "password": plaintext });
        op.output_single::<response::get::ServerPasswordResponse>(decrypted)?;
        op.show_command_hint()?;
        Ok(())
    }
}

/// Load an RSA private key from a PEM file.
///
/// Supported formats:
/// - OpenSSH (`BEGIN OPENSSH PRIVATE KEY`) — passphrase-protected keys prompt
///   interactively.
/// - Traditional PEM RSA (`BEGIN RSA PRIVATE KEY`)
/// - PKCS#8 PEM (`BEGIN PRIVATE KEY`, `BEGIN ENCRYPTED PRIVATE KEY`) —
///   encrypted keys prompt interactively.
///
/// Encrypted traditional PEM keys (`Proc-Type: 4,ENCRYPTED` headers) return
/// a conversion hint.
fn load_rsa_key(key_path: &Path) -> Result<RsaPrivateKey, OpenStackCliError> {
    let content = std::fs::read_to_string(key_path)?;
    load_rsa_key_from_pem(&content, &prompt_passphrase)
}

/// Prompt the user interactively for the private key passphrase.
fn prompt_passphrase() -> Result<String, OpenStackCliError> {
    Ok(Password::new()
        .with_prompt("Private key passphrase")
        .interact()?)
}

/// Parse an RSA private key from PEM content, dispatching on the PEM tag.
///
/// `passphrase_prompt` is invoked when the key is passphrase-protected;
/// injectable for testing.
fn load_rsa_key_from_pem(
    content: &str,
    passphrase_prompt: &dyn Fn() -> Result<String, OpenStackCliError>,
) -> Result<RsaPrivateKey, OpenStackCliError> {
    let document = pem::parse(content).map_err(|e| {
        OpenStackCliError::InputParameters(format!(
            "unrecognized private key format (expected OpenSSH, PEM RSA, or PKCS#8): {e}"
        ))
    })?;

    match document.tag() {
        "OPENSSH PRIVATE KEY" => {
            // ssh-key parses the full armored text, not the decoded contents.
            let private_key = PrivateKey::from_openssh(content).map_err(|e| {
                OpenStackCliError::InputParameters(format!("failed to parse SSH key: {e}"))
            })?;

            let private_key = if private_key.is_encrypted() {
                let passphrase = passphrase_prompt()?;
                private_key.decrypt(passphrase.as_bytes()).map_err(|e| {
                    OpenStackCliError::InputParameters(format!(
                        "failed to decrypt SSH key (wrong passphrase?): {e}"
                    ))
                })?
            } else {
                private_key
            };

            let rsa_keypair = private_key.key_data().rsa().ok_or_else(|| {
                OpenStackCliError::InputParameters(
                    "private key must be RSA (Ed25519/ECDSA keys are not supported)".into(),
                )
            })?;

            RsaPrivateKey::try_from(rsa_keypair).map_err(|e| {
                OpenStackCliError::InputParameters(format!("failed to extract RSA key: {e}"))
            })
        }
        "RSA PRIVATE KEY" => {
            // Traditional PEM signals encryption via RFC 1421 headers.
            if document
                .headers()
                .get("Proc-Type")
                .is_some_and(|v| v.contains("ENCRYPTED"))
            {
                return Err(OpenStackCliError::InputParameters(
                    "encrypted traditional PEM keys are not supported; \
                     convert first with: ssh-keygen -p -m OpenSSH -f <key>"
                        .into(),
                ));
            }
            RsaPrivateKey::from_pkcs1_der(document.contents()).map_err(|e| {
                OpenStackCliError::InputParameters(format!("failed to parse RSA PEM key: {e}"))
            })
        }
        "PRIVATE KEY" => RsaPrivateKey::from_pkcs8_der(document.contents()).map_err(|e| {
            OpenStackCliError::InputParameters(format!("failed to parse PKCS#8 key: {e}"))
        }),
        "ENCRYPTED PRIVATE KEY" => {
            let passphrase = passphrase_prompt()?;
            RsaPrivateKey::from_pkcs8_encrypted_der(document.contents(), passphrase.as_bytes())
                .map_err(|e| {
                    OpenStackCliError::InputParameters(format!(
                        "failed to decrypt PKCS#8 key (wrong passphrase?): {e}"
                    ))
                })
        }
        tag => Err(OpenStackCliError::InputParameters(format!(
            "unsupported PEM tag {tag:?} (expected OpenSSH, PEM RSA, or PKCS#8)"
        ))),
    }
}

/// Base64-decode and RSA PKCS#1 v1.5 decrypt a Nova server password.
fn decrypt_password(key_path: &Path, encrypted_b64: &str) -> Result<String, OpenStackCliError> {
    let rsa_key = load_rsa_key(key_path)?;

    let ciphertext = BASE64.decode(encrypted_b64)?;

    let plaintext_bytes = rsa_key.decrypt(Pkcs1v15Encrypt, &ciphertext).map_err(|_| {
        OpenStackCliError::InputParameters(
            "failed to decrypt password — is this the right key?".into(),
        )
    })?;

    String::from_utf8(plaintext_bytes).map_err(|_| {
        OpenStackCliError::InputParameters("decrypted password is not valid UTF-8".into())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    use tempfile::NamedTempFile;

    use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding as Pkcs1LineEnding};
    use rsa::pkcs8::{EncodePrivateKey, LineEnding as Pkcs8LineEnding};
    use rsa::{RsaPublicKey, rand_core::OsRng};

    fn make_test_keypair() -> (RsaPrivateKey, RsaPublicKey) {
        let private = RsaPrivateKey::new(&mut OsRng, 1024).expect("key generation");
        let public = RsaPublicKey::from(&private);
        (private, public)
    }

    fn nova_encrypt(public_key: &RsaPublicKey, plaintext: &str) -> String {
        let ct = public_key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, plaintext.as_bytes())
            .expect("encrypt");
        BASE64.encode(ct)
    }

    #[test]
    fn test_decrypt_pkcs1_pem() {
        let (priv_key, pub_key) = make_test_keypair();
        let encrypted_b64 = nova_encrypt(&pub_key, "s3cr3t");

        let pem = priv_key
            .to_pkcs1_pem(Pkcs1LineEnding::LF)
            .expect("pkcs1 pem");
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(pem.as_bytes()).expect("write");

        assert_eq!(
            decrypt_password(f.path(), &encrypted_b64).expect("decrypt"),
            "s3cr3t"
        );
    }

    #[test]
    fn test_decrypt_pkcs8_pem() {
        let (priv_key, pub_key) = make_test_keypair();
        let encrypted_b64 = nova_encrypt(&pub_key, "p@ssw0rd");

        let pem = priv_key
            .to_pkcs8_pem(Pkcs8LineEnding::LF)
            .expect("pkcs8 pem");
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(pem.as_bytes()).expect("write");

        assert_eq!(
            decrypt_password(f.path(), &encrypted_b64).expect("decrypt"),
            "p@ssw0rd"
        );
    }

    #[test]
    fn test_wrong_key_returns_error() {
        let (_, pub_key) = make_test_keypair();
        let encrypted_b64 = nova_encrypt(&pub_key, "secret");

        let (wrong_priv, _) = make_test_keypair();
        let pem = wrong_priv
            .to_pkcs1_pem(Pkcs1LineEnding::LF)
            .expect("pkcs1 pem");
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(pem.as_bytes()).expect("write");

        let err = decrypt_password(f.path(), &encrypted_b64).expect_err("should fail");
        assert!(
            matches!(err, OpenStackCliError::InputParameters(_)),
            "expected InputParameters, got {err:?}"
        );
    }

    #[test]
    fn test_invalid_base64_returns_error() {
        let (priv_key, _) = make_test_keypair();
        let pem = priv_key
            .to_pkcs1_pem(Pkcs1LineEnding::LF)
            .expect("pkcs1 pem");
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(pem.as_bytes()).expect("write");

        let err = decrypt_password(f.path(), "not!!valid!!base64").expect_err("should fail");
        assert!(
            matches!(err, OpenStackCliError::Base64Decode(_)),
            "expected Base64Decode, got {err:?}"
        );
    }

    #[test]
    fn test_decrypt_encrypted_pkcs8_pem() {
        let (priv_key, pub_key) = make_test_keypair();
        let encrypted_b64 = nova_encrypt(&pub_key, "s3cr3t");

        let pem = priv_key
            .to_pkcs8_encrypted_pem(&mut OsRng, b"letmein", Pkcs8LineEnding::LF)
            .expect("encrypted pkcs8 pem");

        let key = load_rsa_key_from_pem(&pem, &|| Ok("letmein".to_string())).expect("load key");
        let plaintext = key
            .decrypt(
                Pkcs1v15Encrypt,
                &BASE64.decode(&encrypted_b64).expect("base64"),
            )
            .expect("decrypt");
        assert_eq!(plaintext, b"s3cr3t");
    }

    #[test]
    fn test_encrypted_pkcs8_wrong_passphrase_returns_error() {
        let (priv_key, _) = make_test_keypair();
        let pem = priv_key
            .to_pkcs8_encrypted_pem(&mut OsRng, b"letmein", Pkcs8LineEnding::LF)
            .expect("encrypted pkcs8 pem");

        let err = load_rsa_key_from_pem(&pem, &|| Ok("wrong".to_string()))
            .expect_err("should fail with wrong passphrase");
        assert!(
            matches!(err, OpenStackCliError::InputParameters(_)),
            "expected InputParameters, got {err:?}"
        );
    }

    #[test]
    fn test_encrypted_traditional_pem_returns_helpful_error() {
        // Traditional encrypted PEM: the tag is still "RSA PRIVATE KEY";
        // encryption is signaled by RFC 1421 Proc-Type/DEK-Info headers.
        // The body is not a key, just valid base64.
        let content = concat!(
            "-----BEGIN RSA PRIVATE KEY-----\n", // gitleaks:allow
            "Proc-Type: 4,ENCRYPTED\n",
            "DEK-Info: AES-128-CBC,A1B2C3D4E5F60718293A4B5C6D7E8F90\n",
            "\n",
            "bm90IGEgcmVhbCBrZXkgYm9keSwganVzdCB2YWxpZCBiYXNlNjQ=\n",
            "-----END RSA PRIVATE KEY-----\n",
        );

        let err = load_rsa_key_from_pem(content, &no_passphrase).expect_err("should fail");
        match err {
            OpenStackCliError::InputParameters(msg) => {
                assert!(
                    msg.contains("ssh-keygen"),
                    "expected ssh-keygen hint in: {msg}"
                );
            }
            other => panic!("expected InputParameters, got {other:?}"),
        }
    }

    #[test]
    fn test_unsupported_pem_tag_returns_error() {
        let content = "-----BEGIN CERTIFICATE-----\n\
            bm90IGEga2V5\n\
            -----END CERTIFICATE-----\n";

        let err = load_rsa_key_from_pem(content, &no_passphrase).expect_err("should fail");
        match err {
            OpenStackCliError::InputParameters(msg) => {
                assert!(
                    msg.contains("CERTIFICATE"),
                    "expected unsupported tag to be named in: {msg}"
                );
            }
            other => panic!("expected InputParameters, got {other:?}"),
        }
    }

    fn no_passphrase() -> Result<String, OpenStackCliError> {
        panic!("passphrase prompt must not be invoked for this input");
    }

    #[test]
    fn test_unrecognized_format_returns_error() {
        let mut f = NamedTempFile::new().expect("tempfile");
        f.write_all(b"this is not a key\n").expect("write");

        let err = load_rsa_key(f.path()).expect_err("should fail");
        assert!(
            matches!(err, OpenStackCliError::InputParameters(_)),
            "expected InputParameters, got {err:?}"
        );
    }

    #[test]
    fn test_missing_file_returns_io_error() {
        let err = load_rsa_key(Path::new("/nonexistent/key.pem")).expect_err("should fail");
        assert!(
            matches!(err, OpenStackCliError::IO { .. }),
            "expected IO error, got {err:?}"
        );
    }
}

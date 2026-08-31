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

//! Verification of GitHub artifact attestations (Sigstore bundles produced
//! by `actions/attest`) proving a registry-listed plugin artifact was
//! actually published by CI in its claimed source repository, rather than
//! just checksum-consistent with what the registry index says.
//!
//! This is a deliberately reduced verifier, not a general Sigstore client:
//! it checks the DSSE envelope's signature against the attestation's leaf
//! (Fulcio-issued) certificate, that the leaf certificate chains to a
//! vendored, pinned Fulcio root/intermediate CA (`../trust/*.pem`), and that
//! the leaf certificate's GitHub Actions identity extensions (OIDC issuer +
//! `owner/repo`) match the plugin's claimed source repository. It does
//! **not** verify the Rekor transparency-log inclusion proof (the Merkle
//! audit path + signed tree head) — that needs a TUF trust-root client and
//! Merkle-proof code that only the `sigstore` crate provides today, and that
//! crate's dependency graph (a TUF client, a possible OpenSSL pull-through)
//! conflicts with this workspace's explicit no-OpenSSL stance. This is the
//! same tradeoff `cosign verify-blob --insecure-ignore-tlog` documents,
//! chosen deliberately here rather than silently: a tampered or unattested
//! plugin still fails closed, but a withheld-from-the-public-log attestation
//! is not detected.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::Utc;
use ring::signature::{self, UnparsedPublicKey};
use serde::Deserialize;
use x509_parser::certificate::X509Certificate;
use x509_parser::extensions::GeneralName;
use x509_parser::pem::Pem;
use x509_parser::prelude::FromDer;

use crate::error::WasmPluginError;
use crate::lockfile::ProvenanceRecord;

/// The GitHub REST API host attestations are fetched from by default.
pub const GITHUB_API_BASE_URL: &str = "https://api.github.com";

/// Fulcio's "OIDC issuer" certificate extension OID.
const OID_OIDC_ISSUER: &str = "1.3.6.1.4.1.57264.1.1";
/// Fulcio's "source repository" (`owner/repo`) certificate extension OID.
const OID_SOURCE_REPO: &str = "1.3.6.1.4.1.57264.1.5";
/// The only OIDC issuer a GitHub Actions-signed certificate can have.
const EXPECTED_OIDC_ISSUER: &str = "https://token.actions.githubusercontent.com";

const FULCIO_ROOT_PEM: &str = include_str!("../trust/fulcio_root.pem");
const FULCIO_INTERMEDIATE_PEM: &str = include_str!("../trust/fulcio_intermediate.pem");

/// The pinned Fulcio root + intermediate CA certificates a leaf certificate
/// must chain to before its signature is trusted.
pub struct TrustRoots {
    root_der: Vec<u8>,
    intermediate_der: Vec<u8>,
}

impl TrustRoots {
    /// The vendored, pinned Sigstore public-good-instance Fulcio root and
    /// intermediate CAs (`sdk/plugin-wasm/trust/*.pem`).
    pub fn production() -> Result<Self, WasmPluginError> {
        Self::from_pem(
            FULCIO_ROOT_PEM.as_bytes(),
            FULCIO_INTERMEDIATE_PEM.as_bytes(),
        )
    }

    /// Build from arbitrary PEM bytes. Exposed so tests can substitute a
    /// synthetic root/intermediate pair, making the certificate-chain and
    /// DSSE-signature verification path exercisable without a real
    /// GitHub-signed artifact.
    pub fn from_pem(root_pem: &[u8], intermediate_pem: &[u8]) -> Result<Self, WasmPluginError> {
        Ok(Self {
            root_der: pem_to_der(root_pem)?,
            intermediate_der: pem_to_der(intermediate_pem)?,
        })
    }

    fn root(&self) -> Result<X509Certificate<'_>, WasmPluginError> {
        parse_cert(&self.root_der, "vendored Fulcio root")
    }

    fn intermediate(&self) -> Result<X509Certificate<'_>, WasmPluginError> {
        parse_cert(&self.intermediate_der, "vendored Fulcio intermediate")
    }
}

fn pem_to_der(pem_bytes: &[u8]) -> Result<Vec<u8>, WasmPluginError> {
    let pem = Pem::iter_from_buffer(pem_bytes)
        .next()
        .ok_or_else(|| WasmPluginError::AttestationVerification {
            reason: "no PEM block found in trust anchor".into(),
        })?
        .map_err(|e| WasmPluginError::AttestationVerification {
            reason: format!("invalid trust anchor PEM: {e}"),
        })?;
    Ok(pem.contents)
}

fn parse_cert<'a>(der: &'a [u8], what: &str) -> Result<X509Certificate<'a>, WasmPluginError> {
    let (_, cert) =
        X509Certificate::from_der(der).map_err(|e| WasmPluginError::AttestationVerification {
            reason: format!("invalid {what} certificate: {e}"),
        })?;
    Ok(cert)
}

/// The response body of `GET /repos/{owner}/{repo}/attestations/{subject_digest}`.
#[derive(Clone, Debug, Deserialize)]
pub struct AttestationsResponse {
    /// Attestations found for the given subject digest.
    #[serde(default)]
    pub attestations: Vec<Attestation>,
}

/// A single attestation entry.
#[derive(Clone, Debug, Deserialize)]
pub struct Attestation {
    /// The Sigstore bundle itself.
    pub bundle: Bundle,
}

/// A Sigstore bundle: a signed statement plus the material needed to verify
/// it. Only the fields this verifier uses are modeled; unknown fields
/// (`mediaType`, `bundle_url`, ...) are ignored by `serde` by default.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The signed DSSE envelope.
    pub dsse_envelope: DsseEnvelope,
    /// The certificate(s) and transparency-log entries backing the
    /// signature.
    pub verification_material: VerificationMaterial,
}

/// A DSSE (Dead Simple Signing Envelope) as used by in-toto attestations.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DsseEnvelope {
    /// Base64-encoded payload (an in-toto statement, not itself parsed by
    /// this verifier — only its signature is checked).
    pub payload: String,
    /// The payload's media type, part of the signed pre-authentication
    /// encoding.
    pub payload_type: String,
    /// Signatures over the payload. Only the first is checked.
    #[serde(default)]
    pub signatures: Vec<DsseSignature>,
}

/// A single DSSE signature.
#[derive(Clone, Debug, Deserialize)]
pub struct DsseSignature {
    /// Base64-encoded signature bytes (ASN.1 DER ECDSA signature).
    pub sig: String,
}

/// The certificate(s) and transparency-log entries a bundle carries.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMaterial {
    /// A single leaf certificate, in newer bundle media types.
    #[serde(default)]
    pub certificate: Option<RawCert>,
    /// A certificate chain, in older bundle media types. When present and
    /// `certificate` is absent, its first entry is the leaf.
    #[serde(default)]
    pub x509_certificate_chain: Option<CertChain>,
    /// Rekor transparency-log entries, if any. **Not** verified for Merkle
    /// inclusion by this verifier — see module docs.
    #[serde(default)]
    pub tlog_entries: Vec<TlogEntry>,
}

impl VerificationMaterial {
    fn leaf_certificate_der(&self) -> Result<Vec<u8>, WasmPluginError> {
        let raw_bytes = self
            .certificate
            .as_ref()
            .map(|c| c.raw_bytes.as_str())
            .or_else(|| {
                self.x509_certificate_chain
                    .as_ref()
                    .and_then(|c| c.certificates.first())
                    .map(|c| c.raw_bytes.as_str())
            })
            .ok_or_else(|| WasmPluginError::AttestationVerification {
                reason: "attestation bundle has no leaf certificate".into(),
            })?;
        BASE64
            .decode(raw_bytes)
            .map_err(|e| WasmPluginError::AttestationVerification {
                reason: format!("invalid leaf certificate base64: {e}"),
            })
    }
}

/// A single DER certificate, base64-encoded.
#[derive(Clone, Debug, Deserialize)]
pub struct RawCert {
    /// Base64-encoded DER certificate bytes.
    #[serde(rename = "rawBytes")]
    pub raw_bytes: String,
}

/// A chain of DER certificates.
#[derive(Clone, Debug, Deserialize)]
pub struct CertChain {
    /// Leaf-first certificate chain.
    #[serde(default)]
    pub certificates: Vec<RawCert>,
}

/// A single Rekor transparency-log entry reference. Only kept for display;
/// not verified.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlogEntry {
    /// The log index, if present. GitHub's API has represented this as both
    /// a JSON string and a JSON number across bundle media types, so it's
    /// captured generically and stringified for display.
    #[serde(default)]
    pub log_index: Option<serde_json::Value>,
}

/// Fetch the GitHub artifact attestations for the artifact whose sha256
/// digest is `sha256_hex`, from `{base_url}/repos/{owner}/{repo}/attestations/...`.
///
/// `base_url` is a parameter (rather than hardcoded) so tests can point this
/// at an `httpmock` server; production callers should pass
/// [`GITHUB_API_BASE_URL`]. An optional `GITHUB_TOKEN` environment variable
/// is sent as a bearer token if set — purely a rate-limit/private-repo
/// convenience, never required and never a factor in what ends up trusted.
pub async fn fetch_attestations(
    base_url: &str,
    owner: &str,
    repo: &str,
    sha256_hex: &str,
    client: &reqwest::Client,
) -> Result<AttestationsResponse, WasmPluginError> {
    let url = format!("{base_url}/repos/{owner}/{repo}/attestations/sha256:{sha256_hex}");
    let mut request = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");
    if let Ok(token) = std::env::var("GITHUB_TOKEN")
        && !token.is_empty()
    {
        request = request.bearer_auth(token);
    }
    let to_fetch_err = |source| WasmPluginError::AttestationFetch {
        owner: owner.to_string(),
        repo: repo.to_string(),
        source,
    };
    let response = request
        .send()
        .await
        .map_err(to_fetch_err)?
        .error_for_status()
        .map_err(to_fetch_err)?;
    let bytes = response.bytes().await.map_err(to_fetch_err)?;
    serde_json::from_slice(&bytes).map_err(|e| WasmPluginError::AttestationVerification {
        reason: format!("malformed attestations response: {e}"),
    })
}

/// Fetch and verify GitHub artifact attestations for `sha256_hex`, claimed
/// to have been published from `source_repo` (`owner/repo`), against the
/// production trust roots. Returns the first attestation that verifies
/// successfully; if none do, returns the last verification error
/// encountered (or a "no attestations found" error if none exist at all).
pub async fn verify_for_source(
    source_repo: &str,
    sha256_hex: &str,
    client: &reqwest::Client,
) -> Result<ProvenanceRecord, WasmPluginError> {
    let (owner, repo) =
        source_repo
            .split_once('/')
            .ok_or_else(|| WasmPluginError::AttestationVerification {
                reason: format!(
                    "registry entry source_repo `{source_repo}` is not in `owner/repo` form"
                ),
            })?;
    let response = fetch_attestations(GITHUB_API_BASE_URL, owner, repo, sha256_hex, client).await?;
    let roots = TrustRoots::production()?;

    let mut last_err = WasmPluginError::AttestationVerification {
        reason: "no attestations found for this artifact".into(),
    };
    for attestation in &response.attestations {
        match verify_attestation(&attestation.bundle, owner, repo, &roots) {
            Ok(record) => return Ok(record),
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

/// Verify a single attestation bundle's DSSE signature, certificate chain,
/// and GitHub Actions identity against `expected_owner`/`expected_repo`. See
/// module docs for exactly what is and isn't checked.
pub fn verify_attestation(
    bundle: &Bundle,
    expected_owner: &str,
    expected_repo: &str,
    roots: &TrustRoots,
) -> Result<ProvenanceRecord, WasmPluginError> {
    let payload = BASE64
        .decode(bundle.dsse_envelope.payload.as_bytes())
        .map_err(|e| WasmPluginError::AttestationVerification {
            reason: format!("invalid DSSE payload base64: {e}"),
        })?;
    let signature = bundle.dsse_envelope.signatures.first().ok_or_else(|| {
        WasmPluginError::AttestationVerification {
            reason: "DSSE envelope has no signatures".into(),
        }
    })?;
    let sig_bytes = BASE64.decode(signature.sig.as_bytes()).map_err(|e| {
        WasmPluginError::AttestationVerification {
            reason: format!("invalid DSSE signature base64: {e}"),
        }
    })?;
    let pae = dsse_pae(&bundle.dsse_envelope.payload_type, &payload);

    let leaf_der = bundle.verification_material.leaf_certificate_der()?;
    let leaf = parse_cert(&leaf_der, "attestation leaf")?;

    let leaf_key = UnparsedPublicKey::new(
        &signature::ECDSA_P256_SHA256_ASN1,
        leaf.public_key().subject_public_key.data.as_ref(),
    );
    leaf_key
        .verify(&pae, &sig_bytes)
        .map_err(|_| WasmPluginError::AttestationVerification {
            reason: "DSSE envelope signature verification failed".into(),
        })?;

    // Chain the leaf to our own vendored, pinned root/intermediate —
    // deliberately ignoring any intermediate/root certificates the bundle
    // itself supplied, since trusting the server's own chain would defeat
    // the point of pinning.
    let intermediate = roots.intermediate()?;
    let root = roots.root()?;
    leaf.verify_signature(Some(intermediate.public_key()))
        .map_err(|e| WasmPluginError::AttestationVerification {
            reason: format!(
                "leaf certificate does not chain to the pinned Fulcio intermediate: {e}"
            ),
        })?;
    intermediate
        .verify_signature(Some(root.public_key()))
        .map_err(|e| WasmPluginError::AttestationVerification {
            reason: format!(
                "pinned Fulcio intermediate does not chain to the pinned Fulcio root: {e}"
            ),
        })?;

    let oidc_issuer = extension_string(&leaf, OID_OIDC_ISSUER);
    if oidc_issuer.as_deref() != Some(EXPECTED_OIDC_ISSUER) {
        return Err(WasmPluginError::AttestationVerification {
            reason: format!("unexpected OIDC issuer: {oidc_issuer:?}"),
        });
    }
    let expected_repository = format!("{expected_owner}/{expected_repo}");
    let repository = extension_string(&leaf, OID_SOURCE_REPO);
    if repository.as_deref() != Some(expected_repository.as_str()) {
        return Err(WasmPluginError::AttestationVerification {
            reason: format!(
                "certificate identity `{repository:?}` does not match claimed source repo `{expected_repository}`"
            ),
        });
    }

    let workflow_ref = subject_alternative_name_uri(&leaf);
    let rekor_log_index = bundle
        .verification_material
        .tlog_entries
        .first()
        .and_then(|e| e.log_index.as_ref())
        .map(value_to_display_string);

    Ok(ProvenanceRecord {
        source_repo: expected_repository,
        workflow_ref,
        oidc_issuer,
        rekor_log_index,
        verified_at: Utc::now(),
    })
}

fn value_to_display_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// The DSSE Pre-Authentication Encoding: the exact byte structure that gets
/// signed. See
/// <https://github.com/secure-systems-lab/dsse/blob/master/protocol.md>.
fn dsse_pae(payload_type: &str, payload: &[u8]) -> Vec<u8> {
    let mut pae = Vec::with_capacity(payload.len() + payload_type.len() + 32);
    pae.extend_from_slice(b"DSSEv1");
    pae.push(b' ');
    pae.extend_from_slice(payload_type.len().to_string().as_bytes());
    pae.push(b' ');
    pae.extend_from_slice(payload_type.as_bytes());
    pae.push(b' ');
    pae.extend_from_slice(payload.len().to_string().as_bytes());
    pae.push(b' ');
    pae.extend_from_slice(payload);
    pae
}

/// Best-effort extraction of a Fulcio identity extension's string value.
/// These extensions hold their content as a primitive ASN.1 string TLV
/// (tag + DER length + UTF-8 bytes); rather than pull in a generic ASN.1
/// value parser for this one shape, the two-or-more byte header is skipped
/// directly.
fn extension_string(cert: &X509Certificate, oid: &str) -> Option<String> {
    let ext = cert
        .extensions()
        .iter()
        .find(|e| e.oid.to_id_string() == oid)?;
    decode_der_string_content(ext.value)
}

fn decode_der_string_content(value: &[u8]) -> Option<String> {
    let (_tag, rest) = value.split_first()?;
    let (len_byte, rest) = rest.split_first()?;
    let content = if *len_byte < 0x80 {
        rest.get(..*len_byte as usize)?
    } else {
        let n = (*len_byte & 0x7f) as usize;
        let len_bytes = rest.get(..n)?;
        let len = len_bytes
            .iter()
            .fold(0usize, |acc, b| (acc << 8) | *b as usize);
        rest.get(n..n + len)?
    };
    std::str::from_utf8(content).ok().map(|s| s.to_string())
}

fn subject_alternative_name_uri(cert: &X509Certificate) -> Option<String> {
    let san = cert.subject_alternative_name().ok().flatten()?;
    san.value.general_names.iter().find_map(|gn| match gn {
        GeneralName::URI(uri) => Some((*uri).to_string()),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsse_pae_matches_spec_example() {
        // From the DSSE spec's own worked example: PAE = "DSSEv1" SP
        // LEN(type) SP type SP LEN(body) SP body.
        let payload_type = "http://example.com/HelloWorld";
        let payload = b"hello world";
        let pae = dsse_pae(payload_type, payload);
        let expected = format!(
            "DSSEv1 {} {payload_type} {} {}",
            payload_type.len(),
            payload.len(),
            "hello world"
        );
        assert_eq!(pae, expected.into_bytes());
    }

    #[test]
    fn decode_der_string_content_strips_short_form_header() {
        // UTF8String (tag 0x0c), length 5, "hello"
        let der = [0x0c, 0x05, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(decode_der_string_content(&der).as_deref(), Some("hello"));
    }

    /// A synthetic root -> intermediate -> leaf certificate chain, shaped
    /// like what Fulcio issues (leaf carrying a SAN workflow-ref URI and the
    /// two GitHub Actions identity extension OIDs), built with `rcgen` so the
    /// DSSE-signature and certificate-chain verification paths in
    /// [`verify_attestation`] are exercisable without a real GitHub-signed
    /// artifact. Not related to (and not a stand-in for) the real,
    /// vendored Fulcio roots in [`TrustRoots::production`].
    struct SyntheticChain {
        root_pem: String,
        intermediate_pem: String,
        leaf_der: Vec<u8>,
        leaf_signing_key: ring::signature::EcdsaKeyPair,
    }

    fn der_utf8_string(s: &str) -> Vec<u8> {
        assert!(s.len() < 128, "test helper only supports short strings");
        let mut v = vec![0x0c, s.len() as u8];
        v.extend_from_slice(s.as_bytes());
        v
    }

    fn build_synthetic_chain(
        oidc_issuer: &str,
        source_repo: &str,
        workflow_ref: &str,
    ) -> SyntheticChain {
        use rcgen::string::Ia5String;
        use rcgen::{
            BasicConstraints, CertificateParams, CustomExtension, IsCa, Issuer, KeyPair, SanType,
        };

        let mut root_params =
            CertificateParams::new(Vec::<String>::new()).expect("empty SAN list is always valid");
        root_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        let root_key = KeyPair::generate().expect("key generation");
        let root_cert = root_params.self_signed(&root_key).expect("self-sign root");
        let root_issuer = Issuer::from_params(&root_params, &root_key);

        let mut inter_params =
            CertificateParams::new(Vec::<String>::new()).expect("empty SAN list is always valid");
        inter_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        let inter_key = KeyPair::generate().expect("key generation");
        let inter_cert = inter_params
            .signed_by(&inter_key, &root_issuer)
            .expect("sign intermediate with root");
        let inter_issuer = Issuer::from_params(&inter_params, &inter_key);

        let mut leaf_params =
            CertificateParams::new(Vec::<String>::new()).expect("empty SAN list is always valid");
        leaf_params.is_ca = IsCa::NoCa;
        leaf_params.subject_alt_names = vec![SanType::URI(
            Ia5String::try_from(workflow_ref).expect("valid IA5 URI"),
        )];
        leaf_params
            .custom_extensions
            .push(CustomExtension::from_oid_content(
                &[1, 3, 6, 1, 4, 1, 57264, 1, 1],
                der_utf8_string(oidc_issuer),
            ));
        leaf_params
            .custom_extensions
            .push(CustomExtension::from_oid_content(
                &[1, 3, 6, 1, 4, 1, 57264, 1, 5],
                der_utf8_string(source_repo),
            ));
        let leaf_key = KeyPair::generate().expect("key generation");
        let leaf_cert = leaf_params
            .signed_by(&leaf_key, &inter_issuer)
            .expect("sign leaf with intermediate");

        let leaf_signing_key = ring::signature::EcdsaKeyPair::from_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
            leaf_key.serialize_der().as_slice(),
            &ring::rand::SystemRandom::new(),
        )
        .expect("rcgen's PKCS#8 output loads into ring");

        SyntheticChain {
            root_pem: root_cert.pem(),
            intermediate_pem: inter_cert.pem(),
            leaf_der: leaf_cert.der().to_vec(),
            leaf_signing_key,
        }
    }

    const TEST_PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";

    fn build_bundle(chain: &SyntheticChain, payload: &[u8]) -> Bundle {
        let pae = dsse_pae(TEST_PAYLOAD_TYPE, payload);
        let sig = chain
            .leaf_signing_key
            .sign(&ring::rand::SystemRandom::new(), &pae)
            .expect("sign PAE");
        Bundle {
            dsse_envelope: DsseEnvelope {
                payload: BASE64.encode(payload),
                payload_type: TEST_PAYLOAD_TYPE.to_string(),
                signatures: vec![DsseSignature {
                    sig: BASE64.encode(sig.as_ref()),
                }],
            },
            verification_material: VerificationMaterial {
                certificate: Some(RawCert {
                    raw_bytes: BASE64.encode(&chain.leaf_der),
                }),
                x509_certificate_chain: None,
                tlog_entries: vec![],
            },
        }
    }

    const TEST_WORKFLOW_REF: &str = "https://github.com/gtema/example-auth-plugin/.github/workflows/release.yml@refs/heads/main";

    #[test]
    fn verify_attestation_accepts_valid_synthetic_chain() {
        let chain = build_synthetic_chain(
            EXPECTED_OIDC_ISSUER,
            "gtema/example-auth-plugin",
            TEST_WORKFLOW_REF,
        );
        let bundle = build_bundle(&chain, br#"{"_type":"https://in-toto.io/Statement/v1"}"#);
        let roots =
            TrustRoots::from_pem(chain.root_pem.as_bytes(), chain.intermediate_pem.as_bytes())
                .expect("trust roots parse");

        let record = verify_attestation(&bundle, "gtema", "example-auth-plugin", &roots)
            .expect("valid attestation should verify");
        assert_eq!(record.source_repo, "gtema/example-auth-plugin");
        assert_eq!(record.oidc_issuer.as_deref(), Some(EXPECTED_OIDC_ISSUER));
        assert_eq!(record.workflow_ref.as_deref(), Some(TEST_WORKFLOW_REF));
    }

    #[test]
    fn verify_attestation_rejects_tampered_payload() {
        let chain = build_synthetic_chain(
            EXPECTED_OIDC_ISSUER,
            "gtema/example-auth-plugin",
            TEST_WORKFLOW_REF,
        );
        let mut bundle = build_bundle(&chain, br#"{"_type":"https://in-toto.io/Statement/v1"}"#);
        // Swap in a different (still validly base64/JSON-shaped) payload
        // after signing, without re-signing — this is what a tampered or
        // substituted artifact attestation looks like on the wire.
        bundle.dsse_envelope.payload = BASE64.encode(b"{\"_type\":\"tampered\"}");
        let roots =
            TrustRoots::from_pem(chain.root_pem.as_bytes(), chain.intermediate_pem.as_bytes())
                .expect("trust roots parse");

        assert!(verify_attestation(&bundle, "gtema", "example-auth-plugin", &roots).is_err());
    }

    #[test]
    fn verify_attestation_rejects_repository_mismatch() {
        let chain = build_synthetic_chain(
            EXPECTED_OIDC_ISSUER,
            "gtema/example-auth-plugin",
            TEST_WORKFLOW_REF,
        );
        let bundle = build_bundle(&chain, br#"{"_type":"https://in-toto.io/Statement/v1"}"#);
        let roots =
            TrustRoots::from_pem(chain.root_pem.as_bytes(), chain.intermediate_pem.as_bytes())
                .expect("trust roots parse");

        // The certificate's repository extension says gtema/example-auth-plugin;
        // the caller expects a different repository entirely (as would happen
        // if a registry index entry's source_repo were spoofed/mismatched).
        assert!(verify_attestation(&bundle, "someone-else", "unrelated-repo", &roots).is_err());
    }

    #[test]
    fn verify_attestation_rejects_chain_not_rooted_in_pinned_ca() {
        let chain = build_synthetic_chain(
            EXPECTED_OIDC_ISSUER,
            "gtema/example-auth-plugin",
            TEST_WORKFLOW_REF,
        );
        let bundle = build_bundle(&chain, br#"{"_type":"https://in-toto.io/Statement/v1"}"#);
        // A second, unrelated synthetic root/intermediate pair the leaf was
        // never actually signed by — simulates an attacker presenting their
        // own CA instead of the pinned Fulcio one.
        let other = build_synthetic_chain(
            EXPECTED_OIDC_ISSUER,
            "gtema/example-auth-plugin",
            TEST_WORKFLOW_REF,
        );
        let roots =
            TrustRoots::from_pem(other.root_pem.as_bytes(), other.intermediate_pem.as_bytes())
                .expect("trust roots parse");

        assert!(verify_attestation(&bundle, "gtema", "example-auth-plugin", &roots).is_err());
    }
}

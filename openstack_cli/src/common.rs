//! Common helpers
use crate::error::OpenStackCliError;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::IsTerminal;

use serde_json::Value;

use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio::fs;
use tokio::io::{self};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use tokio_util::io::InspectReader;

use openstack_sdk::types::BoxedAsyncRead;

/// Newtype for the `Vec<String>`
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(pub Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}

/// Newtype for the `Vec<Value>`
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecValue(pub Vec<Value>);
impl fmt::Display for VecValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| serde_json::to_string(v).unwrap_or("!SERIALIZE_ERROR!".to_string()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl From<Vec<Value>> for VecValue {
    fn from(item: Vec<Value>) -> Self {
        VecValue(item)
    }
}
impl From<&Vec<Value>> for VecValue {
    fn from(item: &Vec<Value>) -> Self {
        VecValue(item.clone())
    }
}

/// Newtype for the `HashMap<String, String>`
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringString(HashMap<String, String>);

impl HashMapStringString {
    pub fn new() -> Self {
        HashMapStringString(HashMap::new())
    }
}

impl From<HashMap<String, String>> for HashMapStringString {
    fn from(data: HashMap<String, String>) -> Self {
        HashMapStringString(data.clone())
    }
}

// And here's the display logic.
impl fmt::Display for HashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

/// Newtype for the `Option<HashMap<String, String>>`
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct OptionHashMapStringString(Option<HashMap<String, String>>);
impl fmt::Display for OptionHashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref data) => write!(
                f,
                "{}",
                data.iter()
                    .map(|v| format!("{}={}", v.0, v.1))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            None => write!(f, ""),
        }
    }
}

/// Newtype for the `Option<Vector<HashMap<String, String>>>`
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct OptionVecHashMapStringString(Option<Vec<HashMap<String, String>>>);
impl fmt::Display for OptionVecHashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref data) => write!(
                f,
                "{}",
                data.iter()
                    .map(|v| v
                        .iter()
                        .map(|d| format!("{}={}", d.0, d.1))
                        .collect::<Vec<String>>()
                        .join(","))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            None => write!(f, ""),
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecHashMapStringString(Vec<HashMap<String, String>>);
impl fmt::Display for VecHashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v
                    .iter()
                    .map(|d| format!("{}={}", d.0, d.1))
                    .collect::<Vec<String>>()
                    .join(","))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

/// NumString (Number or Number as string)
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct NumString(u64);
impl fmt::Display for NumString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'de> Deserialize<'de> for NumString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = NumString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumString(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Ok(NumString(0)),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

/// Try to deserialize data and return `Default` if that fails
pub fn deser_ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

/// Parse a single key-value pair
pub(crate) fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

pub(crate) fn parse_json(s: &str) -> Result<Value, Box<dyn Error + Send + Sync + 'static>>
where
{
    Ok(serde_json::from_str(s)?)
}

/// Download content from the reqwests response stream.
/// When dst_name = "-" - write content to the stdout.
/// Otherwise write into the destination and display progress_bar
pub(crate) async fn download_file(
    dst_name: String,
    size: u64,
    data: BoxedAsyncRead,
) -> Result<(), OpenStackCliError> {
    let progress_bar = ProgressBar::new(size);

    let mut inspect_reader =
        InspectReader::new(data.compat(), |bytes| progress_bar.inc(bytes.len() as u64));
    if dst_name == "-" {
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .progress_chars("#>-")
                .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} at {bytes_per_sec}")?,
        );

        let mut writer = io::stdout();
        io::copy(&mut inspect_reader, &mut writer).await?;
    } else {
        let path = Path::new(&dst_name);
        let fname = path.file_name().unwrap().to_str().unwrap();
        progress_bar.set_message(String::from(fname));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .progress_chars("#>-")
                .template(
                    "[{bar:40.cyan/blue}] {bytes}/{total_bytes} at {bytes_per_sec} - {msg}",
                )?,
        );

        let mut writer = fs::File::create(path).await?;
        io::copy(&mut inspect_reader, &mut writer).await?;
    }
    progress_bar.finish();
    Ok(())
}

/// Construct BoxedAsyncRead with progress bar from stdin
async fn build_upload_asyncread_from_stdin() -> Result<BoxedAsyncRead, OpenStackCliError> {
    let progress_bar = ProgressBar::new(0);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .progress_chars("#>-")
            .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} at {bytes_per_sec}")?,
    );

    let inspect_reader = InspectReader::new(io::stdin(), move |bytes| {
        progress_bar.inc(bytes.len() as u64)
    });
    Ok(BoxedAsyncRead::new(inspect_reader.compat()))
}

/// Construct BoxedAsyncRead with progress bar from the file
async fn build_upload_asyncread_from_file(
    file_path: &str,
) -> Result<BoxedAsyncRead, OpenStackCliError> {
    let progress_bar = ProgressBar::new(0);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .progress_chars("#>-")
            .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} at {bytes_per_sec}")?,
    );
    let reader = fs::File::open(&file_path).await?;

    progress_bar.set_length(reader.metadata().await?.len());
    let inspect_reader =
        InspectReader::new(reader, move |bytes| progress_bar.inc(bytes.len() as u64));

    Ok(BoxedAsyncRead::new(inspect_reader.compat()))
}

/// Wrap file or stdout for being uploaded with reqwests library.
/// When dst_name = "-" - write content to the stdout.
/// Otherwise write into the destination and display progress_bar
pub(crate) async fn build_upload_asyncread(
    src_name: Option<String>,
) -> Result<BoxedAsyncRead, OpenStackCliError> {
    if !std::io::stdin().is_terminal() || src_name.is_none() {
        // Reading from stdin
        build_upload_asyncread_from_stdin().await
    } else {
        match src_name.unwrap().as_str() {
            "-" => build_upload_asyncread_from_stdin().await,
            file_name => build_upload_asyncread_from_file(file_name).await,
        }
    }
}

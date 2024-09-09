pub mod instances;
pub mod server;

pub use instances::Instances;
pub use server::Server;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Canceling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    Error = 112,
    Ready = 113,
    Success = 200,
    Failure = 400,
    Canceled = 401,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Type {
    /// standard synchronous operation
    Sync,
    /// background asynchronous operation
    Async,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    status: Status,
    status_code: u16,
    r#type: Type,
    operation: String,
    error: String,
    error_code: u16,
    // #[serde(flatten)]
    // extra: std::collections::HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod test_utils {
    use reqwest::{tls::Version, Client, ClientBuilder, Identity};
    use std::{env, fs::File, io::Read, sync::LazyLock};

    pub static INCUS_IP: &str = env!("INCUS_IP");
    pub static INCUS_PORT: u32 = 8443;
    pub static ID: LazyLock<Identity> = LazyLock::new(|| {
        let config_dir = dirs::config_local_dir().unwrap().join("incus");
        // let cert_path = config_dir.join("ca-cert.pem");
        let cert_path = config_dir.join("client.crt");
        // let key_path = config_dir.join("ca-key.pem");
        let key_path = config_dir.join("client.key");
        // Create an application.
        let mut buf = Vec::new();
        File::open(key_path).unwrap().read_to_end(&mut buf).unwrap();
        File::open(cert_path)
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        Identity::from_pem(&buf).unwrap()

    });

    pub fn new_client() -> Client {
        ClientBuilder::new()
            .use_rustls_tls()
            .min_tls_version(Version::TLS_1_3)
            // client certificate already registered by incus server
            // .add_root_certificate(reqwest::Certificate::from_pem(&buf).unwrap())
            .danger_accept_invalid_certs(true)
            .tls_info(true)
            .identity(ID.clone())
            .build()
            .unwrap()
    }
    pub fn incus_path(path: &str) -> String {
        format!("https://{}:{}/1.0{}", INCUS_IP, INCUS_PORT, path)
    }
}

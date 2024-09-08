use anvil_api::server::Server;
use reqwest::{tls::Version, ClientBuilder, Identity};
use std::{
    env,
    fs::File,
    io::{Read, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    let incus_ip = env::var("INCUS_IP").unwrap_or("localhost".to_string());
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
    let id = Identity::from_pem(&buf).unwrap();
    let client = ClientBuilder::new()
        .use_rustls_tls()
        .min_tls_version(Version::TLS_1_3)
        // client certificate already registered by incus server
        // .add_root_certificate(reqwest::Certificate::from_pem(&buf).unwrap())
        .danger_accept_invalid_certs(true)
        .tls_info(true)
        .identity(id)
        .build()
        .unwrap();
    let r = client
        // .get(format!("https://{}:8443/1.0/instances", incus_ip))
        .get(format!("https://{}:8443/1.0", incus_ip))
        //.form(&[("project", "default")])
        .send()
        .await
        .unwrap()
        .json::<Server>()
        .await
        .unwrap();
    println!("{:#?}", r);

    Ok(())
}

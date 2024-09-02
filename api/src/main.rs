use anvil_api::Base;
use reqwest::{tls::Version, ClientBuilder, Identity};
use std::{
    env,
    fs::File,
    io::{Read, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    let incus_ip = env::var("INCUS_IP").unwrap_or("localhost".to_string());
    let config_dir = dirs::config_local_dir().unwrap().join("anvil");
    let cert_path = config_dir.join("ca-cert.pem");
    let key_path = config_dir.join("ca-key.pem");
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
        // .add_root_certificate(Certificate::from_pem(&buf).unwrap())
        .danger_accept_invalid_certs(true)
        .tls_info(true)
        .identity(id)
        .build()
        .unwrap();
    let r = client
        .get(format!("https://{}:8443/", incus_ip))
        .send()
        .await
        .unwrap()
        .json::<Base>()
        .await
        .unwrap();
    println!("{:?}", r);

    Ok(())
}

use std::{env, error, fs::File, io::Read};

use reqwest::{tls::Version, Certificate, ClientBuilder, Identity};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub client: reqwest::Client,
    addr: String,
}

impl Default for App {
    fn default() -> Self {
        let addr = env::var("INCUS_IP").unwrap_or("localhost".to_owned());

        let config_dir = dirs::config_local_dir().unwrap().join("anvil");
        let cert_path = config_dir.join("ca-cert.pem");
        let key_path = config_dir.join("ca-key.pem");
        // let url = "https://localhost".parse::<hyper::Uri>().unwrap();
        //
        let mut pem_buf = Vec::new();
        File::open(key_path)
            .unwrap()
            .read_to_end(&mut pem_buf)
            .unwrap();
        File::open(cert_path)
            .unwrap()
            .read_to_end(&mut pem_buf)
            .unwrap();
        let id = Identity::from_pem(&pem_buf).unwrap();
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
        Self {
            running: true,
            counter: 0,
            client,
            addr,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
        // let url = "https://localhost".parse::<hyper::Uri>().unwrap();

        // Get the host and the port

        let r = self
            .client
            .get(format!("https://{}:8443/", self.addr))
            .send()
            .await
            .unwrap();
        println!("{:?}", r)
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

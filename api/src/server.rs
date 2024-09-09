use std::net::SocketAddr;

use crate::Base;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(flatten)]
    base: Base,
    metadata: Option<ServerMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMetadata {
    ///
    /// "api_extensions": [
    ///       "etag",
    ///       "patch",
    ///       "network",
    ///       "storage"
    ///     ],
    api_extensions: Vec<String>,
    //     "api_status": "stable",
    api_status: String,
    //     "api_version": "1.0",
    api_version: String,
    //     "auth": "untrusted",
    auth: String,
    //     "auth_methods": [
    //       "tls"
    //     ],
    auth_methods: Vec<String>,
    //     "auth_user_method": "unix",
    auth_user_method: String,
    //     "auth_user_name": "uid=201105",
    auth_user_name: String,
    //     "config": {
    //       "core.https_address": ":8443"
    //     },
    config: Value,
    environment: Environment,
    //     "public": false
    public: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Environment {
    //       "addresses": [
    //         ":8443"
    //       ],
    addresses: Vec<SocketAddr>,
    //       "architectures": [
    //         "x86_64",
    //         "i686"
    //       ],
    architectures: Vec<String>,
    //       "certificate": "X509 PEM certificate",
    certificate: String,
    //       "certificate_fingerprint": "fd200419b271f1dc2a5591b693cc5774b7f234e1ff8c6b78ad703b6888fe2b69",
    certificate_fingerprint: String,
    //       "driver": "lxc | qemu",
    driver: String,
    //       "driver_version": "4.0.7 | 5.2.0",
    driver_version: String,
    //       "firewall": "nftables",
    firewall: String,
    //       "kernel": "Linux",
    kernel: String,
    //       "kernel_architecture": "x86_64",
    kernel_architecture: String,
    //       "kernel_features": {
    //         "netnsid_getifaddrs": "true",
    //         "seccomp_listener": "true"
    //       },
    kernel_features: KernelFeatures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KernelFeatures {
    //       "kernel_version": "5.4.0-36-generic",
    kernel_version: Option<String>,
    //       "lxc_features": {
    //         "cgroup2": "true",
    //         "devpts_fd": "true",
    //         "pidfd": "true"
    //       },
    lxc_features: Option<Value>,
    //       "os_name": "Ubuntu",
    os_name: Option<String>,
    //       "os_version": "22.04",
    os_version: Option<String>,
    //       "project": "default",
    project: Option<String>,
    //       "server": "incus",
    server: Option<String>,
    //       "server_clustered": false,
    server_clustered: Option<bool>,
    //       "server_event_mode": "full-mesh",
    server_event_mode: Option<String>,
    //       "server_name": "castiana",
    server_name: Option<String>,
    //       "server_pid": 1453969,
    server_pid: Option<Number>,
    //       "server_version": "4.11",
    server_version: Option<String>,
    //       "storage": "dir | zfs",
    storage: Option<String>,
    //       "storage_supported_drivers": [
    //         {
    //           "Name": "zfs",
    //           "Remote": false,
    //           "Version": "0.8.4-1ubuntu11"
    //         }
    //       ],
    storage_supported_drivers: Option<Value>,
    //       "storage_version": "1 | 0.8.4-1ubuntu11"
    storage_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::io::Result;
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn test_server_root_get() -> Result<()> {
        let c = new_client();
        let r = c.get(incus_path("")).send()
            .await.unwrap()
            .json::<Server>()
            .await;

        println!("{:#?}", r);

        assert_ok!(r);
        Ok(())
    }
}

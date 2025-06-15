use tokio_multi_proxy;
use tokio;
use anyhow;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let remote = "192.168.1.133:44008";
    tokio_multi_proxy::start_mtls_tcp("127.0.0.1:4401", remote, "cert.pem", "key.pem", "ca.pem").await
}

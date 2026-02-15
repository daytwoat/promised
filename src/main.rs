mod blocklist;
mod dns_server;
mod system_check;


use crate::blocklist::Blocklist;


use std::sync::Arc;
use tokio::sync::RwLock;

mod http_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    system_check::ensure_supported_environment()?;

    //get users inital config later

    let config = Blocklist::load_from_file("./config.json")?;
    let shared_state = Arc::new(RwLock::new(config));

    let dns = tokio::spawn(dns_server::run_dns_server());
    let http = tokio::spawn(http_server::run_http_server(shared_state.clone()));

    tokio::select! {
        res = dns => res??,
        res = http => res??,
    }

    Ok(())
}

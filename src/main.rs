mod blocklist;
mod dns_server;
mod system_check;

use anyhow::Result;

use crate::blocklist::Blocklist;

#[tokio::main]
async fn main() -> Result<()> {
    system_check::ensure_supported_environment()?;

    ///get users inital config later
    let blocklist = Blocklist::load_from_file("config.json")?;

    dns_server::run_dns_server().await?;

    Ok(())
}

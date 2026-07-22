use anyhow::Result;
use tokio::net::UdpSocket;

use crate::blocklist::Blocklist;
use hickory_proto::op::{Message, MessageType};
use hickory_proto::rr::rdata::A;
use hickory_proto::rr::{Name, RData, Record};
use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};
use std::net::Ipv4Addr;

use std::net::SocketAddr;
use tokio::time::{Duration, timeout};

use std::sync::Arc;
use tokio::sync::RwLock;

const PORT: u16 = 5300;

async fn forward_query(query_bytes: &[u8], upstream: &str) -> Result<Vec<u8>> {
    // bind temporary socket to send query
    let local = UdpSocket::bind("0.0.0.0:0").await?;
    let upstream_addr: SocketAddr = format!("{}:53", upstream).parse()?;

    // send to dns
    local.send_to(query_bytes, &upstream_addr).await?;

    // wait response with 2 seconds timeout
    let mut buf = [0u8; 512];
    let (len, _) = timeout(Duration::from_secs(2), local.recv_from(&mut buf)).await??;

    Ok(buf[..len].to_vec())
}

//idk why domains end with .
fn normalize_domain(domain: &str) -> String {
    let mut d = domain.to_lowercase();
    if d.ends_with('.') {
        d.pop();
    }
    d
}

pub async fn run_dns_server(state: Arc<RwLock<Blocklist>>) -> Result<()> {
    let addr: String = format!("127.0.0.1:{}", PORT);
    let socket: UdpSocket = UdpSocket::bind(&addr).await?;
    println!("DNS server running on {}", addr);
    let mut buf: [u8; 512] = [0u8; 512];

    loop {
        let (len, src_addr) = socket.recv_from(&mut buf).await?;
        let message = Message::from_bytes(&buf[..len])?;

        for query in message.queries() {
            let name = normalize_domain(&query.name().to_utf8());
            println!("queury from {} → {}", src_addr, name);

            let blocked = {
                let blocklist = state.read().await;
                blocklist.is_blocked(&name)
            };
            println!("is blocked {name} {:?}", blocked);

            let response_bytes = if blocked {
                let mut response = Message::new();
                response.set_id(message.id());
                response.set_message_type(MessageType::Response);
                response.set_recursion_desired(true);
                response.set_recursion_available(true);
                let record = Record::from_rdata(
                    Name::from_utf8(&name)?,
                    60,
                    RData::A(A(Ipv4Addr::new(0, 0, 0, 0))),
                );
                response.add_answer(record);
                response.to_bytes()?
            } else {
                forward_query(&buf[..len], "8.8.8.8").await?
            };
            socket.send_to(&response_bytes, &src_addr).await?;
        }
    }
}

use std::net::{IpAddr, Ipv6Addr, SocketAddr, ToSocketAddrs};

use anyhow::Result;
use serde::{Deserialize, Serialize};

// Reference source: https://github.com/horizonx-tech/chainsight-backend/blob/c16741812c63ddf7cbe7519d12743c5988daf689/functions/internal/src/app/deploy_relayer/deploy_relayer.rs#L195-L236

pub fn is_supporting_ipv6_url(url_str: &str) -> Result<()> {
    let url = url_str.parse::<url::Url>()?;
    let scheme = url.scheme();
    if scheme != "https" {
        anyhow::bail!("Only HTTPS is acceptable for RPC URL, but got: {}", scheme)
    }
    let host = url
        .host()
        .ok_or_else(|| anyhow::anyhow!("No host in RPC URL"))?;
    match host {
        url::Host::Ipv4(_) => anyhow::bail!("ipv4 address is not acceptable for RPC URL"),
        url::Host::Ipv6(_) => Ok(()),
        url::Host::Domain(domain) => {
            if is_ipv6_supported(domain) {
                Ok(())
            } else {
                anyhow::bail!("Ipv6 not supported")
            }
        }
    }
}

fn is_ipv6_supported(domain: &str) -> bool {
    let socket_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 443);
    let ips: Vec<SocketAddr> = domain
        .to_socket_addrs()
        .unwrap()
        .filter(|addr| match addr {
            SocketAddr::V6(_) => true,
            _ => false,
        })
        .collect();
    for ip in ips {
        if ip == socket_addr {
            return true;
        }
    }
    false
}

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    method: String,
    params: Vec<serde_json::Value>,
    id: u64,
    jsonrpc: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct JsonRpcResponse {
    id: u64,
    jsonrpc: String,
    #[serde(rename = "result")]
    block_number: String,
}

pub fn is_valid_rpc_url(url: &str) -> Result<()> {
    let request = JsonRpcRequest {
        method: "eth_blockNumber".to_string(),
        params: Vec::new(),
        id: 1,
        jsonrpc: "2.0".to_string(),
    };

    let _ = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(&request)?
        .into_json::<JsonRpcResponse>()?;
    Ok(())
}

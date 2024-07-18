use anyhow::{Ok, Result};
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
        url::Host::Ipv4(_) => anyhow::bail!("IPv4 address is not acceptable in Internet Computer"),
        url::Host::Ipv6(_) => Ok(()),
        url::Host::Domain(domain) => is_ipv6_supported_domain(domain),
    }
}

fn is_ipv6_supported_domain(domain: &str) -> Result<()> {
    let ips: Vec<std::net::IpAddr> = dns_lookup::lookup_host(domain)?;
    for ip in ips {
        if ip.is_ipv6() {
            return Ok(());
        }
    }
    anyhow::bail!("No IPv6 address found, IPv4 address is not acceptable in Internet Computer")
}

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    method: String,
    params: Vec<serde_json::Value>,
    id: u64,
    jsonrpc: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct JsonRpcResponse {
    id: u64,
    jsonrpc: String,
    result: Option<String>,
    error: Option<String>,
}

pub fn is_valid_rpc_url(url: &str) -> Result<()> {
    let request = JsonRpcRequest {
        method: "eth_blockNumber".to_string(),
        params: Vec::new(),
        id: 1,
        jsonrpc: "2.0".to_string(),
    };

    let res = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(&request)?
        .into_json::<JsonRpcResponse>()?;
    if let Some(err) = res.error {
        anyhow::bail!("Error in response {:?} from calling eth_blockNumber", err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ref: test ip,domains
    //   https://developers.google.com/speed/public-dns/docs/using

    #[test]
    fn test_is_supporting_ipv6_url() {
        assert!(is_supporting_ipv6_url("https://ipv4.google.com").is_err());
        assert!(is_supporting_ipv6_url("https://ipv6.google.com").is_ok());
        assert!(is_supporting_ipv6_url("https://eth-mainnet.g.alchemy.com").is_ok());
        assert!(is_supporting_ipv6_url("https://eth-mainnet.g.alchemy.com/v2/${KEY}").is_ok());
    }

    #[test]
    fn test_is_ipv6_supported_domain() {
        assert!(is_ipv6_supported_domain("api.coingecko.com").is_ok());
        assert!(is_ipv6_supported_domain("ipv6.google.com").is_ok());
        assert!(is_ipv6_supported_domain("ipv4.google.com").is_err());
    }

    #[test]
    fn test_is_valid_rpc_url() {
        assert!(is_valid_rpc_url("https://eth.llamarpc.com").is_ok());
        assert!(is_valid_rpc_url("https://eth-mainnet.g.alchemy.com/v2/${KEY}").is_err());
    }
}

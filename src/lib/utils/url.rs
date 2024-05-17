use anyhow::Result;

// Reference source: https://github.com/horizonx-tech/chainsight-backend/blob/c16741812c63ddf7cbe7519d12743c5988daf689/functions/internal/src/app/deploy_relayer/deploy_relayer.rs#L195-L236

pub async fn is_supporting_ipv6_url(url_str: &str) -> Result<()> {
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
        url::Host::Domain(domain) => match is_ipv6_supported(domain).await {
            Ok(_) => Ok(()),
            Err(_) => anyhow::bail!("Ipv6 not supported"),
        },
    }
}

async fn is_ipv6_supported(domain: &str) -> Result<()> {
    let ips = tokio::net::lookup_host(domain.to_owned() + ":443").await?;
    for ip in ips {
        match ip {
            std::net::SocketAddr::V6(_) => return Ok(()),
            _ => continue,
        }
    }
    anyhow::bail!("No IPv6 address found for domain: {}", domain)
}

pub async fn is_valid_rpc_url(url: &str) -> Result<()> {
    let tr = web3::transports::Http::new(url)?;
    let web = web3::Web3::new(tr);
    let _ = web.eth().block_number().await?;
    Ok(())
}

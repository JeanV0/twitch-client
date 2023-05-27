use std::time::Duration;

use reqwest::{ClientBuilder, Proxy};
use reqwest_middleware::ClientWithMiddleware as Client;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0";

pub struct ClientHttp {
  pub client: Client,
}

impl ClientHttp {
  pub fn new(proxy: bool, proxy_url: String ) -> Result<Self,reqwest::Error> {
    let mut cb = ClientBuilder::new()
      .user_agent(USER_AGENT)
      .timeout(Duration::from_secs(45));
    if proxy  {
      cb = cb.proxy(Proxy::http(proxy_url).unwrap());
    } else {
      cb = cb.no_proxy();
    }
    let client = cb.build()?;
    
    let backoff = ExponentialBackoff::builder()
    .retry_bounds(Duration::from_millis(1), Duration::from_secs(2))
    .build_with_total_retry_duration(Duration::from_secs(15));
    
    let client = reqwest_middleware::ClientBuilder::new(client)
    .with(RetryTransientMiddleware::new_with_policy(backoff))
    .build();

    Ok(
      Self {
        client,
      }
    )
      
  }
}
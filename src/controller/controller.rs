use std::{time::Duration, sync::{Arc, Mutex}};

use crate::hls::{StructRequestH3u8, ErrorHttpM3u8};
use std::thread::sleep;

pub struct Counter {
  pub ads_view: i64
}

impl Counter {
    pub fn increment(&mut self, v: i64) {
      self.ads_view += v;
    }
}

pub struct Controller {
  quantity: i64,
  timeout: u64,
  client: StructRequestH3u8,
  url_m3u8: String
}

impl Controller {
  pub async fn new(quantity: i64, timeout: u64, channel: String) -> Result<Self, ErrorHttpM3u8> {
    let client = match StructRequestH3u8::new(channel).await {
      Ok(r) => r,
      Err(e) => {
          return Err(ErrorHttpM3u8::new(e))
      }
    };

    let url_m3u8 = match client.get_m3u8_channel().await {
      Ok(r) => r,
      Err(e) => {
          return Err(ErrorHttpM3u8::new(e))
      }
  };

    Ok(Self {
      quantity,
      timeout,
      client,
      url_m3u8,
    })
  }

  pub async fn run(&self, counter: &mut i64) -> Result<i64, ErrorHttpM3u8> {
    let mut quantity: i64 = 0;


    while *counter < self.quantity {
      quantity = self.client.play_without_player(self.url_m3u8.clone())
      .await
      .map_err(ErrorHttpM3u8::new)?;      
      sleep(Duration::from_secs(self.timeout));
      *counter += 1;
      println!("{}", counter);
      *counter += quantity;
    }

    Ok(quantity)
  }
}
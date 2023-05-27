use http::header::USER_AGENT;
use m3u8_rs::{Playlist, MediaPlaylist};
use rand::Rng;
use serde::{Deserialize};
use serde_json::json;
use url::Url;
use std::{env, time::Duration};
use std::thread::sleep;
use crate::hls::{generate_id, get_rng, ClientHttp, errors::{*}};

const BASE: &str = "https://usher.ttvnw.net";

pub const USER_AGENTE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0";
const TWITCH_CLIENT: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko";
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PlaybackAccessToken {
    pub(crate) value: String,
    pub(crate) signature: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct AccessTokenResponse {
    pub(crate) data: Data,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Data {
    #[serde(rename = "streamPlaybackAccessToken", alias = "videoPlaybackAccessToken")]
    #[serde(default)]
    pub(crate) playback_access_token: Option<PlaybackAccessToken>,
}
pub struct StructRequestH3u8 {
    twitch_channel: String,
    client: ClientHttp,
    access_token: PlaybackAccessToken,
}

impl StructRequestH3u8 {
    pub async fn new(twitch: String) -> Result<Self,CreateClient> {
      let proxy_url = env::var("PROXY_URL").unwrap();
      let proxy = env::var("PROXY_CONDITION").unwrap();
      let proxy_condition = proxy.parse::<bool>().unwrap();
      

    let client = match ClientHttp::new( 
      proxy_condition,
      proxy_url) 
      {
      Ok(client_http) => client_http,
      Err(err) => {
        return Err(CreateClient::NetworkError(err))
      },
    }; 

      let request = json!({
        "operationName": "PlaybackAccessToken_Template",
        "query": "query PlaybackAccessToken_Template($login: String!, $isLive: Boolean!, $vodID: ID!, $isVod: Boolean!, $playerType: String!) {  streamPlaybackAccessToken(channelName: $login, params: {platform: \"web\", playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isLive) {    value    signature    __typename  }  videoPlaybackAccessToken(id: $vodID, params: {platform: \"web\", playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isVod) {    value    signature    __typename  }}",
        "variables": {
            "isLive": true,
            "login": twitch,
            "isVod": false,
            "vodID": "",
            "playerType": "site",
        },
    });

    let access_token = client.client
    .post("https://gql.twitch.tv/gql")
    .header("Client-ID", TWITCH_CLIENT)
    .header("Device-ID", &generate_id())
    .header(USER_AGENT, USER_AGENTE)
    .json(&request)
    .send()
    .await
    .map_err(CreateClient::MiddlewareError)?
    .error_for_status()
    .map_err(CreateClient::NetworkError)?
    .json::<AccessTokenResponse>()
    .await
    .map_err(CreateClient::NetworkError)?;

    match access_token.data.playback_access_token {
      Some(playback_token) => {
        Ok(
          Self {
            twitch_channel: twitch,
            client,
            access_token: playback_token,  
          }  
        )
      }
      None => {
          return Err(CreateClient::StreamNameError(twitch))
      }
  }


    } 

  pub async fn get_m3u8_channel(&self) -> Result<String, CreateClient> {
    let channel = &self.twitch_channel;
    let mut url_twitch = Url::parse(&format!("{BASE}/api/channel/hls/{channel}.m3u8")).map_err(CreateClient::UrlError)?;

    url_twitch.query_pairs_mut()
    .append_pair("p", &get_rng().gen_range(0..=9_999_999).to_string())
    .append_pair("play_session_id", &generate_id().to_ascii_lowercase())
    .append_pair("token", &self.access_token.value)
    .append_pair("sig", &self.access_token.signature)
    .append_pair("acmb", "e30=");

    let m3u8 = self.client.client
    .get(url_twitch)
    .header(USER_AGENT, USER_AGENTE)
    .send()
    .await
    .map_err(CreateClient::MiddlewareError)?
    .error_for_status()
    .map_err(CreateClient::NetworkError)?
    .text()
    .await
    .map_err(CreateClient::NetworkError)?;
    
    let mut url: String = String::from("Nothing");

    let lines: Vec<&str> = m3u8.lines().collect();
    let mut iter = lines.iter().enumerate().peekable();
    while let Some((_, line)) = iter.next() {
      if line.contains("480p30") {
        iter.next();
        if let Some((_, next_line)) = iter.next() {
          url = next_line.to_string()
        }
      }
    }
    Ok(url)
  }

  fn process_media_playlist(&self,media_playlist: MediaPlaylist) {
    for segment in media_playlist.segments {

      sleep(Duration::from_secs(2));
    }
  }

  fn get_twitch_id_session(&self,stream: String) -> (String, i64) {
    let mut count = 0;
    let iter_stream: Vec<&str> = stream.lines().collect();
    let mut ads: String = String::from("None");
    let mut iter_streams = iter_stream.iter().enumerate().peekable();
    while let Some((_index, line)) = iter_streams.next() {
      ads = line.to_string();
      let mut id: String = String::from("");
      let vars: Vec<&str> = ads.split(",").collect();
      for ele in vars {
        if ele.contains("X-TV-TWITCH-AD-POD-LENGTH") {
          let env: Vec<&str> = ele.split("=").collect();
          count = env[1].to_string().replace('"', "").parse().unwrap();
        }

        if ele.contains("X-TV-TWITCH-AD-AD-SESSION-ID") {
          let env: Vec<&str> = ele.split("=").collect();
          id = env[1].to_string().replace('"', "");
        }
      }
      ads = id;
      break;

    }
    return (ads, count)
  }

  pub async fn play_without_player(&self, url: String) -> Result<i64, CreateClient> {
    let mut count: i64 = 0;
    let mut _ad_closed = true;
    println!("La ele");
    while _ad_closed {
      let mut _id_twitch: String;
      let stream: String = self.client.client
      .get(url.clone())
      .send()
      .await
      .map_err(CreateClient::MiddlewareError)?
      .error_for_status()
      .map_err(CreateClient::NetworkError)?
      .text()
      .await
      .map_err(CreateClient::NetworkError)?;

      if stream.contains("X-TV-TWITCH-AD") {
        _ad_closed = true;
        break;
      }

      (_id_twitch, count) = self.get_twitch_id_session(stream.clone());
      
      println!("{}", count);

      match m3u8_rs::parse_playlist(stream.as_bytes()) {
        Result::Ok((_, Playlist::MasterPlaylist(_))) => println!("Executando"),
        Result::Ok((_, Playlist::MediaPlaylist(pl))) => {
          self.process_media_playlist(pl);
        },
        Result::Err(e) =>  panic!("Parsing error: \n{}", e),
      }
    };
    println!("{}", count);

    Ok(count)
  }
}
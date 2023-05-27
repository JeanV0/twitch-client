use std::fmt;

#[derive(Debug)]
pub enum CreateClient {
  NetworkError(reqwest::Error),
  MiddlewareError(reqwest_middleware::Error),
  StreamNameError(String),
  // IOError(std::io::Error),
  // CustomError(String),
  UrlError(url::ParseError)
}
pub struct ErrorHttpM3u8 {
  pub error: CreateClient
}

impl ErrorHttpM3u8 {
  pub fn new(err: CreateClient) -> Self {
    Self {
      error: err,
    }
  }
}

impl std::error::Error for ErrorHttpM3u8 {}

impl fmt::Display for ErrorHttpM3u8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Erro HTTP M3U8: {}", self.details())
    }
}

impl fmt::Debug for ErrorHttpM3u8 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ErrorHttpM3u8: \n{}", self.details())
  }
}

impl ErrorHttpM3u8 {
  fn details(&self) -> String {
    let mut return_error: String = String::from("Unexpected");
    let error = &self.error;
    match error {
        CreateClient::NetworkError(err) => {
          if err.is_status() {
            if let Some(final_stop) = err.status() {
              if final_stop.is_server_error() {
                return_error = String::from(format!("Request error: send to support this error\nError: {}\nUrlç", final_stop).as_str());
              } else if final_stop.is_client_error() {
                return_error = String::from(format!("Status code error: Streamer not found or any resource\nError: {}",final_stop).as_str());
              } else if final_stop.is_redirection() {
                return_error = String::from(format!("Redirection error: Unknow possible error\nError: {}",final_stop).as_str());
              } else if final_stop.is_success() {
              }
            };
          } else if err.is_body() {
            return_error = String::from(format!("Body error: send to suport this error\nError: {}",err).as_str());
          } else if err.is_request() {
            return_error = String::from(format!("Request error: send to suport this error\nError: {}",err).as_str());
          } else if err.is_decode() {
            return_error = String::from(format!("Decode error: send to suport this error\nError: {}",err).as_str());
          } else if err.is_timeout() {
            return_error = String::from(format!("timeout error: check your proxy or internet to bad connection\nError: {}",err).as_str());

          }
        },
        CreateClient::MiddlewareError(err) => {
          return_error = String::from(format!("Error in middleware: {}", err.to_string()))
        },
        CreateClient::UrlError(err) => {
          return_error = String::from(format!("Error in url: {}", err.to_string()))
        },
        CreateClient::StreamNameError(err) => {
          return_error = String::from(format!("Error in name streamer: You put a invalid name \nName: {}", err.to_string()))

        }
    };
    return_error
  }
}
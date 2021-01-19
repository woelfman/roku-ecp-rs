pub use error::Error;
use surf::Client;
mod api;
mod error;

/// HTTP port for communicating with the ECP RESTful service.
const ECP_PORT: &str = "8060";

/// A Roku device to communicate with via the [External Control Protocol](https://developer.roku.com/docs/developer-program/debugging/external-control-api.md).
pub struct Device {
    /// Base URL of the Roku device's IP and port.
    pub url: String,
    /// Http client for communicating with Roku device.
    http: Client,
}

impl Device {
    /// Constructs a client to communicate with a Roku device. This assumes the device is on the local network.
    pub fn new(ip: &str) -> Device {
        Device {
            url: format!("http://{}:{}", ip, ECP_PORT),
            http: Client::new(),
        }
    }

    /// Constructs a client to communicate with a Roku at the specified URL .
    pub fn from_url(url: &str) -> Device {
        Device {
            url: url.to_owned(),
            http: Client::new(),
        }
    }

    /// Ping the Roku device to test the connection.
    pub async fn ping(&self) -> Result<(), Error> {
        self.http.get(&self.url).send().await?;
        Ok(())
    }
}

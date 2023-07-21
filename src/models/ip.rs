use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct IpAddr {
    pub ip: Option<String>,
}
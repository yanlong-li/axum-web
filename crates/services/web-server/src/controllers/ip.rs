use axum::extract::Query;
use axum::response::Response;
use maxminddb::{geoip2, MaxMindDBError};
use serde::{Deserialize, Serialize};

use lib_utils::ip_library::get_reader;
use crate::utils::response::{error, success};
use crate::utils::response::status_code::StatusCode;

#[derive(Serialize,Deserialize)]
pub struct IpAddr {
    pub ip: Option<String>,
}

pub async fn action_ip_info(
    Query(ip_addr): Query<IpAddr>
) -> Response {
    let reader = get_reader();


    match ip_addr.ip {
        Some(ip) => {
            let info_result: Result<geoip2::City, MaxMindDBError> = reader.lookup(ip.parse().unwrap());
            match info_result {
                Ok(data) => success(Some(data)),
                Err(_) => error(StatusCode::NOT_FOUND)
            }
        }
        None => error(StatusCode::NOT_FOUND)
    }
}
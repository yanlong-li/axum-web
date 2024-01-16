use axum::extract::Query;
use axum::response::Response;
use maxminddb::{geoip2, MaxMindDBError};

use crate::models::IpAddr;
use lib_utils::ip_library::get_reader;
use crate::utils::response::{client, error, success};

pub async fn action_ip_info(
    Query(ip_addr): Query<IpAddr>
) -> Response {
    let reader = get_reader();


    match ip_addr.ip {
        Some(ip) => {
            let info_result: Result<geoip2::City, MaxMindDBError> = reader.lookup(ip.parse().unwrap());
            match info_result {
                Ok(data) => success(Some(data)),
                Err(_) => error(client::ClientStatusCode::NOT_FOUND_IP)
            }
        }
        None => error(client::ClientStatusCode::NOT_FOUND_IP)
    }
}
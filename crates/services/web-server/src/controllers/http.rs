use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(2)).timeout(std::time::Duration::from_secs(5))
    .tcp_keepalive(std::time::Duration::from_secs(30)).build().expect("Failed to create client");
}

pub async fn action_request() -> String {
    let a = CLIENT.get("https://ip.yanlongli.com").send().await.unwrap().text().await.unwrap();
    a
}
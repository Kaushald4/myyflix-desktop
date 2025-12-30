use reqwest::Client;

pub fn client() -> Client {
    Client::builder().user_agent("Mozilla/5.0").build().unwrap()
}

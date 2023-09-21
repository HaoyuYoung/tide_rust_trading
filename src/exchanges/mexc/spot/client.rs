use crate::exchanges::Exchanges;

pub struct Client {
    api_key: String,
    secret_key: String,
    url: String,
}
impl Exchanges for Client{
    fn new_client(&self) -> Self {
        todo!()
    }
}
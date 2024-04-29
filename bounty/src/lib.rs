
#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}

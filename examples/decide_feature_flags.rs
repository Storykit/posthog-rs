use posthog_rs::{Client, ClientOptionsBuilder, PublicAPI};

async fn run() {
    let client_options = ClientOptionsBuilder::new()
        .set_endpoint(dotenv::var("POSTHOG_URL").unwrap())
        .set_api_key(dotenv::var("POSTHOG_PROJECT_API_KEY").unwrap())
        .build()
        .unwrap();

    let client = Client::new(client_options);
    let feature_flags = client.decide("1234".to_owned()).await.unwrap();

    for flag in &feature_flags {
        println!("name: {}, value: {}", flag.0, flag.1);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}

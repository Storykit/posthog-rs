use posthog_rs::{ClientBuilder, Event, PublicAPI};

async fn run() {
    let client = ClientBuilder::new()
        .set_public_api_key(dotenv::var("POSTHOG_API_KEY").unwrap())
        .set_endpoint(dotenv::var("POSTHOG_URL").unwrap())
        .build()
        .unwrap();

    let mut event = Event::new("test", "1234");
    event.insert_prop("key1", "value1").unwrap();
    event.insert_prop("key2", vec!["a", "b"]).unwrap();

    let result = client.capture(event).await;
    println!("result {:?}", result);
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}

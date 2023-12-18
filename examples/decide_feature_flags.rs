use posthog_rs::{ClientBuilder, PublicAPI};

async fn run() {
    let client = ClientBuilder::new()
        .set_endpoint(dotenv::var("POSTHOG_URL").unwrap())
        .set_public_api_key(dotenv::var("POSTHOG_PROJECT_API_KEY").unwrap())
        .build()
        .unwrap();

    let feature_flags = client.decide("1234").await;

    match feature_flags {
        Ok(flags) => {
            for flag in &flags {
                println!("name: {}, value: {}", flag.0, flag.1);
            }
        }
        Err(e) => println!("error: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}

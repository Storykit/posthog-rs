use posthog_rs::{Client, ClientOptionsBuilder, FeatureFlagsAPI};

async fn run() {
    let client_options = ClientOptionsBuilder::new()
        .set_endpoint(dotenv::var("POSTHOG_URL").unwrap())
        .set_api_key(dotenv::var("POSTHOG_API_KEY").unwrap())
        .build()
        .unwrap();

    let client = Client::new(client_options).private();

    let feature_flags = client
        .list_feature_flags(&dotenv::var("POSTHOG_PROJECT_ID").unwrap())
        .await
        .unwrap();

    for flag in &feature_flags {
        println!("key: {}, name/desc: {}", flag.key, flag.name);
    }

    println!("==========================");

    if let Some(flag) = feature_flags.first() {
        let first_flag = client
            .get_feature_flag(&dotenv::var("POSTHOG_PROJECT_ID").unwrap(), &flag.key)
            .await
            .unwrap();
        println!("Specific data about the first flag:");
        println!("{:?}", first_flag);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}

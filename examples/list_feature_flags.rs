use posthog_rs::{ClientBuilder, FeatureFlagsAPI};

async fn run() {
    let client = ClientBuilder::new()
        .set_private_api_key(dotenv::var("POSTHOG_API_KEY").unwrap())
        .set_endpoint(dotenv::var("POSTHOG_URL").unwrap())
        .build()
        .unwrap();

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

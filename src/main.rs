use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Credentials, Region};
use polars::prelude::*;
use std::env;
use std::error::Error;
use std::io::Cursor;

// We don't use this, we use trigger-command instead.
// Keep it for now because we don't know whether trigger-command runs on spinkube.
// use spin_sdk::http::{IntoResponse, Request, Response};
// use spin_sdk::http_component;
// 
// /// A simple Spin HTTP component.
// #[http_component]
// fn handle_s3_wasm(req: Request) -> anyhow::Result<impl IntoResponse> {
//     println!("Handling request to {:?}", req.header("spin-full-url"));
//     Ok(Response::builder()
//         .status(200)
//         .header("content-type", "text/plain")
//         .body("Hello World!")
//         .build())
// }
// 

#[wstd::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get configuration from environment variables
    let bucket_name = env::var("SPIN_CONFIG_S3_BUCKET")?;
    let object_key = env::var("SPIN_CONFIG_S3_KEY")?;
    let region = env::var("SPIN_CONFIG_AWS_REGION")?;
    let access_key_id = env::var("SPIN_CONFIG_AWS_ACCESS_KEY_ID")?;
    let secret_access_key = env::var("SPIN_CONFIG_AWS_SECRET_ACCESS_KEY")?;
    let session_token = env::var("SPIN_CONFIG_AWS_SESSION_TOKEN")
        .ok()
        .filter(|s| !s.is_empty());

    // Load configuration with explicit credentials from environment variables
    let credentials = Credentials::new(
        &access_key_id,
        &secret_access_key,
        session_token,
        None,
        "environment",
    );

    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(wstd_aws::http_client())
        .sleep_impl(wstd_aws::sleep_impl())
        .region(Region::new(region))
        .credentials_provider(credentials)
        .load()
        .await;

    // Create the S3 Client and get the object
    let client = aws_sdk_s3::Client::new(&sdk_config);

    let output = client
        .get_object()
        .bucket(&bucket_name)
        .key(&object_key)
        .send()
        .await?;

    let data = output.body.collect().await?;
    let bytes = data.into_bytes();

    // Parse CSV with Polars
    let cursor = Cursor::new(bytes.as_ref());
    let df = CsvReadOptions::default()
        .into_reader_with_file_handle(cursor)
        .finish()?;

    // Group by Country and count
    let country_counts = df
        .clone()
        .lazy()
        .group_by([col("Country")])
        .agg([len().alias("Count")])
        .sort(
            ["Count"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?;

    // Non-lazy version, should we every need it:
    // let country_counts = df
    // .group_by(["Country"])?
    // .select(["Country"]) // Ensures we have the key
    // .count()?
    // .rename("Country_count", "Count".into())? // .count() creates a default name
    // .sort(
    //     ["Count"],
    //     SortMultipleOptions::default().with_order_descending(true),
    // )?;

    println!("{}", country_counts);

    Ok(())
}

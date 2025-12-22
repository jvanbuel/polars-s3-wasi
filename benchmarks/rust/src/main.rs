use aws_config::BehaviorVersion;
use polars::prelude::*;
use std::env;
use std::error::Error;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get configuration from environment variables
    let bucket_name = env::var("S3_BUCKET").unwrap_or_else(|_| "wasi-s3-dm".to_string());
    let object_key = env::var("S3_KEY").unwrap_or_else(|_| "data.csv".to_string());

    // Load configuration from environment variables (AWS_ACCESS_KEY_ID, etc.)
    let sdk_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

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

    println!("{}", country_counts);

    Ok(())
}

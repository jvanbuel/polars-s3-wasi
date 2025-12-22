import io
import os

import boto3
import polars as pl


def main():
    # Get configuration from environment variables
    bucket_name = os.environ.get("S3_BUCKET", "wasi-s3-dm")
    object_key = os.environ.get("S3_KEY", "data.csv")

    # Create S3 client (uses AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION from env)
    s3_client = boto3.client("s3")

    # Download the object
    response = s3_client.get_object(Bucket=bucket_name, Key=object_key)
    data = response["Body"].read()

    # Parse CSV with Polars
    df = pl.read_csv(io.BytesIO(data))

    # Group by Country and count
    country_counts = (
        df.lazy()
        .group_by("Country")
        .agg(pl.len().alias("Count"))
        .sort("Count", descending=True)
        .collect()
    )

    print(country_counts)


if __name__ == "__main__":
    main()

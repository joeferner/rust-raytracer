use anyhow::{Context, Result, anyhow};
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use serde::{Deserialize, Serialize};

pub async fn write_json_to_s3<T: Serialize>(
    client: &S3Client,
    bucket: &str,
    key: &str,
    data: &T,
) -> Result<()> {
    let json_string = serde_json::to_string(data)
        .with_context(|| format!("writing json to s3://{bucket}/{key}"))?;
    write_to_s3(
        client,
        bucket,
        key,
        "application/json",
        ByteStream::from(json_string.into_bytes()),
    )
    .await
}

pub async fn read_json_from_s3<T: for<'de> Deserialize<'de>>(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<T>> {
    let resp = read_from_s3(client, bucket, key)
        .await
        .with_context(|| format!("reading json from s3://{bucket}/{key}"))?;
    if let Some(resp) = resp {
        let data = resp.body.collect().await?;
        let bytes = data.into_bytes();
        let parsed: T = serde_json::from_slice(&bytes)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

pub async fn write_to_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
    content_type: &str,
    data: ByteStream,
) -> Result<()> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(data)
        .content_type(content_type)
        .send()
        .await
        .with_context(|| format!("writing to s3://{bucket}/{key}"))?;
    Ok(())
}

pub struct ReadFromS3Data {
    pub content_type: Option<String>,
    pub body: ByteStream,
}

pub async fn read_from_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<ReadFromS3Data>> {
    let resp = client.get_object().bucket(bucket).key(key).send().await;
    match resp {
        Ok(resp) => {
            let content_type = resp.content_type().map(|s| s.to_string());
            let body = resp.body;
            Ok(Some(ReadFromS3Data { content_type, body }))
        }
        Err(err) => {
            // Check if it's a NoSuchKey error
            if let Some(service_err) = err.as_service_error()
                && service_err.is_no_such_key()
            {
                Ok(None)
            } else {
                Err(anyhow!("reading s3://{bucket}/{key}: {err}"))
            }
        }
    }
}

/// Converts an email address into a valid S3 key by replacing or removing
/// characters that are problematic in S3 keys.
///
/// S3 keys can technically contain most characters, but some should be avoided:
/// - Special characters like &, $, @, =, ;, :, +, space, comma
/// - Characters that need URL encoding
///
/// This function:
/// - Replaces '@' with '_at_'
/// - Replaces '.' with '_'
/// - Replaces other special characters with '-'
/// - Converts to lowercase for consistency
pub fn email_to_s3_key(email: &str) -> String {
    email
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '@' => "_at_".to_string(),
            '.' => "_".to_string(),
            'a'..='z' | '0'..='9' | '-' | '_' => c.to_string(),
            _ => "-".to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_email() {
        assert_eq!(email_to_s3_key("user@example.com"), "user_at_example_com");
    }

    #[test]
    fn test_email_with_dots() {
        assert_eq!(
            email_to_s3_key("first.last@example.com"),
            "first_last_at_example_com"
        );
    }

    #[test]
    fn test_email_with_special_chars() {
        assert_eq!(
            email_to_s3_key("user+tag@example.com"),
            "user-tag_at_example_com"
        );
    }

    #[test]
    fn test_uppercase_email() {
        assert_eq!(email_to_s3_key("User@Example.COM"), "user_at_example_com");
    }
}

use hyper::body::Bytes;
use reqwest::Client;
use crate::{api::OnedriveError, error::Res};

const ITEMS_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive/items/";

/// Request the content of a drive item
/// Yields a download URL for the file, if it exists
/// Then, reqwest will follow that URL to download
/// Returns the bytes_stream of the download
pub async fn get_download_handle(
    access_token: String,
    photo_id: String
) -> Res<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
    let client = Client::new();

    // Redirect to temporary download URL
    let res = client.get(format!("{ITEMS_ENDPOINT}{photo_id}/content"))
        .bearer_auth(access_token.as_str())
        .send()
        .await?;

    // If the status indicates failure, don't bother with serde.
    if !res.status().is_success() {
        Err(OnedriveError::BadStatus)?;
    }

    Ok(res.bytes_stream())
}

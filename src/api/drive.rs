use serde::{Deserialize, Serialize};

use crate::api::make_request;
use crate::error::Res;

const URL: &str = "https://graph.microsoft.com/v1.0/me/drive";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DriveData {

    #[serde(rename = "@odata.context")]
    odata_context: String,
    #[serde(rename = "createdDateTime")]
    creation_date: String,
    #[serde(rename = "description")]
    description: Option<String>,

    pub id: Option<String>,

    #[serde(rename = "driveType")]
    pub drive_type: String,
    pub owner: Owner
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    pub user: User
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub email: Option<String>
}

/// Get the drive data for the current access token
pub async fn get_drive(access_token: String) -> Res<DriveData> {
    make_request::<DriveData>(URL, access_token, vec![]).await
}

/// Get the User owning the access token
pub async fn get_user(access_token: String) -> Res<User> {
    Ok(get_drive(access_token).await?.owner.user)
}

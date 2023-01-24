use serde_json::Value;

use anyhow::Result;
use crate::error::ReportableError;

#[derive(PartialEq, Debug, Deserialize)]
pub struct User {
    #[serde(rename = "name")]
    username: String,
    #[serde(rename = "id")]
    user_id: i64,
    description: String,
}

impl User {
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_user_id(&self) -> i64 {
        self.user_id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub async fn get_thumbnail(&self) -> Result<String, ReportableError> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "imageUrl")]
            url: String,
        }
        let response = reqwest::get(&format!("https://thumbnails.roblox.com/v1/users/avatar-bust?userIds={}&size=420x420&format=Png&isCircular=false", self.user_id)).await?;
        
        let data = response.json::<Value>().await?.get("data").and_then(|v| serde_json::from_value::<[Response; 1]>(v.to_owned()).ok());
        data.and_then(|data| { data
            .into_iter()
            .next()
            .map(|v| v.url)
        }).ok_or(ReportableError::InternalError("Failed to get avatar URL"))
    }

    pub async fn from_userid(user_id: i64) -> Result<Self, ReportableError> {
        let response =
            reqwest::get(&format!("https://users.roblox.com/v1/users/{}", user_id)).await?;
        if response.status().is_success() {
            response.json::<Self>().await.map_err(|e| e.into())
        } else {
            Err(ReportableError::InternalError("unexpected status code"))
        }
    }

    pub async fn from_username<T: AsRef<str>>(username: T) -> Result<Self, ReportableError> {
        let response = reqwest::get(&format!(
            "https://api.roblox.com/users/get-by-username?username={}",
            username.as_ref()
        ))
        .await?;
        if response.status().is_success() {
            Self::from_userid(
                response
                    .json::<Value>()
                    .await?
                    .get("Id")
                    .and_then(|v| v.as_i64())
                    .ok_or(ReportableError::InternalError("User does not exist"))?,
            )
            .await
        } else {
            Err(ReportableError::InternalError("unexpected status code"))
        }
    }

    pub async fn get_rank_in_group(&self, group_id: i64, rank: u8) -> Result<u8, ReportableError> {
        unimplemented!()
    }

    pub async fn set_rank_in_group(&self, group_id: i64, rank: u8) -> Result<u8, ReportableError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_fromuserid() {
        let user = User::from_userid(1).await.unwrap();
        assert_eq!(user.get_user_id(), 1);
        assert_eq!(user.get_username(), "Roblox");
    }

    #[tokio::test]
    async fn test_get_user_fromusername() {
        let user = User::from_username("Roblox").await.unwrap();
        assert_eq!(user.get_user_id(), 1);
        assert_eq!(user.get_username(), "Roblox");
    }


    // May fail if avatar URL changes
    #[tokio::test]
    async fn test_get_thumbnail() {
        let user = User::from_userid(1).await.unwrap();
        assert_eq!(user.get_thumbnail().await.unwrap(), "https://tr.rbxcdn.com/b7c2dce11d623d2261d6cc9368174a41/420/420/AvatarBust/Png".to_owned());
    }
}
use serde_json::Value;

use super::url;
use crate::error::ReportableError;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Group {
    group_id: i64,
    name: String,
    description: String,
    roles: Vec<Role>,
}

impl Group {
    pub async fn new(group_id: i64) -> Result<Self, ReportableError> {
        let info = reqwest::get(url::groups_v2_groups_1(group_id)).await?;
        if info.status().is_success() {
            let info: Group = info
                .json::<Value>()
                .await?
                .get(0)
                .and_then(|v| serde_json::from_value(v.to_owned()).ok())
                .ok_or(ReportableError::InternalError("fuck"))?;
            let roles = reqwest::get(url::groups_v1_roles(group_id)).await?;
            if roles.status().is_success() {}
        }
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
struct Role {
    role_id: i64,
    name: String,
    rank: i64,
    member_count: i64,
}

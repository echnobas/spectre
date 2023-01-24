use serde_json::Value;

use anyhow::Result;
use crate::error::ReportableError;

#[derive(Debug, Deserialize)]
pub struct Group {
    group_id: i64,
    name: String,
    description: String,
    roles: Vec<Role>,
}

impl Group {
    pub async fn new(group_id: i64) -> Result<Self, ReportableError> {
        let info = reqwest::get(&format!(
            "https://groups.roblox.com/v2/groups?groupIds={}",
            group_id
        ))
        .await?;
        if info.status().is_success() {
            let info: Group = info
                .json::<Value>()
                .await?
                .get(0)
                .and_then(|v| serde_json::from_value(v.to_owned()).ok())
                .ok_or(ReportableError::InternalError("fuck"))?;

            let roles = reqwest::get(&format!(
                "https://groups.roblox.com/v1/groups/{}/roles",
                group_id
            ))
            .await?;
            if roles.status().is_success() {}
        }
        unimplemented!()
        // let group = Self { group_id };
    }
}

#[derive(Debug, Deserialize)]
struct Role {
    role_id: i64,
    name: String,
    rank: i64,
    member_count: i64,
}

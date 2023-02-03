// https://groups.roblox.com/v2/groups?groupIds={}
pub fn groups_v2_groups_1(group_id: i64) -> String {
    format!("https://groups.roblox.com/v2/groups?groupIds={group_id}")
}

// https://groups.roblox.com/v1/groups/{}/roles
pub fn groups_v1_roles(group_id: i64) -> String {
    format!("https://groups.roblox.com/v1/groups/{group_id}/roles")
}

// https://users.roblox.com/v1/users/{}
pub fn user_v1_users(user_id: i64) -> String {
    format!("https://users.roblox.com/v1/users/{user_id}")
}

// https://api.roblox.com/users/get-by-username?username={}
pub fn users_get_by_username<S: AsRef<str>>(username: S) -> String {
    format!("https://api.roblox.com/users/get-by-username?username={}", username.as_ref())
}

// https://thumbnails.roblox.com/v1/users/avatar-bust?userIds={}&size=420x420&format=Png&isCircular=false
pub fn thumbnails_users_avatar_bust(user_id: i64) -> String {
    format!("https://thumbnails.roblox.com/v1/users/avatar-bust?userIds={user_id}&size=420x420&format=Png&isCircular=false")
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

impl Into<UserInfo> for &str {
    fn into(self) -> UserInfo {
        let split: Vec<&str> = self.splitn(2, ':').collect();
        let (username, password) = if split.len() == 2 {
            (split[0].to_owned(), split[1].to_owned())
        } else {
            (split[0].to_owned(), String::new())
        };
        UserInfo { username, password }
    }
}

impl Into<UserInfo> for String {
    fn into(self) -> UserInfo {
        self.as_str().into()
    }
}

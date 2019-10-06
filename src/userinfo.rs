#[derive(Clone, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

impl UserInfo {
    pub fn from_str(s: &str) -> Self {
        let split: Vec<&str> = s.splitn(2, ':').collect();
        let (username, password) = if split.len() == 2 {
            (split[0].to_owned(), split[1].to_owned())
        } else {
            (split[0].to_owned(), String::new())
        };
        UserInfo { username, password }
    }
}

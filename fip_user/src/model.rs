use crate::proto::{UserRes, UserSaveReq, UserUpdateReq};
use sqlx::FromRow;

/// TODO: documentation
#[derive(Debug, FromRow)]
pub struct Model {
    /// TODO: documentation
    pub avatar: String,
    /// TODO: documentation
    pub biography: String,
    /// TODO: documentation
    pub birth_date: i64,
    /// TODO: documentation
    pub cellphone: String,
    /// TODO: documentation
    pub email: String,
    /// TODO: documentation
    pub first_name: String,
    /// TODO: documentation
    pub gender: String,
    /// TODO: documentation
    pub id: String,
    /// TODO: documentation
    pub language_code: String,
    /// TODO: documentation
    pub last_name: String,
    /// TODO: documentation
    pub password: String,
    /// TODO: documentation
    pub registered_at: i64,
    /// TODO: documentation
    pub username: String,
}

/// TODO: documentation
impl From<&UserSaveReq> for Model {
    /// TODO: documentation
    fn from(req: &UserSaveReq) -> Self {
        Self {
            avatar: req.avatar.clone(),
            biography: req.biography.clone(),
            birth_date: req.birth_date,
            cellphone: req.cellphone.clone(),
            email: req.email.clone(),
            first_name: req.first_name.clone(),
            gender: req.gender.clone(),
            id: req.id.clone(),
            language_code: req.language_code.clone(),
            last_name: req.last_name.clone(),
            password: req.password.clone(),
            registered_at: req.registered_at,
            username: req.username.clone(),
        }
    }
}

/// TODO: documentation
impl From<&UserUpdateReq> for Model {
    /// TODO: documentation
    fn from(req: &UserUpdateReq) -> Self {
        Self {
            avatar: req.avatar.clone(),
            biography: req.biography.clone(),
            birth_date: req.birth_date,
            cellphone: req.cellphone.clone(),
            email: req.email.clone(),
            first_name: req.first_name.clone(),
            gender: req.gender.clone(),
            id: req.id.clone(),
            language_code: req.language_code.clone(),
            last_name: req.last_name.clone(),
            password: req.password.clone(),
            registered_at: req.registered_at,
            username: req.username.clone(),
        }
    }
}

/// TODO: documentation
impl Into<UserRes> for Model {
    /// TODO: documentation
    fn into(self) -> UserRes {
        UserRes {
            avatar: self.avatar,
            biography: self.biography,
            birth_date: self.birth_date,
            cellphone: self.cellphone,
            email: self.email,
            first_name: self.first_name,
            gender: self.gender,
            id: self.id,
            language_code: self.language_code,
            last_name: self.last_name,
            password: self.password,
            registered_at: self.registered_at,
            username: self.username,
        }
    }
}

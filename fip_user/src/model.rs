use crate::proto::{UserResDto, UserSaveReqDto, UserUpdateReqDto};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Model {
    pub avatar: String,
    pub biography: String,
    pub birth_date: i64,
    pub cellphone: String,
    pub email: String,
    pub first_name: String,
    pub gender: String,
    pub id: String,
    pub language_code: String,
    pub last_name: String,
    pub password: String,
    pub registered_at: i64,
    pub username: String,
}

impl From<&UserSaveReqDto> for Model {
    fn from(req: &UserSaveReqDto) -> Self {
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

impl From<&UserUpdateReqDto> for Model {
    fn from(req: &UserUpdateReqDto) -> Self {
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

impl Into<UserResDto> for Model {
    fn into(self) -> UserResDto {
        UserResDto {
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

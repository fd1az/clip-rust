use crate::domain::clip::field;
use crate::ShortCode;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
    pub shortcode: field::ShortCode,
}

impl GetClip {
    pub fn from_raw(sc: &str) -> Self {
        Self {
            shortcode: ShortCode::from(sc),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(shortcode: &str) -> Self {
        Self::from_raw(shortcode)
    }
}

//! Models of the Tag section.

use serde::{Deserialize, Serialize};

/// TagListResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagListResponse {
    /// Рядок "success".
    /// Допустимі значення: `success`.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Масив об'єктів з даними користувацьких міток.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagsResponseObject>>,
}

/// Об'єкт із властивостями користувацьких міток.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagsResponseObject {
    /// Унікальний id мітки.
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i64>,
    /// Назва мітки.
    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// TagDeleteRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TagDeleteRequest {
    /// Унікальний id мітки.
    #[serde(rename = "tag_id", default)]
    pub tag_id: i64,
}

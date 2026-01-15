use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::types::store::{load_from_store, write_to_store};

/// 应用设置
#[derive(Debug, Serialize, Deserialize)]
pub struct AppUsttings {
    pub title: String,
    // 主题
    pub theme: String,
    // 认证令牌
    pub auth_token: Option<String>,
}

impl Default for AppUsttings {
    fn default() -> Self {
        Self {
            title: Default::default(),
            theme: "system".to_string(),
            auth_token: Default::default(),
        }
    }
}

impl AppUsttings {
    pub fn load_from_store(app: &AppHandle) -> anyhow::Result<Self> {
        Ok(load_from_store::<AppUsttings>(
            app,
            "settings.json",
            "appSettings",
        )?)
    }
    pub fn write_to_store(&self, app: &AppHandle) -> anyhow::Result<()> {
        Ok(write_to_store(
            app,
            "settings.json",
            "appSettings",
            self,
            true,
        )?)
    }
}

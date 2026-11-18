use std::io::ErrorKind;
use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::error::{AppResult, ensure_range};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
const PREVIEW_CHARS: usize = 1000;
const MAX_RECENT: usize = 5;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenedFile {
    path: String,
    size: u64,
    lines: usize,
    preview: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentFiles {
    store_path: String,
    paths: Vec<String>,
}

#[tauri::command]
pub async fn open_text_file(app: AppHandle, path: String) -> AppResult<OpenedFile> {
    let size = tokio::fs::metadata(&path).await?.len();
    ensure_range("文件大小", size, 0, MAX_FILE_SIZE)?;
    let content = tokio::fs::read_to_string(&path).await?;

    let mut paths = load_recent(&app).await?;
    paths.retain(|p| p != &path);
    paths.insert(0, path.clone());
    paths.truncate(MAX_RECENT);
    save_recent(&app, &paths).await?;

    Ok(OpenedFile {
        path,
        size,
        lines: content.lines().count(),
        preview: content.chars().take(PREVIEW_CHARS).collect(),
    })
}

#[tauri::command]
pub async fn recent_files(app: AppHandle) -> AppResult<RecentFiles> {
    Ok(RecentFiles {
        store_path: recent_path(&app)?.display().to_string(),
        paths: load_recent(&app).await?,
    })
}

fn recent_path(app: &AppHandle) -> AppResult<PathBuf> {
    Ok(app.path().app_data_dir()?.join("recent.json"))
}

async fn load_recent(app: &AppHandle) -> AppResult<Vec<String>> {
    match tokio::fs::read_to_string(recent_path(app)?).await {
        Ok(json) => Ok(serde_json::from_str(&json)?),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(err.into()),
    }
}

async fn save_recent(app: &AppHandle, paths: &[String]) -> AppResult<()> {
    tokio::fs::create_dir_all(app.path().app_data_dir()?).await?;
    tokio::fs::write(recent_path(app)?, serde_json::to_string_pretty(paths)?).await?;
    Ok(())
}

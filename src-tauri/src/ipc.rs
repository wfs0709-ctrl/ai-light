use ai_light::aggregator::StateAggregator;
use ai_light::hook_installer::{check_hooks_installed, install_hooks, preview_hook_config};
use ai_light::types::LightState;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn confirm_light(project_id: String, aggregator: State<Arc<StateAggregator>>) {
    aggregator.confirm_light(&project_id);
}

#[tauri::command]
pub fn remove_light(project_id: String, aggregator: State<Arc<StateAggregator>>) {
    aggregator.remove_light(&project_id);
}

#[tauri::command]
pub fn get_lights(aggregator: State<Arc<StateAggregator>>) -> Vec<LightState> {
    aggregator.get_lights()
}

#[tauri::command]
pub fn open_project(project_id: String) -> Result<(), String> {
    open_path(&project_id)
}

#[tauri::command]
pub fn open_session_logs(project_id: String) -> Result<(), String> {
    let path = claude_project_log_dir(&project_id)?;
    open_path(&path.to_string_lossy())
}

#[tauri::command]
pub fn copy_path(project_id: String) -> String {
    project_id
}

#[tauri::command]
pub fn pause_monitoring() {}

#[tauri::command]
pub fn resume_monitoring() {}

#[tauri::command]
pub fn open_settings() {}

#[tauri::command]
pub fn check_hooks() -> bool {
    check_hooks_installed()
}

#[tauri::command]
pub fn install_hooks_command() -> Result<(), String> {
    install_hooks().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn preview_hook_config_command() -> Result<String, String> {
    preview_hook_config()
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

fn open_path(path: &str) -> Result<(), String> {
    let mut command = platform_open_command(path)?;

    command.spawn().map_err(|error| error.to_string())?;
    Ok(())
}

fn claude_project_log_dir(project_id: &str) -> Result<PathBuf, String> {
    let home = home_dir().ok_or_else(|| "failed to resolve home directory".to_string())?;
    Ok(home
        .join(".claude")
        .join("projects")
        .join(encode_claude_project_dir(project_id)))
}

fn encode_claude_project_dir(project_id: &str) -> String {
    project_id
        .replace("\\\\?\\", "")
        .replace(':', "")
        .replace(['\\', '/'], "-")
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

fn platform_open_command(path: &str) -> Result<std::process::Command, String> {
    #[cfg(target_os = "windows")]
    {
        let mut command = std::process::Command::new("explorer");
        command.arg(path);
        return Ok(command);
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = std::process::Command::new("open");
        command.arg(path);
        return Ok(command);
    }

    #[cfg(target_os = "linux")]
    {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(path);
        return Ok(command);
    }

    #[allow(unreachable_code)]
    Err("opening paths is not supported on this platform".to_string())
}

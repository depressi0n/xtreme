use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;
#[derive(Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    name: String,
    description: String,
    command: String,
    file_path: String,
}
pub(crate) fn get_plugin_dir(app_handle: &AppHandle) -> PathBuf {
    let mut plugins_dir = app_handle.path().app_config_dir().unwrap();
    plugins_dir.push("plugins");
    plugins_dir
}

// 假设插件都存放在一个固定的文件夹里
#[tauri::command]
pub async fn load_plugins(app_handle: tauri::AppHandle) -> Result<Vec<PluginInfo>, String> {
    let plugins_dir = get_plugin_dir(&app_handle);

    println!("plugin path dir: {:?}", plugins_dir);
    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;
    }

    let mut plugins = Vec::new();
    // 遍历插件目录下的所有文件
    for entry in fs::read_dir(plugins_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let plugin: PluginInfo = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            plugins.push(plugin);
        }
    }
    Ok(plugins)
}

#[tauri::command]
pub async fn run_plugin_command(
    app_handle: tauri::AppHandle,
    command_name: String,
) -> Result<String, String> {
    // 实际的执行逻辑会更复杂，这里我们简化处理
    // 可以使用 `tauri_plugin_shell` 来执行外部脚本
    // 使用 `quickjs-rs` 等库来在 Rust 中执行 JS
    let plugins_dir = get_plugin_dir(&app_handle);
    let plugin_file_path = plugins_dir.join(format!("{}.js", command_name));

    if !plugin_file_path.exists() {
        return Err(format!("Plugin script not found: {}", command_name));
    }

    let plugin_code = fs::read_to_string(&plugin_file_path)
        .map_err(|e| format!("Failed to read plugin script: {}", e))?;

    // QuickJS 沙盒执行环境
    let runtime = rquickjs::Runtime::new().map_err(|e| e.to_string())?;
    let context = rquickjs::Context::full(&runtime).map_err(|e| e.to_string())?;

    let app_handle_clone = app_handle.clone();
    context.with(|scope| {
        // 在这里创建并暴露一个有限的 API 给插件
        // 例如，一个用于打印日志的函数
        let console_log = rquickjs::Function::new(scope.clone(), |msg: String| {
            println!("[Plugin Log]: {}", msg);
        });
        scope.globals().set("console_log", console_log).unwrap();

        // 一个用于打开 URL 的函数，但要经过后端校验
        let open_url = rquickjs::Function::new(scope.clone(), move |url: String| {
            println!("{:?}", url);

            let app_handle_clone_inner = app_handle_clone.clone(); // 再次克隆用于 spawn
            tauri::async_runtime::spawn(async move {
                if url.starts_with("https://") || url.starts_with("http://") {
                    app_handle_clone_inner
                        .opener()
                        .open_url(&url, None::<&str>)
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to open URL: {}", e);
                        });
                } else {
                    eprintln!("Only http/https URLs are allowed");
                }
            });
            // open_url 函数立即返回，不阻塞
        })
        .unwrap();
        scope.globals().set("open_url", open_url).unwrap();

        // 执行插件代码
        let result: Result<String, rquickjs::Error> = scope.eval(plugin_code);
        match result {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("Plugin execution failed: {}", e)),
        }
    })
}

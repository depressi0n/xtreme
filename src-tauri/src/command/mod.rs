use tauri_plugin_opener::OpenerExt;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn open_url(app_handle: tauri::AppHandle, command: String) -> Result<String, String> {
    let opener = app_handle.opener();

    // 去除命令前缀，并分割命令和参数
    let parts: Vec<&str> = command.trim_start_matches('>').splitn(2, ' ').collect();
    let cmd = parts[0];
    let args = if parts.len() > 1 {
        Some(parts[1])
    } else {
        None
    };

    match cmd {
        "google" => {
            let url = "https://www.google.com".to_string();
            if opener.open_url(&url, None::<&str>).is_ok() {
                Ok(format!("Opened {}", url))
            } else {
                Err("Failed to open URL".to_string())
            }
        }
        "wiki" => {
            if let Some(term) = args {
                let url = format!("https://en.wikipedia.org/wiki/{}", term);
                if opener.open_url(&url, None::<&str>).is_ok() {
                    Ok(format!("Opened {}", url))
                } else {
                    Err("Failed to open URL".to_string())
                }
            } else {
                Err("Missing search term for wiki command".to_string())
            }
        }
        // 在这里添加更多自定义命令
        _ => Err(format!("Unknown command: {}", cmd)),
    }
}

#[tauri::command]
pub async fn execute_shell(
    app_handle: tauri::AppHandle,
    _command: String,
) -> Result<String, String> {
    let shell = app_handle.shell();
    let output = tauri::async_runtime::block_on(async move {
        shell
            .command("echo")
            .args(["Hello from Rust!"])
            .output()
            .await
            .unwrap()
    });
    if output.status.success() {
        println!("Result: {:?}", String::from_utf8(output.stdout));
    } else {
        println!("Exit with code: {}", output.status.code().unwrap());
    }
    Ok("success".to_string())
}

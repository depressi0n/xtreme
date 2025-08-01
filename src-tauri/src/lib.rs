use tauri::Manager;

mod command;
mod shortcut;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // [macOS]在Dock栏隐藏
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                app.set_activation_policy(ActivationPolicy::Accessory);
            }

            window::init(app)?;
            shortcut::init(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::execute_shell,
            command::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

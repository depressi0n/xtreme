use tauri::{Emitter, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
pub fn init(app: &tauri::App) -> Result<(), tauri::Error> {
    // 获取主窗口实例
    let main_window = app.get_webview_window("main").unwrap();

    // 注册全局快捷键 Option + Space
    app.global_shortcut()
        .on_shortcut("Alt+Space", move |_app_handle, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if main_window.is_visible().unwrap() {
                    main_window.hide().unwrap();
                } else {
                    main_window.show().unwrap();
                    if !main_window.is_always_on_top().unwrap(){
                        main_window.set_always_on_top(true).unwrap();
                    }
                    main_window.set_focus().unwrap();
                }
            }
        })
        .unwrap();
    // TODO 窗口快捷键 ESC

    Ok(())
}

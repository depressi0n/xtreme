use crate::window;
use tauri::{
    window::{Effect, EffectState, EffectsBuilder},
    LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};

pub fn init(app: &tauri::App) -> Result<(), tauri::Error> {
    let effects = EffectsBuilder::new()
        .effects(vec![
            Effect::HudWindow, // macos
            Effect::Acrylic,   // win10/11
            Effect::Blur,      // win7
        ])
        .radius(20.) // 圆角
        .state(EffectState::Active)
        .build();

    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .decorations(false) // 无边框、标题栏
        .inner_size(800f64, 600f64) // 窗口大小
        .resizable(false) // 不可调整
        .shadow(false) // 阴影
        .center() // 屏幕居中
        .visible(false) // 默认隐藏，通过快捷键显示
        .skip_taskbar(true)
        .always_on_top(true) // 默认置顶
        .transparent(true) // 透明背景
        .effects(effects) // 磨砂
        .build()?;

    // 监听窗口事件
    let window_clone = window.clone();
    window.on_window_event(move |event| match event {
        // 失去焦点（blur），自动隐藏
        WindowEvent::Focused(false) => {
            if window_clone.is_visible().unwrap() {
                window_clone.hide().unwrap();
            }
        }
        _ => {}
    });

    Ok(())
}

mod application;
pub mod assets;
pub mod bridge_security_platform;
pub(crate) mod components;
pub mod i18n;
mod shell;
mod system_tray;
pub(crate) mod theme;
pub(crate) mod utils;
mod windowing;

pub use application::{initialize_application_state, reload_application_state};
pub use shell::{AppView, AppWindowRole, open_settings_from_menu};
pub use system_tray::{
    configure_detached_window_close, configure_main_window_close, initialize_system_tray,
    quit_application, request_main_window_close, restore_main_window, sync_system_tray,
};
pub use windowing::{DetachedWindowTarget, register_detached_window_opener};

pub fn init_markdown(_cx: &mut gpui_kit::App) {
    // Initialize language registry for tree-sitter based code block syntax highlighting
    // Languages are enabled via Cargo features: tree-sitter-rust, tree-sitter-python, etc.
    use gpui_kit::component::highlighter::LanguageRegistry;
    let _ = LanguageRegistry::singleton();
}

/// Extra key bindings for the shared text-input context so that Alt+arrows and
/// Alt+Backspace/Delete move and remove whole words on Windows and Linux, like
/// Option does on macOS. Ctrl is already bound by gpui-kit on non-macOS.
#[cfg(not(target_os = "macos"))]
pub fn init_input_key_bindings(cx: &mut gpui_kit::App) {
    use gpui_kit::KeyBinding;
    use gpui_kit::component::input::{
        DeleteToNextWordEnd, DeleteToPreviousWordStart, MoveToNextWord, MoveToPreviousWord,
        SelectToNextWordEnd, SelectToPreviousWordStart,
    };

    cx.bind_keys([
        KeyBinding::new("alt-left", MoveToPreviousWord, Some("Input")),
        KeyBinding::new("alt-right", MoveToNextWord, Some("Input")),
        KeyBinding::new("alt-backspace", DeleteToPreviousWordStart, Some("Input")),
        KeyBinding::new("alt-delete", DeleteToNextWordEnd, Some("Input")),
        KeyBinding::new("alt-shift-left", SelectToPreviousWordStart, Some("Input")),
        KeyBinding::new("alt-shift-right", SelectToNextWordEnd, Some("Input")),
    ]);
}

#[cfg(target_os = "macos")]
pub fn init_input_key_bindings(_cx: &mut gpui_kit::App) {}

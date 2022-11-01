use tauri::AppHandle;


#[default_runtime(crate::Wry, wry)]
#[derive(Debug)]
pub struct AppHandle<R: Runtime> {
  runtime_handle: R::Handle,
  pub(crate) manager: WindowManager<R>,
  #[cfg(all(desktop, feature = "global-shortcut"))]
  global_shortcut_manager: R::GlobalShortcutManager,
  #[cfg(feature = "clipboard")]
  clipboard_manager: R::ClipboardManager,
  /// The updater configuration.
  #[cfg(updater)]
  pub(crate) updater_settings: UpdaterSettings,
}
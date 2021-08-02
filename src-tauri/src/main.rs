#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::WindowBuilder;
use tauri::WindowUrl;
use tauri::Menu;
use tauri::MenuItem;

fn main() {
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)    
    .add_native_item(MenuItem::Cut)    
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::Quit);

    tauri::Builder::default()
    .setup(|app|{
        app.create_window("emailit".to_string(), WindowUrl::default(), |WinAttrs, WebviewAttrs| {
          let WinAttrs = WinAttrs
        .title("EmailIt")
        .resizable(false)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(1000.0, 600.0)
        .min_inner_size(1000.0, 600.0)
        .fullscreen(false);
        (WinAttrs, WebviewAttrs)
      }).unwrap();
      Ok(())
    })
    .menu(menu);
}

use tauri::WindowBuilder;
use tauri::WindowUrl;
use tauri::Menu;
use tauri::MenuItem;
use tauri::Submenu;

fn main() {
  let main_submenu = Submenu::new(
    "EmailIt", 
    Menu::new()
      .add_native_item(MenuItem::About("EmailIt".to_string()))
      .add_native_item(MenuItem::Services)
      .add_native_item(MenuItem::Quit)
  );

  let file_submenu = Submenu::new(
    "File",
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Quit)
  );

  let edit_submenu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Paste)
      .add_native_item(MenuItem::SelectAll)
  );

  let help_submenu = Submenu::new(
    "Help",
    Menu::new()
  );

  let menu = Menu::new()
    .add_submenu(main_submenu)
    .add_submenu(file_submenu)
    .add_submenu(edit_submenu)
    .add_submenu(help_submenu);

  tauri::Builder::default()
    .setup(|app|{
      app.create_window("emailit".to_string(), WindowUrl::default(), |win_attrs, webview_attrs| {
        let win_attrs = win_attrs
          .title("EmailIt")
          .resizable(false)
          .transparent(false)
          .decorations(true)
          .always_on_top(false)
          .inner_size(1000.0, 600.0)
          .min_inner_size(1000.0, 600.0)
          .visible(false)
          .fullscreen(false);
          (win_attrs, webview_attrs)
      }).unwrap();
      Ok(())
    })
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

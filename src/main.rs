slint::include_modules!();
use rfd::FileDialog;

fn main() {
    let app = AppWindow::new().unwrap();

    let ui_handle = app.as_weak();
    app.on_select_path(move || {
        let ui = ui_handle.unwrap();
        let select_dir = FileDialog::new().pick_folder();
        let mut current_path = String::new();
        if let Some(select_dir) = select_dir {
            current_path = select_dir.as_path().display().to_string();
        }
        ui.set_path(current_path.into());
    });

    app.run().unwrap();
}

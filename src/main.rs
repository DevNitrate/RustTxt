slint::include_modules!();
use std::fs::write;
use std::fs;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_save({
        move |content, filename| {

            let contents = content;
            let f = String::from(filename);

            if f != "" {
                write(f, contents).expect("Could not save file");
            }
        }
    });

    ui.on_load({
        let ui_handle = ui.as_weak();
        move |filename| {
            let ui = ui_handle.unwrap();
            let file = String::from(filename);

            let content = fs::read_to_string(file).expect("Could not read file");
            ui.set_data(content.into());
        }
    });

    ui.run()
}

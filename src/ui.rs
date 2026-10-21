use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, Image};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;

pub fn run_ui(is_talking: Arc<AtomicBool>) {
    let app = Application::builder()
        .application_id("com.popcat.tuber")
        .build();

    app.connect_activate(move |app| {
        build_ui(app, Arc::clone(&is_talking));
    });

    app.run_with_args(&[""]);
}

fn build_ui(app: &Application, is_talking: Arc<AtomicBool>) {
    let image = Image::from_file("assets/popcat-close.png");
    image.set_pixel_size(400);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PopCat PNGTuber")
        .default_width(450)
        .default_height(450)
        .child(&image)
        .build();

    let provider = CssProvider::new();
    provider.load_from_data("window { background-color: #00FF00; }");

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let mut last_state = false;
    glib::timeout_add_local(Duration::from_millis(16), move || {
        let current_state = is_talking.load(Ordering::Relaxed);

        if current_state != last_state {
            if current_state {
                image.set_from_file(Some("assets/popcat-open.png"));
            } else {
                image.set_from_file(Some("assets/popcat-close.png"));
            }
            last_state = current_state;
        }

        glib::ControlFlow::Continue
    });

    window.present();
}

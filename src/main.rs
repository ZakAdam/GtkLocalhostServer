use warp::Filter;
use std::path::PathBuf;
use std::thread;
use async_channel::*;
use gtk::{glib};
use gtk::prelude::*;
use gtk::{gio, Application, ApplicationWindow, Button, FileDialog, Builder};

mod window;
use window::Window;

const APP_ID: &str = "Localhost Server";

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    gio::resources_register_include!("resources.gresource").expect("Failed to register resources");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    window.present();
}

fn run_server(path: String, sender: Sender<String>) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            //let static_assets = warp::fs::dir(PathBuf::from(path.clone()));
            println!("{:?}", path);

            let clone_sender = sender.clone();
            let _ = clone_sender.try_send(format!("Server started on {}", path));
            let _ = sender.try_send(format!("SENDER !! Server started on {}", path));

            let log_request = warp::log::custom(move |info| {
                let _ = sender.try_send(format!(
                    "[{} {}] {} - {:?} bytes",
                    info.method(),
                    info.path(),
                    info.status(),
                    info.request_headers()
                        .get("content-length")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("-"),
                ));
            });

            let static_assets = warp::fs::dir(PathBuf::from(path.clone()))
                .with(log_request);

            warp::serve(static_assets).run(([127, 0, 0, 1], 3030)).await;
        });
    });
}


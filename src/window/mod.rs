mod imp;

use glib::Object;
use gtk::{gio, glib, Application, Button, FileDialog};
use async_channel::*;
use gtk::prelude::{ButtonExt, FileExt, TextBufferExt};
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use crate::run_server;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application ) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn set_channels(&self, sender: Sender<String>, receiver: Receiver<String>) {
        self.imp().sender.replace(Some(sender));
        self.imp().receiver.replace(Some(receiver));
    }

    pub fn open_file_dialog(&self) {
        let dialog = FileDialog::builder()
            .title("Open directory")
            .accept_label("OpenLOL")
            .build();

        let sender_clone = self.imp().sender.borrow().clone().unwrap();

        dialog.select_folder(Some(self), gio::Cancellable::NONE, move |file| {
            println!("{:?}", file);
            if let Ok(file) = file {
                println!("{:?}", file.path());

                let folder_path = file.path().unwrap().to_str().unwrap().to_string();
                //let static_assets = warp::path("/").and(warp::fs::dir(path.as_path()));
                //warp::serve(static_assets).run(([127, 0, 0, 1], 3030)).await;

                run_server(folder_path, sender_clone);
            }
        })
    }
}
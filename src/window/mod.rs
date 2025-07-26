mod imp;

use crate::run_server;
use adw::gio::ActionEntry;
use async_channel::*;
use glib::Object;
use gtk::glib::clone;
use gtk::prelude::{
    ActionMapExtManual, BoxExt, ButtonExt, EntryExt, FileExt, GtkWindowExt, TextBufferExt,
};
use gtk::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use gtk::{
    Align, Application, ApplicationWindow, Button, Entry, FileDialog, FontButton, FontDialog,
    FontDialogButton, Label, Orientation, SpinButton, gio, glib,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn set_channels(&self, sender: Sender<String>, receiver: Receiver<String>) {
        self.imp().sender.replace(Some(sender));
        self.imp().receiver.replace(Some(receiver));
    }

    pub fn setup_actions(&self, app: &adw::Application) {
        let open_preferences_action = ActionEntry::builder("open-preferences")
            .activate(clone!(
                #[weak]
                app,
                move |_, _, _| {
                    println!("Open preferences");

                    let font_dialog = FontDialog::builder().modal(true).build();
                    let entry = FontDialogButton::builder()
                        .dialog(&font_dialog)
                        .name("font-button")
                        .build();
                    let label = Label::builder().label("Font").build();

                    let gtk_box = gtk::Box::builder()
                        .orientation(Orientation::Horizontal)
                        .margin_top(12)
                        .margin_bottom(12)
                        .margin_start(12)
                        .margin_end(12)
                        .spacing(12)
                        .halign(Align::Center)
                        .build();

                    gtk_box.append(&label);
                    gtk_box.append(&entry);

                    let window = ApplicationWindow::builder()
                        .application(&app)
                        .title("Preferences")
                        .child(&gtk_box)
                        .build();

                    window.present();
                }
            ))
            .build();

        self.add_action_entries([open_preferences_action]);
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

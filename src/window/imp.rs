use std::cell::RefCell;
use async_channel::{Receiver, Sender};
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, TextView, CompositeTemplate, FileDialog, gio};
use gtk::glib::clone;

#[derive(CompositeTemplate, Default)]
#[template(resource="/org/gtk-localhost-server/window.ui")]
pub struct Window {
    #[template_child]
    pub folder_button: TemplateChild<Button>,
    #[template_child]
    pub logs_view: TemplateChild<TextView>,

    //pub sender: RefCell<Option<Sender<String>>>,
    pub sender: RefCell<Option<Sender<String>>>,
    pub receiver: RefCell<Option<Receiver<String>>>
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Localhost_Server";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        
        let (sender, receiver) = async_channel::unbounded();

        self.obj().set_channels(sender.clone(), receiver.clone());

        self.folder_button.connect_clicked(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.obj().open_file_dialog();
        }));

        let buffer = self.logs_view.buffer();//clone();

        buffer.set_text("Text to show initialization of TextView\n");

        glib::spawn_future_local(clone!(
            async move {
                println!("{:?}", receiver.recv().await);
                while let Ok(log) = receiver.recv().await {
                    println!("{:?}", log);
                    buffer.insert(&mut buffer.end_iter(), &(log + "\n"));
                }
            }
        ));
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
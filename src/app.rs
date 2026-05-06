use std::convert::identity;

use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::device_controls_view::{DeviceControlsInit, DeviceControlsView};
use crate::shortcuts::register_shortcuts;
use crate::widget_ext::*;

pub struct App;

pub struct AppWidgets;

impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = adw::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder()
            .title("Alsamixer")
            .application(&relm4::main_adw_application())
            .default_width(600)
            .default_height(400)
            .build()
    }

    fn init(
        _args: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        register_shortcuts(&root);

        root.set_content(Some(
            &gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build()
                .child(&adw::HeaderBar::builder().build())
                .child(
                    DeviceControlsView::builder()
                        .launch(DeviceControlsInit {
                            devices: vec![
                                String::from("Test"),
                                String::from("Test 2"),
                                String::from("Test 3"),
                            ],
                        })
                        .forward(sender.input_sender(), identity)
                        .widget(),
                ),
        ));

        ComponentParts {
            model: Self {},
            widgets: AppWidgets {},
        }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) -> Self::Output {}
}

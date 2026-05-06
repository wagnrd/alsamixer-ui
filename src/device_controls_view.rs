use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::widget_ext::{ApplyExt, BoxChildExt, SidebarChildExt};

pub struct DeviceControlsView {
    devices: Vec<String>,
}

pub struct DeviceControlsInit {
    pub devices: Vec<String>,
}

pub struct DeviceControlsWidgets {}

#[derive(Debug)]
pub enum DeviceControlsMsg {
    OnDeviceClick(String),
}

impl SimpleComponent for DeviceControlsView {
    type Root = adw::NavigationSplitView;
    type Init = DeviceControlsInit;
    type Input = DeviceControlsMsg;
    type Output = ();
    type Widgets = DeviceControlsWidgets;

    fn init_root() -> Self::Root {
        adw::NavigationSplitView::builder().vexpand(true).build()
    }

    fn init(
        args: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        root.set_sidebar(Some(&adw::NavigationPage::new(
            &adw::Sidebar::new().child(adw::SidebarSection::new().apply(|section| {
                args.devices.iter().for_each(|device| {
                    section.append(adw::SidebarItem::builder().title(device).build());
                });
            })),
            "Sidebar",
        )));

        ComponentParts {
            model: Self {
                devices: args.devices,
            },
            widgets: DeviceControlsWidgets {},
        }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) -> Self::Output {}

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

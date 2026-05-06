use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;

/// Generic chainable method for any widget.
/// Useful for calling setters that return `()` without breaking the chain.
pub trait ApplyExt: Sized {
    fn apply(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
}

impl<T: IsA<glib::Object>> ApplyExt for T {}

pub trait BoxChildExt {
    fn child(self, child: &impl IsA<gtk::Widget>) -> Self;
}

impl BoxChildExt for gtk::Box {
    fn child(self, widget: &impl IsA<gtk::Widget>) -> Self {
        self.append(widget);
        self
    }
}

pub trait ListBoxChildExt {
    fn child(self, child: &impl IsA<gtk::Widget>) -> Self;
}

impl ListBoxChildExt for gtk::ListBox {
    fn child(self, widget: &impl IsA<gtk::Widget>) -> Self {
        self.append(widget);
        self
    }
}

pub trait SidebarChildExt {
    fn child(self, section: adw::SidebarSection) -> Self;
}

impl SidebarChildExt for adw::Sidebar {
    fn child(self, section: adw::SidebarSection) -> Self {
        self.append(section);
        self
    }
}

pub trait SidebarSectionChildExt {
    fn child(self, child: impl IsA<adw::SidebarItem>) -> Self;
}

impl SidebarSectionChildExt for adw::SidebarSection {
    fn child(self, widget: impl IsA<adw::SidebarItem>) -> Self {
        self.append(widget);
        self
    }
}

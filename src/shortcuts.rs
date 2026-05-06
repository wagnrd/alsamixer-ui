use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::adw::{self, prelude::*};

pub fn register_shortcuts(root: &adw::Window) {
    register_macos_quit(root);
}

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn register_macos_quit(root: &adw::Window) {
    let mut app_actions = RelmActionGroup::<AppActionGroup>::new();
    let quit_action =
        RelmAction::<QuitAction>::new_stateless(move |_| relm4::main_adw_application().quit());
    app_actions.add_action(quit_action);
    app_actions.register_for_widget(&root);
}

use adw::prelude::*;
use relm4::{Component, ComponentParts, adw};

use crate::{APP_ID, APP_NAME, GITHUB_URL, VERSION};

pub struct AboutDialog;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AboutDialogMsg {
    Show(adw::ApplicationWindow),
}

#[relm4::component(pub)]
impl Component for AboutDialog {
    type Init = ();
    type Input = AboutDialogMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        dialog = adw::AboutDialog {
            set_application_name: APP_NAME,
            set_application_icon: APP_ID,
            set_developer_name: "Frank Ploss",
            set_version: VERSION,
            set_license_type: gtk::License::Gpl30,
            set_copyright: "© 2025 Frank Ploss",
            set_developers: &vec!["Frank Ploss"],
            set_issue_url: &format!("{}/issues", GITHUB_URL),
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AboutDialogMsg::Show(parent) => {
                widgets.dialog.present(Some(&parent));
            }
        }

        self.update_view(widgets, sender);
    }
}

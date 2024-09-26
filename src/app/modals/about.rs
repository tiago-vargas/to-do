use relm4::prelude::*;

use crate::config;

pub(crate) struct Model;

pub(crate) struct Init;

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	view! {
		adw::AboutDialog {
			set_application_icon: config::APP_ID,
			set_application_name: "To-Do",
			set_developer_name: "Tiago Vargas",
			set_version: config::VERSION,

			set_website: "https://tiago-vargas.github.io/to-do",

			set_issue_url: "https://github.com/tiago-vargas/to-do/issues",

			set_developers: &["Tiago Vargas <tiago.vargaspo@proton.me>"],
			set_copyright: "© 2024 Tiago Vargas",
			set_license_type: gtk::License::MitX11,
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self;
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}
}

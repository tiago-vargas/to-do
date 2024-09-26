use adw::prelude::*;
use relm4::prelude::*;

use crate::config::{BUILD_TYPE, RESOURCE_PREFIX};

mod actions;
mod modals;
mod settings;

pub(crate) struct MainWindow;

#[relm4::component(pub(crate))]
impl SimpleComponent for MainWindow {
	type Init = ();
	type Input = ();
	type Output = ();

	menu! {
		primary_menu: {
			section! {
				"Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
				"About To-Do" => actions::ShowAbout,
			},
		}
	}

	view! {
		main_window = adw::ApplicationWindow {
			set_title: Some("To-Do"),

			add_css_class?: if BUILD_TYPE == "debug" { Some("devel") } else { None },

			#[wrap(Some)]
			set_help_overlay: keyboard_shortcuts_window =
				&gtk::Builder::from_resource(
					&format!("{RESOURCE_PREFIX}/app/modals/keyboard-shortcuts-window.ui")
				)
				.object::<gtk::ShortcutsWindow>("help_overlay")
				.unwrap() -> gtk::ShortcutsWindow,

			adw::ToolbarView {
				add_top_bar = &adw::HeaderBar {
					pack_end = &gtk::MenuButton {
						set_icon_name: "open-menu-symbolic",
						set_menu_model: Some(&primary_menu),
						set_tooltip_text: Some("Main Menu"),
					},
				},

				gtk::Label {
					set_label: "Hello, World!",
					set_css_classes: &["title-1"],
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		_root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self;

		let widgets = view_output!();

		Self::load_window_state(&widgets);
		Self::create_actions(&widgets, &sender);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}

	fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
		Self::save_window_state(widgets);
	}
}

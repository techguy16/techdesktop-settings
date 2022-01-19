// SPDX-License-Identifier: GPL-3.0-only

pub mod nav;
pub mod popup;
pub mod search;
pub mod section;

pub use self::{nav::SettingsNavGui, popup::PopupGui, search::SearchGui};
use crate::widgets::ListBoxSelectionRow;
use gtk4::{
	glib::{self, clone},
	prelude::*,
	ApplicationWindow, HeaderBar, Orientation, Stack,
};

/// A struct containing references to various base elements of the COSMIC settings GUI.
#[derive(Clone)]
pub struct SettingsGui {
	/// The title bar of the application
	pub header: HeaderBar,
	/// The box containing the header contents
	pub header_box: gtk4::Box,
	/// The search bar of the application
	pub search: SearchGui,
	/// The base box that contains everything except the header.
	pub base: gtk4::Box,
	/// Various elements related to navigation.
	pub nav: SettingsNavGui,
	/// Various elements related to popup UIs
	pub popup: PopupGui,
	/// The stack that contains the pages of primary content.
	pub content: Stack,
}

impl SettingsGui {
	pub fn new(window: &ApplicationWindow) -> Self {
		let header = Self::create_header();
		let header_box = Self::create_header_box();
		let search = SearchGui::new();
		let base = Self::create_base_box();
		let nav = SettingsNavGui::new(&header_box);
		header_box.append(&search.bar);
		header.pack_start(&header_box);
		let content = Stack::new();
		Self::setup_row_active(&nav, &content);
		base.append(&nav.revealer);
		base.append(&content);
		let popup = PopupGui::new(&base);
		window.set_child(Some(&popup.overlay));
		window.set_titlebar(Some(&header));
		Self {
			header,
			header_box,
			search,
			base,
			nav,
			popup,
			content,
		}
	}

	fn create_header() -> HeaderBar {
		HeaderBar::builder()
			.css_classes(vec!["titlebar".into()])
			.build()
	}
	fn create_base_box() -> gtk4::Box {
		gtk4::Box::builder()
			.orientation(Orientation::Horizontal)
			.margin_top(16)
			.margin_bottom(0)
			.margin_start(16)
			.margin_end(16)
			.build()
	}

	fn create_header_box() -> gtk4::Box {
		gtk4::Box::builder()
			.orientation(Orientation::Horizontal)
			.build()
	}

	fn setup_row_active(nav: &SettingsNavGui, content: &Stack) {
		nav.list.connect_row_activated(
			clone!(@weak content, @weak nav.labels as labels, @weak nav.stack as nav_stack, @weak nav.subsection_revealer as nav_stack_revealer => move |_, row| {
				let row = row
					.downcast_ref::<ListBoxSelectionRow>()
					.expect("invalid object");
				if row.subsection() {
					nav_stack_revealer.set_reveal_child(true);
					nav_stack.set_visible_child_name(&row.row_id());
					for label in labels.borrow().iter() {
						label.hide();
					}
				} else {
					nav_stack_revealer.set_reveal_child(false);
					content.set_visible_child_name(&row.row_id());
					for label in labels.borrow().iter() {
						label.show();
					}
				}
			}),
		);
	}
}

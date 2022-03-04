// SPDX-License-Identifier: GPL-3.0-only

use crate::{
	sections::{Section, SectionLayout, SettingsGroup, SettingsGroupStore},
	ui::SettingsGui,
	widgets::ListBoxSelectionRow,
};
use gtk4::{
	glib::{self, clone},
	prelude::*,
	Align, Image, Label, ListBox, Orientation, PolicyType, ScrolledWindow,
};
use std::rc::Rc;

pub fn setup<S: Section>(ui: Rc<SettingsGui>, sections_store: SettingsGroupStore) {
	view! {
		row = ListBoxSelectionRow::new(S::NAME.into()) {
			add_css_class: "nav-element",
			set_margin_top: 8,
			set_margin_bottom: 8,
			set_margin_start: 8,
			set_margin_end: 8,
			set_child: entry_box = Some(&gtk4::Box) {
				set_orientation: Orientation::Horizontal,
				set_spacing: 8,
				set_margin_start: 10,
				set_margin_end: 10,
				set_margin_top: 10,
				set_margin_bottom: 10,
				append: icon = &Image {
					set_icon_name: Some(S::ICON)
				},
				append: label = &Label {
					set_text: S::NAME
				},
			}
		}
	}
	ui.nav.list.append(&row);
	ui.nav.labels.borrow_mut().push(label);

	let entries = S::layout();
	match entries {
		SectionLayout::Single(groups) => {
			// Alright, now we setup the actual settings panel
			view! {
				scroll_window = ScrolledWindow {
					set_hscrollbar_policy: PolicyType::Never,
					set_vscrollbar_policy: PolicyType::Automatic,
					set_child: panel = Some(&gtk4::Box) {
						set_orientation: Orientation::Vertical,
						set_spacing: 24,
						set_hexpand: true
					}
				}
			}
			setup_single(&panel, ui.clone(), groups, sections_store);
			ui.content
				.add_titled(&scroll_window, Some(S::NAME), S::NAME);
		}
		SectionLayout::Multiple(subsections) => {
			setup_multi(S::NAME, ui, subsections, sections_store);
			row.set_subsection(true);
		}
	}
}

fn setup_single(
	panel: &gtk4::Box,
	ui: Rc<SettingsGui>,
	groups: Vec<Box<dyn SettingsGroup>>,
	sections_store: SettingsGroupStore,
) {
	for group in groups {
		let title = group.title();
		view! {
			group_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				set_spacing: 8,
				append: group_box_inner = &gtk4::Box {
					add_css_class: "settings-group",
					set_orientation: Orientation::Vertical,
					set_spacing: 16,
					append: group_title = &Label {
						add_css_class: "settings-group-title",
						set_label: title,
						set_halign: Align::Start,
						set_visible: !title.is_empty()
					}
				}
			}
		}
		group.layout(&group_box_inner, ui.clone());
		panel.append(&group_box);
		sections_store.borrow_mut().push(group);
	}
}

fn setup_multi(
	name: &str,
	ui: Rc<SettingsGui>,
	sections: Vec<(&'static str, Vec<Box<dyn SettingsGroup>>)>,
	sections_store: SettingsGroupStore,
) {
	view! {
		nav = ListBox::new() {
			add_css_class: "nav-subsection",
			set_margin_top: 20,
			set_margin_bottom: 20,
			set_margin_start: 10,
			set_margin_end: 10,
		}
	}
	for (name, groups) in sections {
		view! {
			row = ListBoxSelectionRow::new(name.into()) {
				add_css_class: "nav-element",
				set_margin_top: 8,
				set_margin_bottom: 8,
				set_child: label = Some(&Label) {
					set_text: name,
					set_margin_top: 5,
					set_margin_bottom: 5,
					set_margin_end: 8,
					set_halign: Align::Start
				}
			}
		}
		nav.append(&row);
		// Set up the actual groups
		view! {
			scroll_window = ScrolledWindow {
				set_hscrollbar_policy: PolicyType::Never,
				set_vscrollbar_policy: PolicyType::Automatic,
				set_child: panel = Some(&gtk4::Box) {
					set_orientation: Orientation::Vertical,
					set_spacing: 24,
					set_hexpand: true
				}
			}
		}
		setup_single(&panel, ui.clone(), groups, sections_store.clone());
		ui.content.add_named(&scroll_window, Some(name));
	}
	let main_stack = &ui.content;
	nav.connect_row_activated(clone!(@weak main_stack, => move |_, row| {
		let row = row
			.downcast_ref::<ListBoxSelectionRow>()
			.expect("invalid object");
		main_stack.set_visible_child_name(&row.row_id());
	}));
	ui.nav.stack.add_named(&nav, Some(name));
}

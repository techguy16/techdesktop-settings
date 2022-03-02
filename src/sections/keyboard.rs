// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::ui::SettingsGui;
use gtk4::{prelude::*, Align, CheckButton, Label, Orientation};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use std::rc::Rc;

pub struct KeyboardSection;

impl Section for KeyboardSection {
	const NAME: &'static str = "Keyboard";
	const ICON: &'static str = "preferences-desktop-keyboard-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Multiple(vec![
			(
				"Input",
				vec![
					InputSourceSwitching::boxed(),
					TypeSpecialCharacters::boxed(),
				],
			),
			("Keyboard Shortcuts", vec![]),
		])
	}
}

#[derive(Default)]
struct InputSourceSwitching;

impl SettingsGroup for InputSourceSwitching {
	fn title(&self) -> &'static str {
		"Input Source Switching"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["input", "source", "switch", "shortcut", "keyboard"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: same_source = &LabeledItem {
					set_title: "Use the same source for all windows",
					set_alignment: Align::Start,
					set_child: same_check = &CheckButton {
						set_valign: Align::Center
					}
				},
				container_add: switch_source = &LabeledItem {
					set_title: "Switch input sources individually for each window",
					set_alignment: Align::Start,
					set_child: switch_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&same_check)
					}
				},
				container_add: kb_shortcut = &LabeledItem {
					set_title: "Keyboard Shortcut",
					set_description: "This can be changed in Shortcuts",
					set_alignment: Align::Start,
					set_child: kb_label = &Label {
						set_text: "Super+Space"
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
struct TypeSpecialCharacters;

impl SettingsGroup for TypeSpecialCharacters {
	fn title(&self) -> &'static str {
		"Type Special Characters"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"type",
			"special",
			"characters",
			"unicode",
			"utf",
			"alternate char",
			"compose",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: alt_characters = &LabeledItem {
					set_title: "Alternate Characters Key",
					set_description: "Hold down and type to enter different characters",
					set_child: alt_label = &Label {
						set_text: "Layout Default"
					}
				},
				container_add: compose_characters = &LabeledItem {
					set_title: "Compose Key",
					set_child: compose_label = &Label {
						set_text: "Layout Default"
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

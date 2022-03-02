// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui};
use gtk4::{prelude::*, Align, CheckButton, Orientation, Switch};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use std::rc::Rc;

#[derive(Default)]
pub struct Dock;

impl SettingsGroup for Dock {
	fn title(&self) -> &'static str {
		"Dock"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				container_add: entry = &LabeledItem {
					set_title: "Enable Dock",
					set_child: switch = &Switch {
						set_valign: Align::Center
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
pub struct DockOptions;

impl SettingsGroup for DockOptions {
	fn title(&self) -> &'static str {
		"Dock Options"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"dock",
			"edge",
			"launcher",
			"workspace",
			"application",
			"drive",
			"mount",
			"click",
			"icon",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: extend_entry = &LabeledItem {
					set_title: "Extend dock to the edge of the screen",
					set_child: extend_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: launcher_entry = &LabeledItem {
					set_title: "Show Launcher Icon in Dock",
					set_child: launcher_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: workspaces_entry = &LabeledItem {
					set_title: "Show Workspaces Icon in Dock",
					set_child: workspaces_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: apps_entry = &LabeledItem {
					set_title: "Show Applications Icon in Dock",
					set_child: apps_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: drives_entry = &LabeledItem {
					set_title: "Show Mounted Drives",
					set_child: drives_switch = &Switch {
						set_valign: Align::Center
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
pub struct DockVisibility;

impl SettingsGroup for DockVisibility {
	fn title(&self) -> &'static str {
		"Dock Visibility"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock", "visible", "visibility", "hide"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: always_vis_entry = &LabeledItem {
					set_title: "Always Visible",
					set_alignment: Align::Start,
					set_child: always_vis_check = &CheckButton {
						set_valign: Align::Center
					}
				},
				container_add: medium_entry = &LabeledItem {
					set_title: "Always Hide",
					set_description: "Dock always hides unless actively being revealed by the mouse",
					set_alignment: Align::Start,
					set_child: medium_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&always_vis_check)
					}
				},
				container_add: large_entry = &LabeledItem {
					set_title: "Intelligently Hide",
					set_description: Some("Dock hides when any window overlaps the dock area"),
					set_alignment: Align::Start,
					set_child: large_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&always_vis_check)
					}
				},
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
pub struct DockSize;

impl SettingsGroup for DockSize {
	fn title(&self) -> &'static str {
		"Dock Visibility"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["dock", "size"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let check = CheckButton::builder().valign(Align::Center).build();
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: small_entry = &LabeledItem {
					set_title: "Small (36px)",
					set_alignment: Align::Start,
					set_child: small_check = &CheckButton {
						set_valign: Align::Center
					}
				},
				container_add: medium_entry = &LabeledItem {
					set_title: "Medium (48px)",
					set_alignment: Align::Start,
					set_child: medium_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&small_check)
					}
				},
				container_add: large_entry = &LabeledItem {
					set_title: "Large (60px)",
					set_alignment: Align::Start,
					set_child: large_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&small_check)
					}
				},
			}
		}
		target.append(&entry_box);
	}
}

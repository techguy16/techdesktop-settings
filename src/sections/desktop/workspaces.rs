// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui};
use gtk4::{prelude::*, Align, CheckButton, Orientation, SpinButton};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use std::rc::Rc;

#[derive(Default)]
pub struct Workspaces;

impl SettingsGroup for Workspaces {
	fn title(&self) -> &'static str {
		"Workspaces"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["workspace", "dynamic", "fixed"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: dynamic_workspaces = &LabeledItem {
					set_title: "Dynamic Workspaces",
					set_description: "Automatically removes empty workspaces",
					set_alignment: Align::Start,
					set_child: dynamic_check = &CheckButton {
						set_valign: Align::Center,
					}
				},
				container_add: fixed_workspaces = &LabeledItem {
					set_title: "Fixed Number of Workspaces",
					set_description: "Specify a number of Workspaces",
					set_alignment: Align::Start,
					set_child: fixed_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&dynamic_check)
					}
				},
				container_add: number_of_workspaces = &LabeledItem {
					set_title: "Number of Workspaces",
					set_child: fixed_spin = &SpinButton::with_range(1., 10., 1.) {}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
pub struct MultiMonitorBehavior;

impl SettingsGroup for MultiMonitorBehavior {
	fn title(&self) -> &'static str {
		"Multi-monitor Behavior"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["monitor", "screen", "display"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: workspaces_span = &LabeledItem {
					set_title: "Workspaces Span Displays",
					set_alignment: Align::Start,
					set_child: span_check = &CheckButton {
						set_valign: Align::Center,
					}
				},
				container_add: workspaces_primary = &LabeledItem {
					set_title: "Workspaces on Primary Display Only",
					set_alignment: Align::Start,
					set_child: primary_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&span_check)
					}
				},
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
pub struct PlacementWorkspacePicker;

impl SettingsGroup for PlacementWorkspacePicker {
	fn title(&self) -> &'static str {
		"Placement of the Workspace Picker"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["place", "side", "workspace", "pick", "picker"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: left_side = &LabeledItem {
					set_title: "Along the left side",
					set_alignment: Align::Start,
					set_child: left_side_check = &CheckButton {
						set_valign: Align::Center,
					}
				},
				container_add: right_side_primary = &LabeledItem {
					set_title: "Along the right side",
					set_alignment: Align::Start,
					set_child: right_side_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&left_side_check)
					}
				},
			}
		}
		target.append(&entry_box);
	}
}

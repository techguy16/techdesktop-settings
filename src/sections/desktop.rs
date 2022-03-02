// SPDX-License-Identifier: GPL-3.0-only

mod dock;
mod workspaces;

use super::{Section, SectionLayout, SettingsGroup};
use crate::ui::SettingsGui;
use gtk4::{prelude::*, Align, CheckButton, Orientation, Switch};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use std::rc::Rc;

pub struct DesktopSection;

impl Section for DesktopSection {
	const NAME: &'static str = "Desktop";
	const ICON: &'static str = "user-desktop-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Multiple(vec![
			(
				"Desktop",
				vec![
					SuperKeyAction::boxed(),
					HotCorner::boxed(),
					TopBar::boxed(),
					WindowControls::boxed(),
				],
			),
			(
				"Dock",
				vec![
					dock::Dock::boxed(),
					dock::DockOptions::boxed(),
					dock::DockVisibility::boxed(),
					dock::DockSize::boxed(),
				],
			),
			(
				"Workspaces",
				vec![
					workspaces::Workspaces::boxed(),
					workspaces::MultiMonitorBehavior::boxed(),
					workspaces::PlacementWorkspacePicker::boxed(),
				],
			),
		])
	}
}

#[derive(Default)]
struct SuperKeyAction;

impl SettingsGroup for SuperKeyAction {
	fn title(&self) -> &'static str {
		"Super Key Action"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"super",
			"launcher",
			"window",
			"workspace",
			"overview",
			"app",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: launcher_super = &LabeledItem {
					set_title: "Launcher",
					set_description: "Pressing the Super key opens the Launcher",
					set_child: super_check = &CheckButton {
						set_valign: Align::Center
					}
				},
				container_add: workspaces_super = &LabeledItem {
					set_title: "Workspaces",
					set_description: "Pressing the Super key opens the Window and Workspaces Overview",
					set_child: workspaces_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&super_check)
					}
				},
				container_add: apps_super = &LabeledItem {
					set_title: "Applications",
					set_description: "Pressing the Super key opens the Applications Overview",
					set_child: apps_check = &CheckButton {
						set_valign: Align::Center,
						set_group: Some(&super_check)
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
struct HotCorner;

impl SettingsGroup for HotCorner {
	fn title(&self) -> &'static str {
		"Hot Corner"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["corner", "hot"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry = LabeledItem {
				set_title: "Enable top-left hot corner for Workspaces",
				set_child: switch = &Switch {
					set_valign: Align::Center
				}
			}
		}
		target.container_add(&entry);
	}
}

#[derive(Default)]
struct TopBar;

impl SettingsGroup for TopBar {
	fn title(&self) -> &'static str {
		"Top Bar"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"top bar",
			"menu bar",
			"workspace",
			"application",
			"app",
			"date",
			"time",
			"clock",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: workspaces_button = &LabeledItem {
					set_title: "Show Workspaces Button",
					set_child: workspaces_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: apps_button = &LabeledItem {
					set_title: "Show Applications Button",
					set_child: apps_switch = &Switch {
						set_valign: Align::Center
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

#[derive(Default)]
struct WindowControls;

impl SettingsGroup for WindowControls {
	fn title(&self) -> &'static str {
		"Window Controls"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["minimize", "maximize", "window", "controls"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry_box = gtk4::Box {
				set_orientation: Orientation::Vertical,
				container_add: min_button = &LabeledItem {
					set_title: "Show Minimize Button",
					set_child: min_switch = &Switch {
						set_valign: Align::Center
					}
				},
				container_add: max_button = &LabeledItem {
					set_title: "Show Maximize Button",
					set_child: max_switch = &Switch {
						set_valign: Align::Center
					}
				}
			}
		}
		target.append(&entry_box);
	}
}

// SPDX-License-Identifier: GPL-3.0-only

mod saved_networks;
mod visible_networks;

use super::{Section, SectionLayout, SettingsGroup};
use crate::ui::SettingsGui;
use gtk4::{glib, prelude::*, Align, Button, Label, Orientation, Switch};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use std::rc::Rc;

pub struct WifiSection;

impl Section for WifiSection {
	const NAME: &'static str = "WiFi";
	const ICON: &'static str = "network-wireless-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![
			AirplaneMode::boxed(),
			Wifi::boxed(),
			visible_networks::VisibleNetworks::boxed(),
			AdditionalNetworkSettings::boxed(),
			saved_networks::SavedNetworks::boxed(),
		])
	}
}

#[derive(Default)]
struct AirplaneMode;

impl SettingsGroup for AirplaneMode {
	fn title(&self) -> &'static str {
		"Airplane Mode"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["airplane", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry = SettingsEntry {
				set_title: "Airplane Mode",
				set_description: "Disables Wi-Fi, Bluetooth, and mobile broadband",
				set_child: checkbox = &Switch {
					set_valign: Align::Center
				}
			}
		}
		target.append(&entry);
	}
}

#[derive(Default)]
struct Wifi;

impl SettingsGroup for Wifi {
	fn title(&self) -> &'static str {
		"Wi-Fi"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "wireless", "disable", "turn off"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		view! {
			entry = SettingsEntry {
				set_title: "Wi-Fi",
				set_description: "Disables all Wi-Fi functions",
				set_child: checkbox = &Switch {
					set_valign: Align::Center
				}
			}
		}
		target.append(&entry);
	}
}

#[derive(Default)]
struct AdditionalNetworkSettings;

impl AdditionalNetworkSettings {
	pub fn create_hidden_network_popup() -> gtk4::Box {
		view! {
			base = gtk4::Box {
				set_orientation: Orientation::Vertical,
				append: label = &Label {
					set_markup: "<b>Hidden Network</b>",
					set_halign: Align::Center
				},
			}
		}
		base
	}
}

impl SettingsGroup for AdditionalNetworkSettings {
	fn title(&self) -> &'static str {
		"Additional Network Settings"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"wifi", "wi-fi", "wireless", "hotspot", "hidden", "network", "tether", "hot-spot",
			"hot spot",
		]
	}

	fn layout(&self, target: &gtk4::Box, ui: Rc<SettingsGui>) {
		let button = Button::builder()
			.label("Wi-Fi Hotspot")
			.css_classes(vec!["settings-button".into()])
			.build();
		target.append(&button);
		let button = Button::builder()
			.label("Connect to Hidden Networks")
			.css_classes(vec!["settings-button".into()])
			.build();
		button.connect_clicked(glib::clone!(@strong ui => move |_| {
			ui.popup.pop_up("hidden-net");
		}));
		target.append(&button);
		ui.popup
			.add_overlay("hidden-net", Self::create_hidden_network_popup);
	}
}

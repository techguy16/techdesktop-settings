// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui};
use cosmic_dbus_networkmanager::settings::{connection::Settings, NetworkManagerSettings};
use gtk4::{prelude::*, Button, Image, Label, Orientation};
use std::rc::Rc;
use tokio::sync::mpsc::UnboundedSender;
use zbus::Connection;

#[derive(Default)]
pub struct SavedNetworks;

impl SavedNetworks {
	async fn get_connections(tx: UnboundedSender<Vec<Settings>>) {
		let sys_conn = match Connection::system().await {
			Ok(conn) => conn,
			Err(_) => return, //TODO err msg
		};
		let settings = match NetworkManagerSettings::new(&sys_conn).await {
			Ok(p) => p,
			Err(_) => todo!(), //TODO err msg
		};
		let connections = match settings.list_connections().await {
			Ok(connections) => connections,
			Err(_) => todo!(), //TODO err msg
		};
		let mut out = Vec::with_capacity(connections.len());
		for connection in connections {
			let settings = match connection.get_settings().await.map(Settings::new) {
				Ok(settings) => settings,
				Err(err) => todo!("error: {}", err), //TODO err msg
			};
			out.push(dbg!(settings));
		}
		if let Err(_why) = tx.send(out) {
			eprintln!("{}:{}", file!(), line!());
		}
	}
}

impl SettingsGroup for SavedNetworks {
	fn title(&self) -> &'static str {
		"Saved Networks"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&[
			"network",
			"wifi",
			"wi fi",
			"wi-fi",
			"saved",
			"ethernet",
			"connection",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let (net_tx, mut net_rx) = tokio::sync::mpsc::unbounded_channel();
		crate::task::spawn(async move {
			Self::get_connections(net_tx).await;
		});

		let target = target.downgrade();
		crate::task::spawn_local(async move {
			let settings = match net_rx.recv().await {
				Some(settings) => settings,
				None => return, //TODO err msg
			};
			let target = match target.upgrade() {
				Some(target) => target,
				None => return,
			};
			for setting in settings {
				let connection_settings = setting.connection.unwrap();
				view! {
					outer_box = gtk4::Box {
						set_orientation: Orientation::Horizontal,
						set_margin_start: 24,
						set_margin_end: 24,
						set_margin_top: 8,
						set_margin_bottom: 8,
						append: connect_button = &Button {
							add_css_class: "settings-button",
							set_hexpand: true,
							set_child: inner_box = Some(&gtk4::Box) {
								set_orientation: Orientation::Horizontal,
								set_spacing: 16,
								append: icon = &Image::from_icon_name(Some("network-wireless-symbolic")) {},
								append: label = &Label::new(Some(&connection_settings.id.unwrap())) {}
							}
						},
						append: settings_button = &Button {
							add_css_class: "settings-button",
							set_icon_name: "emblem-system-symbolic",
						}
					}
				}
				target.append(&outer_box);
			}
		});
	}
}

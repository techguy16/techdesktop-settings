// SPDX-License-Identifier: GPL-3.0-only

use crate::{sections::SettingsGroup, ui::SettingsGui};
use cosmic_dbus_networkmanager::{
	device::SpecificDevice, interface::enums::ApSecurityFlags, nm::NetworkManager,
};
use futures::{StreamExt, TryFutureExt};
use gtk4::{
	glib, prelude::*, Align, Button, Dialog, HeaderBar, Image, Label, Orientation, Spinner,
};
use itertools::Itertools;
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use slotmap::{DefaultKey, SlotMap};
use std::{
	rc::Rc,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};
use tokio::sync::mpsc::UnboundedSender;
use zbus::Connection;

pub struct VisibleNetworks {
	spinner: Spinner,
}

impl Default for VisibleNetworks {
	fn default() -> Self {
		view! {
			spinner = Spinner {
				set_margin_top: 8,
				set_margin_bottom: 8,
				set_margin_start: 8,
				set_margin_end: 8,
				set_halign: Align::Center,
				set_spinning: true
			}
		}
		Self { spinner }
	}
}

impl VisibleNetworks {
	fn handle_access_point(
		target: &gtk4::Box,
		tx: &UnboundedSender<NetworksEvent>,
		aps: &SlotMap<DefaultKey, AccessPoint>,
	) {
		while let Some(widget) = target.first_child().as_ref() {
			target.remove(widget);
		}

		for (id, ap) in aps.iter() {
			// dbg!(&ap);
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
							append: icon = &Image::from_icon_name("network-wireless-symbolic") {},
							append: label = &Label::new(Some(&ap.ssid)) {}
						}
					},
					append: settings_button = &Button {
						add_css_class: "settings-button",
						set_icon_name: "emblem-system-symbolic",
					}
				}
			}

			connect_button.connect_clicked(glib::clone!(@strong tx => move |_| {
				let _ = tx.send(NetworksEvent::ConfigureDevice(id));
			}));

			settings_button.connect_clicked(glib::clone!(@strong tx => move |_| {
				let _ = tx.send(NetworksEvent::ConfigureDevice(id));
			}));

			target.prepend(&outer_box);
		}
	}

	async fn scan_for_devices(tx: UnboundedSender<NetworksEvent>) {
		let sys_conn = match Connection::system().await {
			Ok(conn) => conn,
			Err(err) => {
				error!(%err, "Failed to connect to system dbus session");
				return;
			}
		};
		let nm = match NetworkManager::new(&sys_conn).await {
			Ok(p) => p,
			Err(err) => {
				error!(%err, "Failed to set up connection to NetworkManager dbus");
				return;
			}
		};
		let devices = match nm.devices().await {
			Ok(d) => d,
			Err(err) => {
				error!(%err, "Failed to get devices from NetworkManager");
				return;
			}
		};
		let mut all_aps = SlotMap::new();

		for d in devices {
			if let Ok(Some(SpecificDevice::Wireless(w))) = d.downcast_to_device().await {
				let id = d
					.active_connection()
					.and_then(|ac| async move { ac.id().await })
					.await
					.unwrap_or_else(|_| "unknown".to_string());
				if let Err(err) = w.request_scan(std::collections::HashMap::new()).await {
					error!(%err, %id, "Wi-Fi scan failed");
					continue;
				};
				let mut scan_changed = w.receive_last_scan_changed().await;
				if let Some(t) = scan_changed.next().await {
					if let Ok(t) = t.get().await {
						if t == -1 {
							error!(%id, "Getting access point failed");
							continue;
						}
					}
					match w.get_access_points().await {
						Ok(aps) => {
							if !aps.is_empty() {
								for ap in AccessPoint::from_list(aps).await {
									all_aps.insert(ap);
								}

								break;
							}
						}
						Err(err) => {
							error!(%err, %id, "Getting access points failed");
							continue;
						}
					};
				}
			}
		}

		if let Err(err) = tx.send(NetworksEvent::ApList(all_aps)) {
			error!(%err, "failed to send AP list");
		}
	}
}

#[derive(Debug)]
enum NetworksEvent {
	ApList(SlotMap<DefaultKey, AccessPoint>),
	ConfigureDevice(DefaultKey),
	Quit,
}

#[derive(Debug)]
pub struct AccessPoint {
	pub ssid: String,
	pub hw_address: String,
	pub strength: u8,
	pub wpa_flags: ApSecurityFlags,
}

impl AccessPoint {
	pub async fn new(
		ap: cosmic_dbus_networkmanager::access_point::AccessPoint<'_>,
	) -> Option<Self> {
		Some(Self {
			ssid: ap
				.ssid()
				.await
				.map(|x| String::from_utf8_lossy(&x).into_owned())
				.ok()?,
			hw_address: ap.hw_address().await.ok()?,
			strength: ap.strength().await.ok()?,
			wpa_flags: ap.wpa_flags().await.ok()?,
		})
	}

	pub async fn from_list(
		aps: Vec<cosmic_dbus_networkmanager::access_point::AccessPoint<'_>>,
	) -> Vec<Self> {
		let mut out = Vec::<Self>::with_capacity(aps.len());
		for ap in aps {
			if let Some(ap) = Self::new(ap).await {
				out.push(ap);
			}
		}
		let mut ret = out
			.into_iter()
			.sorted_by(|a, b| a.strength.cmp(&b.strength))
			.rev()
			.unique_by(|ap| ap.ssid.clone())
			.collect::<Vec<Self>>();
		// for some reason adding .rev() messes up unique_by, so we do this instead
		ret.reverse();
		ret
	}
}

impl SettingsGroup for VisibleNetworks {
	fn title(&self) -> &'static str {
		"Visible Networks"
	}

	fn keywords(&self) -> &'static [&'static str] {
		&["wifi", "wi-fi", "connect", "ssid"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		target.append(&self.spinner);

		let (net_tx, mut net_rx) = tokio::sync::mpsc::unbounded_channel();

		target.connect_destroy(glib::clone!(@strong net_tx => move |_| {
			let _ = net_tx.send(NetworksEvent::Quit);
		}));

		let cancel = Arc::new(AtomicBool::new(false));

		crate::task::spawn({
			let cancel = cancel.clone();
			let tx = net_tx.clone();
			async move {
				loop {
					if cancel.load(Ordering::Relaxed) {
						warn!("Stopped network scanning");
						return;
					}

					Self::scan_for_devices(tx.clone()).await;
					tokio::time::sleep(std::time::Duration::from_secs(5)).await;
				}
			}
		});

		let target = target.downgrade();
		crate::task::spawn_local(async move {
			let mut aps = SlotMap::new();

			while let Some(event) = net_rx.recv().await {
				match event {
					NetworksEvent::ApList(update) => {
						if let Some(target) = target.upgrade() {
							Self::handle_access_point(&target, &net_tx, &update);
						}

						aps = update;
					}

					NetworksEvent::ConfigureDevice(device) => {
						if let Some(ap) = aps.get(device) {
							debug!(?ap, "Configuring access point");

							view! {
								dialog = Dialog {
									set_titlebar: header = Some(&HeaderBar) {
										add_css_class: "titlebar"
									},
									set_child: info_box = Some(&gtk4::Box) {
										set_orientation: Orientation::Vertical,
										set_spacing: 8,
										container_add: ssid_section = &LabeledItem {
											set_title: "SSID",
											set_child: ssid_label = &gtk4::Label::new(Some(ap.ssid.as_str())) {
												add_css_class: "settings-entry-text"
											}
										},
										container_add: bssid_section = &LabeledItem {
											set_title: "BSSID",
											set_child: bssid_label = &gtk4::Label::new(Some(ap.hw_address.as_str())) {
												add_css_class: "settings-entry-text"
											}
										},
										container_add: strength_section = &LabeledItem {
											set_title: "Signal Strength",
											set_child: strength_label = &gtk4::Label::new(Some(&format!("{}%", ap.strength))) {
												add_css_class: "settings-entry-text"
											}
										},
									}
								}
							}

							crate::task::spawn_local(async move {
								dialog.run_future().await;
								dialog.close();
							});
						}
					}

					NetworksEvent::Quit => {
						warn!("Stopping network scanning");
						cancel.store(true, Ordering::SeqCst);
						break;
					}
				}
			}
		});
	}
}

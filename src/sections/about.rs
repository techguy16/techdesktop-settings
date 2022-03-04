// SPDX-License-Identifier: GPL-3.0-only

use super::{Section, SectionLayout, SettingsGroup};
use crate::ui::SettingsGui;
use bytesize::ByteSize;
use gtk4::{prelude::*, Image, Label, Orientation};
use libcosmic_widgets::{relm4::RelmContainerExt, LabeledItem};
use os_release::OsRelease;
use std::rc::Rc;
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

thread_local!(static SYSTEM_INFO: System = System::new_all());

pub struct AboutSection;

impl Section for AboutSection {
	const NAME: &'static str = "About";
	const ICON: &'static str = "dialog-information-symbolic";

	fn layout() -> SectionLayout {
		SectionLayout::Single(vec![
			PopIcon::boxed(),
			Device::boxed(),
			DeviceSpecs::boxed(),
			OsInfo::boxed(),
		])
	}
}

#[derive(Default)]
struct PopIcon;
impl SettingsGroup for PopIcon {
	fn keywords(&self) -> &'static [&'static str] {
		&[]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		let logo_name = OsRelease::new()
			.ok()
			.and_then(|mut os_release| os_release.extra.remove("LOGO"))
			.unwrap_or_else(|| "distribution-logo".to_string());
		let logo = Image::builder()
			.icon_name(&logo_name)
			.pixel_size(128)
			.build();
		target.append(&logo);
		target.remove_css_class("settings-group");
	}
}

#[derive(Default)]
struct Device;
impl SettingsGroup for Device {
	fn keywords(&self) -> &'static [&'static str] {
		&["device", "hostname", "name", "computer"]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			if let Some(hostname) = info.host_name() {
				view! {
					row = LabeledItem {
						set_title: "Device Name",
						set_child: label = &Label {
							set_text: &hostname
						}
					}
				}
				target.container_add(&row);
			}
		});
	}
}

#[derive(Default)]
struct DeviceSpecs;

impl DeviceSpecs {
	fn gpus() -> Vec<String> {
		use std::process::Command;

		let mut out = vec![];
		if let Ok(output) = Command::new("glxinfo").output() {
			let output = String::from_utf8_lossy(&output.stdout);
			for line in output.lines() {
				if let Some(renderer_string) = line.strip_prefix("OpenGL renderer string: ") {
					let mut gpu = renderer_string.trim().to_string();
					if let Some(index) = gpu.find(';') {
						gpu.truncate(index);
					}
					out.push(gpu);
				}
			}
		}
		out
	}
}

impl SettingsGroup for DeviceSpecs {
	fn keywords(&self) -> &'static [&'static str] {
		&[
			"device",
			"specs",
			"hardware",
			"cpu",
			"memory",
			"ram",
			"storage",
			"gpu",
			"graphics",
			"processor",
			"disk",
			"space",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			let memory = ByteSize::kb(info.total_memory());
			view! {
				memory_row = LabeledItem {
					set_title: "Memory",
					set_child: memory_label = &Label {
						set_text: &memory.to_string_as(true)
					}
				}
			}
			target.container_add(&memory_row);
			let cpu = info.global_processor_info();
			view! {
				memory_row = LabeledItem {
					set_title: "Processor",
					set_child: memory_label = &Label {
						set_text: cpu.name()
					}
				}
			}
			target.container_add(&memory_row);
			let gpus = Self::gpus();
			let graphics_box = gtk4::Box::new(Orientation::Vertical, 8);
			if !gpus.is_empty() {
				for gpu in gpus {
					let label = Label::builder()
						.label(&gpu)
						.css_classes(vec!["settings-entry-text".into()])
						.build();
					graphics_box.append(&label);
				}
				view! {
					gpus_row = LabeledItem {
						set_title: "Memory",
						set_child: &graphics_box
					}
				}
				target.container_add(&gpus_row);
			}
			let disk = &info.disks()[0];
			view! {
				disk_row = LabeledItem {
					set_title: "Disk Capacity",
					set_child: disk_label = &Label {
						set_text: &ByteSize::b(disk.total_space()).to_string_as(true)
					}
				}
			}
			target.container_add(&disk_row);
		});
	}
}

#[derive(Default)]
struct OsInfo;

impl SettingsGroup for OsInfo {
	fn keywords(&self) -> &'static [&'static str] {
		&[
			"x11",
			"xorg",
			"wayland",
			"os",
			"operating system",
			"window",
			"windowing system",
			"64 bit",
			"32 bit",
			"64-bit",
			"32-bit",
			"x86",
			"x64",
			"x86_64",
		]
	}

	fn layout(&self, target: &gtk4::Box, _ui: Rc<SettingsGui>) {
		SYSTEM_INFO.with(|info| {
			if let (Some(os_version), Some(os_name)) = (info.os_version(), info.name()) {
				view! {
					os_row = LabeledItem {
						set_title: "Operating System",
						set_child: os_label = &Label {
							set_text: &format!("{} {}", os_name, os_version)
						}
					}
				}
				target.container_add(&os_row);
			}

			let os_type = if cfg!(target_arch = "aarch64") {
				"64-bit (ARM)"
			} else if cfg!(target_arch = "x86_64") {
				"64-bit (x86)"
			} else if cfg!(target_arch = "x86") {
				"32-bit (x86)"
			} else {
				"Unknown"
			};
			view! {
				arch_row = LabeledItem {
					set_title: "OS Type",
					set_child: arch_label = &Label {
						set_text: os_type
					}
				}
			}
			target.container_add(&arch_row);

			let window_system = match std::env::var("XDG_SESSION_TYPE").as_deref() {
				Ok("wayland") => "Wayland",
				Ok("x11") => "X11",
				_ => "Unknown",
			};
			view! {
				window_system_row = LabeledItem {
					set_title: "Windowing System",
					set_child: window_label = &Label {
						set_text: window_system
					}
				}
			}
			target.container_add(&window_system_row);
		});
	}
}

// SPDX-License-Identifier: GPL-3.0-only

#[macro_use]
extern crate cascade;
#[macro_use]
extern crate relm4_macros;

mod sections;
mod task;
mod ui;
mod widgets;

use crate::{sections::SettingsGroupStore, ui::section};
use gtk4::{
	gdk::Display, gio::ApplicationFlags, prelude::*, CssProvider, StyleContext,
	STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use once_cell::sync::OnceCell;
use std::{cell::RefCell, rc::Rc};
use tokio::runtime::{self, Runtime};

static RT: OnceCell<runtime::Runtime> = OnceCell::new();

fn main() {
	let application = gtk4::Application::new(
		Some("com.system76.cosmic.settings"),
		ApplicationFlags::default(),
	);
	application.connect_activate(build_ui);
	application.run();
}

fn build_ui(application: &gtk4::Application) {
	let provider = cascade! {
		CssProvider::new();
		..load_from_data(include_bytes!(concat!(env!("OUT_DIR"), "/style.css")));
	};
	// We give the CssProvided to the default screen so the CSS rules we added
	// can be applied to our window.
	StyleContext::add_provider_for_display(
		&Display::default().expect("Could not connect to a display."),
		&provider,
		STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	RT.set(Runtime::new().unwrap()).unwrap();
	let window = gtk4::ApplicationWindow::builder()
		.application(application)
		.title("Settings")
		.default_width(800)
		.default_height(600)
		.build();

	let sections_store: SettingsGroupStore = Rc::new(RefCell::new(Vec::new()));
	let ui = Rc::new(ui::SettingsGui::new(&window));
	section::setup::<sections::WifiSection>(ui.clone(), sections_store.clone());
	section::setup::<sections::DesktopSection>(ui.clone(), sections_store.clone());
	section::setup::<sections::KeyboardSection>(ui.clone(), sections_store.clone());
	section::setup::<sections::AboutSection>(ui.clone(), sections_store.clone());
	ui.search.setup(ui.clone(), sections_store);
	ui.content
		.add_named(&ui.search.all_results_scroll, Some("_search"));

	window.show();
}

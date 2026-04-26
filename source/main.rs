// source/main.rs

//                           | 
//            _           _  | [esud] mvtool
//  _____ _ _| |_ ___ ___| | | 30/01/2026
// |     | | |  _| . | . | | | 
// |_|_|_|\_/|_| |___|___|_| | Лицензия: MIT / Apache 2.0
//                           | 

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod service;
mod widgets;
mod system;

mod application;
use application::Application;

fn main() -> Result<(), String> {
	return Application::new()?.init().run();
}
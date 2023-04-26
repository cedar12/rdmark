// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod cmd;
mod menu;
mod lang;
mod db;
mod utils;

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {

    if let Err(e)=db::init(){
        println!("{:?}",e);
    }

    let lang=db::get_conf_default("lang".into(), "zh".into());
    println!("init lang {}",lang);

    let builder=tauri::Builder::default();
    builder.menu(menu::create_menu(lang.clone()))
        
        .setup(move |app|{
            let window = app.get_window("main").unwrap();
            if cfg!(target_os = "windows") {
                set_shadow(&window, true).expect("Unsupported platform!");
            }else{
                window.set_decorations(true).unwrap();
            }
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                window.open_devtools();
            }
            window.emit("lang", lang).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::read_file,cmd::save_file,cmd::update_menu_lang,cmd::get_lang,cmd::pin,cmd::update_picgo,cmd::set_picgo_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

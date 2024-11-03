use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
//use chrono::*;
use chrono::Timelike;
//use chrono::{DateTime, Local};

static mut GLOBAL_VAR: i32 = 0;

//fn time_handler(intial_time: chrono::DateTime<chrono::Local>) -> bool {
fn time_handler(intial_time: u32) -> bool {
    //let current_string = chrono::Local::now().to_string();
    //let mut seconds = chrono::TimeDelta::seconds(10);
    let parsed_intial:u32 = intial_time.to_string().parse().unwrap();
    let current_time = chrono::Local::now().second();
    let mut parsed_current:u32 = current_time.to_string().parse().unwrap();
    let mut end_time = parsed_intial + 5; // Todo, don't forget about 60 seconds in a min, reseting intial
    if end_time > 54 {
        end_time = 59;
    }
    while parsed_current < end_time {
        parsed_current = chrono::Local::now().second().to_string().parse().unwrap();
    }
    return true;
}

fn app() -> Element {
    log::info!("startup log");
    let mut started = use_signal(||false);
    let timer_fin = use_signal(||false);
    let mut count = use_signal(||0); // creates new var init with 0 ( HOOK )
    let mut intial_time = use_signal(||chrono::Local::now().second());
    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        p { "current seconds: {intial_time} "}

        button {
            onclick: move |_event | {
                if started == use_signal(||false) {
                    started.set(true);
                    intial_time.set(chrono::Local::now().second());
                    time_handler(chrono::Local::now().second());
                } else {
                    count+=1;
                }
                //intial_time = use_signal(||chrono::Local::now().second()); you can not use use_signal, use set
            },
            "start timer"
        }
        p { " timer_fin: {timer_fin}" }
        p {class: "white", "Clicked : {count}"}
    }
}

fn main() {
    launch(app)
}


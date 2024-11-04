use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
//use chrono::*;
use chrono::Timelike;
//use chrono::{DateTime, Local};

static mut Global_Counting: bool = false;

//fn time_handler(intial_time: chrono::DateTime<chrono::Local>) -> bool {
fn time_handler(intial_time: u32) {
    unsafe { Global_Counting = true;};
    //let current_string = chrono::Local::now().to_string();
    //let mut seconds = chrono::TimeDelta::seconds(10);
    let parsed_intial:u32 = intial_time.to_string().parse().unwrap();
    let current_time = chrono::Local::now().second();
    let mut parsed_current:u32 = current_time.to_string().parse().unwrap();
    let mut end_time = parsed_intial + 5; 
    if end_time > 54 {
        end_time = 59;
    }
    while parsed_current < end_time {
        parsed_current = chrono::Local::now().second().to_string().parse().unwrap();
    }
    unsafe { Global_Counting = false;};
    return;
}

fn app() -> Element {
    log::info!("startup log");

    let mut show_first_bt = use_signal(||true);
    let timer_fin = use_signal(||false);
    let mut count = use_signal(||0); // creates new var init with 0 ( HOOK )
    let mut intial_time = use_signal(||chrono::Local::now().second());
    let mut oncounting = use_signal(||false);

    let mut parsed_current:u32 = current_time.to_string().parse().unwrap();
    let mut end_time = parsed_intial + 5; 

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        
        p {"time started: {intial_time}"}

        button {
            onclick: move |_event | {
                if (oncounting()) { // * only for display
                    count+=1;
                    intial_time.set(chrono::Local::now().second());
                    if end_time > 54 { // TODO test and fix
                        end_time = 59;
                    }
                } else {
                    let parsed_intial:u32 = intial_time.to_string().parse().unwrap();
                    let current_time = chrono::Local::now().second();

                    oncounting.set(true)

                }
                // TODO if less then end time, add count, When more stop. Problem if no click.
                unsafe { if Global_Counting == false {
                    show_first_bt.set(false);
                    intial_time.set(chrono::Local::now().second()); // displaying time started at
                    time_handler(chrono::Local::now().second());
                } if Global_Counting == true {
                    count+=1;
                }}
            },
            "start timer"
        }

        p { " timer_fin: {timer_fin}" }
        p {class: "white", "Clicked : {count}"}
    }
}

fn main() {
    launch(app);
}


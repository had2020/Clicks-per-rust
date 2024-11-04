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
    let mut end_time = parsed_intial + 5; // Todo, don't forget about 60 seconds in a min, reseting intial
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
    let data = use_future(|| async {
        time_handler(chrono::Local::now().second())
    });

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        
        p {"time started: {intial_time}"}

        button {
            onclick: move |_event | {
                count+=1;
            },
            "test"
        }

        if (*show_first_bt)() {
            button {
                onclick: move |_event | {
                    unsafe { if Global_Counting == false {
                        show_first_bt.set(false);
                        intial_time.set(chrono::Local::now().second()); // displaying time started at
                        data;
                    } if Global_Counting == true {
                        count+=1;
                    }}
                },
                "start timer"
            }
        } else {
            p {"test"}
        }

        p { " timer_fin: {timer_fin}" }
        p {class: "white", "Clicked : {count}"}
    }
}

fn main() {
    launch(app);
}


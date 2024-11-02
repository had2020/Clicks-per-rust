use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
//use chrono::*;
use chrono::Timelike;
//use chrono::{DateTime, Local};

// declaration of a prop
#[derive(Props, Clone, PartialEq)] // marcos
struct CustomProps {
    text: String,
    #[props(optional)] // makes props under optional 
    size: i32
}

fn time_handler(intial_time: chrono::DateTime<chrono::Local>) -> bool {
    //let current_string = chrono::Local::now().to_string();
    //let mut seconds = chrono::TimeDelta::seconds(10);
    let parsed_intial:i32 = intial_time.to_string().parse().unwrap();
    let current_time = chrono::Local::now();
    let mut parsed_current:i32 = current_time.to_string().parse().unwrap();
    let end_time = parsed_intial + 5;
    while parsed_current > end_time {
        parsed_current = chrono::Local::now().to_string().parse().unwrap();
    }
    return true;
}

// props with componet
#[component]
fn Notes1(props: CustomProps) -> Element {
    let current_time = chrono::Local::now();
    rsx! {
        p { "{props.text}" }
        p { "{current_time.second()}" }
    }
}

fn app() -> Element {
    log::info!("startup log");
    let mut timer_fin = use_signal(||false);
    let mut count = use_signal(||0); // creates new var init with 0 ( HOOK )
    let mut intial_time = use_signal(||chrono::Local::now());
    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        p {class: "white", "Testing 1 2 3" } 
        Notes1 {text: "test from  struct"}

        button {
            onclick: move |_event | {
                intial_time = use_signal(||chrono::Local::now());
                time_handler(chrono::Local::now());
            },
            "start timer"
        }
        p { " timer_fin: {timer_fin}" }

        // button tied to event handeler
        button {
            onclick: move |_event | { 
                count+=1;
            },
            "Click me!"
        }
        p {class: "white", "Clicked : {count}"}
    }
}

fn main() {
    launch(app)
}


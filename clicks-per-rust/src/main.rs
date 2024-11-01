use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
use chrono::Timelike;

// declaration of a prop
#[derive(Props, Clone, PartialEq)] // marcos
struct CustomProps {
    text: String,
    #[props(optional)] // makes props under optional 
    size: i32
}

// basic componet
#[component]
fn Notes() -> Element {
    rsx! {
        "Notes component"
    }
}

// testing fuction
fn test(event: Event<MouseData>) {
    //println!("{event:?}"); // printing despite type mismatch
    log::info!("Event thing: {event:?}"); // yes you need a crate called log (A rust universal)
}

fn time_handler(intial_time) {
    let end_time = intial_time + 5 ;
    let mut current_time = chrono::Local::now();
    while (current_time < end_time) {
        current_time = chrono::Local::now();
    }
    return;

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
    let mut count = use_signal(||0); // creates new var init with 0 ( HOOK )
    let mut intial_time = chrono::Local::now(); 
    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        p {class: "white", "Testing 1 2 3" } 
        Notes {}
        Notes1 {text: "test from  struct"}

        button {
            onclick: move |event | {
                intial_time = chrono::Local::now();
                time_handler(intial_time);
            }
        }

        // button tied to event handeler
        button {
            onclick: move |event | { 
                test(event);
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


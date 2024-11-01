use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
use std::time::{SystemTime, UNIX_EPOCH}; // for system time

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
    start_timer();
}

fn start_timer() {
    let system_time = SystemTime::now();
    // Check if time is greater than UNIX_EPOCH to avoid errors
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => log::info!("System time in seconds since UNIX_EPOCH: {}", duration.as_secs()),
        Err(e) => log::info!("Error getting system time: {:?}", e),
    }
}

// props with componet
#[component]
fn Notes1(props: CustomProps) -> Element {
    rsx! {
        p { "{props.text}" }
    }
}

fn app() -> Element {
    log::info!("startup log");
    let mut count = use_signal(||0); // creates new var init with 0 ( HOOK )
    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        p {class: "white", "Testing 1 2 3" } 
        Notes {}
        Notes1 {text: "test from  struct"}

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


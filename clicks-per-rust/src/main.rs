use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform

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

fn start_timer() {

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

    let mut email = use_signal(|| String::from("")); // init with empty sting
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


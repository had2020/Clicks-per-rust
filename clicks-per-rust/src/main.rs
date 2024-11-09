use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform
//use chrono::*;
use chrono::Timelike;
use tokio;
//use chrono::{DateTime, Local};

fn app() -> Element {
    log::info!("startup log");

    let mut count = use_signal(||0.0); // creates new var init with 0 ( HOOK )
    let mut intial_time = use_signal(||chrono::Local::now().second());
    let mut nocounting = use_signal(||true);
    let mut not_ended = use_signal(||true);

    let mut last_current_time = use_signal(||chrono::Local::now().second());
    let mut last_end_time = use_signal(||chrono::Local::now().second());


    let mut cps = use_signal(||0.0);
    let mut cps_float = 0.0_f32;

    let test_future = use_future(///     use_future(move || async move {
        ///         loop {
        ///            if running() {
        ///                count += 1;
        ///            }
        ///            tokio::time::sleep(Duration::from_millis(400)).await;
        ///        })

    spawn(async {
        let _ = tokio::spawn(async {}).await;
    
        let _ = tokio::task::spawn_local(async {
            // some !Send work
        })
        .await;
    });

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        
        //p {"time started: {intial_time}"} // debug 

        if (*not_ended)() {
            button {
                onclick: move |_event | {
                    if nocounting() { 
    
                        intial_time.set(chrono::Local::now().second());
                        nocounting.set(false);
                    } else {
                        //let parsed_intial:u32 = intial_time.to_string().parse().unwrap();
                        let parsed_initial:u32 = (intial_time()).to_string().parse().unwrap(); //error somewhere here!
                        let current_time = chrono::Local::now().second(); // TODO forget errors make a total counted and count the change in seconds place, for timer
                        let parsed_current:u32 = current_time.to_string().parse().unwrap();
                        let mut end_time = parsed_initial + 5; 
                        if end_time > 54 { // TODO test and fix
                            end_time = 59;
                        }
                        last_end_time.set(end_time);
                        
                        if parsed_current < end_time {
                            count+=1.0;
                            last_current_time.set(parsed_current);
                        } else if parsed_current > end_time{ // error here
                            nocounting.set(false);
                            not_ended.set(false);
                        }
                    }
                },
                " Click Me To Count! "
            }
        } else {
            { cps.set(count / 5.0); }
            { cps_float = (cps)() }
            { cps_float = cps_float.round()}
            p {
                class: "White",
                "You clicked {count} times in 5 seconds."
                "Your Clicks Per a Second is {cps} (CPS)"
            }
        }

        p {"end time: {last_end_time}"} // debug
        p {"current time: {last_current_time}"} // debug

        //p { " timer_fin: {timer_fin}" } // debug
        if (*not_ended)() {
            p {class: "white", "Clicked : {count}"}
        } else {
            button {
                onclick: move |_event | {
                    count.set(0.0);
                    nocounting.set(true);
                    not_ended.set(true);
                },
                "Test Again"
            }
            match cps_float {
                2.0 => rsx!{ p {"Your a Grandma"} p {class: "big", "👵"}},
                1.0 => rsx!{ p {"Your an Turtle"} p {class: "big", "🐢"}},
                _ => rsx!{ p {"Your an Auto clicker!"} p {class: "big", "🤖"} }
            }
        }
    }
}

fn main() {
    launch(app);
}


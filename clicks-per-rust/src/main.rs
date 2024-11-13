use dioxus::prelude::*;
use chrono::{TimeDelta, Timelike};
//use tokio::time::{self, Duration};
//use tokio::time::{sleep, Duration};
//use tokio::spawn;
//use async_std::task;
use async_std::task::spawn;


static mut TIME_END: bool = false;

async fn timer() -> f32 {
    //unsafe { TIME_END = true } // DEBUG ONLY
    let mut current_time = chrono::Local::now().second();
    let mut end_time = current_time + 5;
    if end_time > 58 {
        end_time = 5;
    }
    
    loop {
        current_time = chrono::Local::now().second();
        if current_time == end_time {
            break;
        }
        //sleep(Duration::from_millis(100)).await; // short sleep to yield control, otherwise halt 
    }
    
    unsafe { TIME_END = true };
    0.1 
}

fn app() -> Element {
    log::info!("startup log");

    let mut signal = use_signal_sync(|| 0.0_f32);
    let mut count = use_signal(|| 0.0); // creates new var init with 0 ( HOOK )
    let mut nocounting = use_signal(|| true);
    let mut not_ended = use_signal(|| true);

    let mut result = use_signal(||0.0_f32);
    //let last_current_time = use_signal(|| chrono::Local::now().second());
    //let last_end_time = use_signal(|| chrono::Local::now().second());

    //let mut cps = use_signal(|| 0.0);
    let mut cps_float = 0.0_f32;

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        if (*not_ended)() {
            button {
                onclick: move | _event | {
                    if (nocounting)() == true {
                        nocounting.set(false);
                        //spawn(timer()); not returning only running
                        spawn(async move {
                            /* 
                            let _future_cps = timer();
                            //signal.set(_future_cps.clone()); nope of course not
                            // Testing and debuging here
                            let result = _future_cps.await; //works but halts 
                            //result.set(future_cps.await);
                            //count+= 999.9;
                            count.set(result);
                            //not_ended.set(true);
                            */ //only without use async_std::task::spawn;

                            unsafe { TIME_END = true };
                        });
                    }
                },
                " Click Me To Count! OLD"
            }
        }

        button {
            onclick: move | _event | {
                unsafe { if TIME_END == true {
                    count+=888.8;
                } }
                count+= 1.0;
            },
            " Click Me To Count! "
        }

        //p {"end time: {last_end_time}"} // debug
       //p {"current time: {last_current_time}"} // debug

        if (*not_ended)() {
            p {class: "white", "Clicked : {count}"}
        } else {
            button {
                onclick: move |_event | {
                    count.set(0.0);
                    nocounting.set(true);
                    not_ended.set(true);
                    unsafe { TIME_END = false }
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

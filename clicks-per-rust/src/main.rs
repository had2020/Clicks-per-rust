use dioxus::prelude::*;
use chrono::{TimeDelta, Timelike};

fn app() -> Element {
    log::info!("startup log");

    let mut count = use_signal(|| 0.0); // creates new var init with 0 ( HOOK )
    let mut counting = use_signal(|| false); // for using only one variable
    let mut not_ended = use_signal(|| true);
    let mut cps_float = use_signal(|| 0.0_f32);
    let mut end_second = use_signal(|| 0_u32);
    let mut end_minute = use_signal(|| 0_u32);
    let mut starting_second = use_signal(|| 0_u32);
    let mut dev_mode = use_signal(|| false);

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        // The following comments are for debuging, the are under a hidden class
        if (*dev_mode)() {
            p {"current sec{chrono::Local::now().second()}"}
            p {"end sec {end_second}"}
            p {"end min {end_minute}"}
            p {"{starting_second}"}
        }

        if (*not_ended)() {
            button {
                onclick: move | _event | {
                    if (counting)() == true {
                        count+=1.0;
                        let current_second = chrono::Local::now().second();
                        let current_minute = chrono::Local::now().minute();
                        if current_second >= (*end_second)() && current_minute >= (*end_minute)() { // error with min
                            not_ended.set(false);
                            cps_float.set(count / 5.0);
                        } if current_minute < (*end_minute)() {
                            not_ended.set(false);
                            cps_float.set(count / 5.0);
                        } if current_second < (*starting_second)() {
                            not_ended.set(false);
                            cps_float.set(count / 5.0);
                        }
                    } else {
                        let current_second = chrono::Local::now().second();
                        let current_minute = chrono::Local::now().minute();
                        end_second.set(current_second + 5);
                        starting_second.set(current_second);
                        if current_second + 5 > 59 {
                            end_minute.set(current_minute + 1);
                            end_second.set(end_second - 59);
                        }
                        counting.set(true);
                    }
                },
                " Click Me To Count! "
            }
        }

        if (*not_ended)() {
            p {class: "white", "Clicked : {count}"}
        } else {
            button {
                onclick: move |_event | {
                    count.set(0.0);
                    counting.set(false);
                    not_ended.set(true);
                    end_minute.set(0);
                    end_second.set(0);
                },
                "Test Again"
            }

            match (*cps_float)().round() {
                20.0 => rsx!{ p {"Your a God"} p {class: "big", "🤑"}},
                16.0 => rsx!{ p {"Your a Gamer"} p {class: "big", "😎"}},
                14.0 => rsx!{ p {"Your a Boss"} p {class: "big", "🤵‍♂️"}},
                10.0 => rsx!{ p {"Your a Chad"} p {class: "big", "💪"}},
                9.0 => rsx!{ p {"Your a Normie"} p {class: "big", "😃"}},
                8.0 => rsx!{ p {"Your a Ipad kid"} p {class: "big", "👶"}},
                7.0 => rsx!{ p {"Your a Grandma"} p {class: "big", "👵"}},
                5.0 => rsx!{ p {"Your an Turtle"} p {class: "big", "🐢"}},
                _ => rsx!{ p {"Your an Auto clicker!"} p {class: "big", "🤖"} }
            }

            p {"You averaged {cps_float} Clicks per Second (CPS)"}
        }
    }
}

fn main() {
    launch(app);
}

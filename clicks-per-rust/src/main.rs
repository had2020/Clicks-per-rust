use dioxus::prelude::*;
use chrono::Timelike;
use tokio;

static mut TIME_END: bool = false;

fn app() -> Element {
    log::info!("startup log");

    let mut count = use_signal(|| 0.0); // creates new var init with 0 ( HOOK )
    let mut nocounting = use_signal(|| true);
    let mut not_ended = use_signal(|| true);

    let last_current_time = use_signal(|| chrono::Local::now().second());
    let last_end_time = use_signal(|| chrono::Local::now().second());

    let mut cps = use_signal(|| 0.0);
    let mut cps_float = 0.0_f32;

    let exp = move || {
        spawn(async move {
            unsafe {TIME_END = true}
            count+= 999.9;
        });
    };

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        if (*not_ended)() {
            button {
                onclick: move |_event | {
                    exp;
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

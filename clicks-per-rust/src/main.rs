use dioxus::prelude::*;
use chrono::Timelike;
//use tokio;

//static mut TIME_END: bool = false;

fn app() -> Element {
    log::info!("startup log");

    let mut count = use_signal(|| 0.0); // creates new var init with 0 ( HOOK )
    let mut nocounting = use_signal(|| true);
    let mut not_ended = use_signal(|| true);

    let last_current_time = use_signal(|| chrono::Local::now().second());
    let last_end_time = use_signal(|| chrono::Local::now().second());

    let mut cps = use_signal(|| 0.0);
    let mut cps_float = 0.0_f32;

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        if (*not_ended)() {
            button {
                onclick: move | _event | {
                    if (nocounting)() == true {
                        nocounting.set(false);
                        // running parrarell 
                        spawn(async move {

                            let mut current_time = chrono::Local::now().second();
                            let mut end_time = current_time + 5;
                            if end_time > 58 {
                                end_time = 5;
                            }

                            while current_time != end_time {
                                current_time =  chrono::Local::now().second();
                            }
                            not_ended.set(false);

                            // debug test, if parrel runing
                            /* 
                            count+= 999.9;
                            for i in 0..=100000{
                                count+= 10.0;
                            }
                            println!("Clicked!");
                            */
                        });
                    }
                },
                " Click Me To Count! "
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

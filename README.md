# Click Per Second (CPS) Tracker Application

This project is a **Click Per Second (CPS) Tracker** implemented using the **Dioxus framework** and **Rust**. It allows users to measure their clicking speed over a 5-second interval and provides fun, interactive feedback based on their performance. The project showcases **state management**, **hooks**, and **conditional rendering** in Dioxus, serving as an fun example for developers.

---

## Features

### 1. **Click Counter**
- Tracks the number of times the **"Click Me To Count!"** button is pressed within a 5-second window.

### 2. **Timer Management**
- Implements a dynamic 5-second countdown for tracking clicks.

### 3. **Performance Evaluation**
- Delivers personalized feedback based on the user's CPS, using fun labels such as:
  - 🏆 **God**
  - 🐢 **Turtle**

### 4. **Restart Functionality**
- Allows users to reset and retry, testing their clicking skills repeatedly.

### 5. **Developer Mode**
- Includes a debug view that displays internal state variables, aiding in troubleshooting and development.

---

## Screenshots

### **Start of the Program**
![Start of the Program](https://github.com/user-attachments/assets/38c26706-3478-4a93-87a1-006d6a89b8e9)

---

### **After One User Loop**
![After One User Loop](https://github.com/user-attachments/assets/c64eb17e-6ad8-4f29-8eb6-5c80e2978f95)

---

This project is perfect example for those looking to explore **interactive UI development** with **Dioxus and Rust**. 


# Code Documentation

## Imports

```rust
use dioxus::prelude::*;
use chrono::{TimeDelta, Timelike};
```
- Dioxus: The UI framework used for creating the reactive application.
- Chrono: A date and time library for managing timer-related logic.

### Application Logic
``` rust
app() Function
```
 The main component of the application. It defines the UI structure and the application's logic.

### Hooks
 - count: Tracks the number of clicks.
 - counting: Boolean to track if the click counting is active.
 - not_ended: Boolean to track whether the game is in progress.
 - cps_float: Tracks the calculated CPS score.
 - end_second & end_minute: Tracks the end time for the 5-second interval.
 - starting_second: Tracks the starting second of the interval.
 - dev_mode: Enables or disables developer debugging mode.

### Developer Mode
```rust
if (*dev_mode)() {
    p {"current sec{chrono::Local::now().second()}"}
    p {"end sec {end_second}"}
    p {"end min {end_minute}"}
    p {"{starting_second}"}
}
```
Displays internal variables to help debug the timing and logic.

UI Components
# Example Counting Button
```rust
button {
    onclick: move | _event | { ... },
    " Click Me To Count! "
}
```
Handles:

- Starting the 5-second interval.
- Incrementing the click count.
- Detecting the end of the interval to calculate CPS.

# Results and Feedback
``` rust
p {"You averaged {cps_float} Clicks per Second (CPS)"}
```
Conditional rendering displays:

- A personalized message based on the user's CPS score.
- A restart button to reset the game.

# Feedback Levels
The match statement provides entertaining feedback for various CPS ranges:

``` rust
match (*cps_float)().round() {
    20.0 => rsx!{ p {"Your a God"} p {class: "big", "🤑"}},
    16.0 => rsx!{ p {"Your a Gamer"} p {class: "big", "😎"}},
    9.0 => rsx!{ p {"Your a Normie"} p {class: "big", "😃"}},
    5.0 => rsx!{ p {"Your a Turtle"} p {class: "big", "🐢"}},
    _ => rsx!{ p {"Your an Auto clicker!"} p {class: "big", "🤖"} }
}
```

# Restart Button
- Allows users to reset the state and try again:

``` rust
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
```

# Styling
The app uses an external CSS file (styles.css) for styling. Make sure to include it in the project directory.

Main Function
``` rust
fn main() {
    launch(app);
}
```

### How to Run
1. Install Rust: Ensure you have Rust installed on your machine. If not, install it from rust-lang.org.
2. Add Dependencies:
```bash
cargo add chrono
```
- Add hrono to your Cargo.toml.
3. Run the App:
```bash
cargo tauri dev
```
4. Build the App:
```bash
cargo tauri build
```

### Future Enhancements
- Add user authentication for tracking individual scores.
- Implement a leaderboard to compare scores globally.
- Support custom time intervals for more flexibility.
- Enhance UI with animations or sound effects.
# Feel free to fork the project and contribute! 🎉

use std::cmp::Ordering;

use dioxus::prelude::*;
use rand::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut rng = rand::thread_rng();
    let random_number = use_signal(|| rng.gen_range(1..=100));
    let mut hint = use_signal(|| String::from("Try once!")); 
    let mut isVictory = use_signal(|| false);
    let mut guesses = use_signal(Vec::<i32>::new);
    let mut guess = use_signal(|| 0);

    let check_guess = move |_| {
        let guessed_number = &guess.cloned();
        guesses.push(*guessed_number);
        
        match guessed_number.cmp(&random_number.cloned()){
            Ordering:: Less => {hint.set(String::from("Too small!"))},
            Ordering:: Greater => {hint.set(String::from("Too Big"))},
            Ordering:: Equal => {hint.set(String::from("You Win"));  isVictory.set(true)},
        }
    };

    rsx! {
        section { class: "flex flex-col items-center justify-center grow",
            div { class: "flex flex-col gap-3 items-center",
                label { class: "flex flex-col text-lg gap-2 items-center",
                    "Guess the random number from 1 to 100"
                    input {
                        disabled: if isVictory.cloned() { true },
                        r#type: "number",
                        class: if isVictory.cloned() {
                            "border-2 rounded-md text-center border-green-300"
                        } else {
                            "border-2 rounded-md text-center"
                        },
                        oninput: move |event| {
                            let valueParsed: Result<i32, _> = event.value().parse();
                            match valueParsed {
                                Ok(number) => guess.set(number),
                                Err(_) => {}
                            }
                        }
                    }
                }
                button {
                    disabled: if isVictory.cloned() { true },
                    class: if isVictory.cloned() {
                        "border-2 p-1 w-fit px-10 bg-green-300"
                    } else {
                        "border-2 p-1 w-fit px-10 bg-slate-300 hover:bg-slate-400"
                    },
                    onclick: check_guess,
                    "Enter"
                }
                span {
                    if isVictory.cloned() {
                        "You Win!"
                    } else {
                        "Hint: {hint}"
                    }
                }
                span { "Numbers tried: {guesses:?}" }
            }
        }
    }
}
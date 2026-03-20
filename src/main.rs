use std::io::{stdin, Write, stdout};
use std::fs::{write, read_to_string};
use std::env::var;
use chrono::Local;
use serde::Deserialize;

fn input(prompt: &str) -> String {
    let mut buffer = String::new();

    println!("{}", prompt);
    print!("> ");
    stdout().flush();
    stdin().read_line(&mut buffer);

    return buffer.trim().to_string();
}

fn input_y_or_n(prompt: &str, default: bool) -> bool {
    let formatted_prompt = format!("{} [y/n]", prompt);

    loop {
        match input(&formatted_prompt).as_str() {
            "y" | "Y" => {
                return true;
            },
            "N" | "n" => {
                return false;
            },
            "D" | "d" => {
                return default;
            }
            result => {
                eprintln!("{} is incorrectly formatted. try again", result);
            }
        }
    }
}

fn input_num(prompt: &str) -> i32 {
    let formatted_prompt = format!("{} [number]", prompt);
    loop {
        match input(&formatted_prompt).parse(){
            Ok(num) => {
                return num;
            },
            Err(e) => {
                eprintln!("Invalid number: {}", e);
            }
        }
    }
}

#[derive(Deserialize)]
struct Check {
    prompt: String,
    item: String,
    expected: bool
}

#[derive(Deserialize)]
struct Template {
    checks: Vec<Check>
}

fn main() {
    let diary_dir = format!("{}/diary", var("HOME").unwrap());
    let current_date = Local::now()
        .format("%Y-%m-%d")
        .to_string();
    let mut report = String::new();

    
    let template: Template = serde_json::from_value(
        serde_json::from_str(
            &read_to_string(
                &format!("{}/template.json", diary_dir)
            ).unwrap()
        ).unwrap()
    ).unwrap();
    
    report.push_str("## Checks\n");
    let mut all_checks_done = true;
    for check in template.checks {
        let answer = input_y_or_n(&check.prompt, check.expected);

        if answer != check.expected {
            report.push_str(&format!("[ ] {}\n", check.item));
            all_checks_done = false;
        }
    }
    if all_checks_done {
        report.push_str("All Checks Covered.\n\n");
    }
    
    let wakeup_time = input("When did you wake up today? ");
    let sleep_time  = input("When did you logoff diary today? ");
    report.push_str("## Nums\n");
    report.push_str(&format!(
        "Wakeup time: {wakeup_time}.\nlogoff time: {sleep_time}.\n"));
    
    let thoughts    = input("any toughts?");
    report.push_str("## Thoughts\n");
    report.push_str(&format!("{}\n", thoughts));

    println!("{}",&report);
    write(&format!("{}/{}", &diary_dir, &current_date), &report).unwrap();
}

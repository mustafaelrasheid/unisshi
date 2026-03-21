mod args;
mod utils;
mod template;

use std::fs::{write, read_to_string};
use std::env::var;
use chrono::Local;
use clap::Parser;
use crate::args::{Cli, Commands};
use crate::template::{Template, Check};
use crate::utils::{input, input_y_or_n};

fn handle_checks(checks: Option<&[Check]>, report: &mut String) {
    let checks = if let Some(val) = checks { val } else { return; };
    let mut all_checks_done = true;
    
    report.push_str("## Checks\n");
    for check in checks {
        let answer = input_y_or_n(&check.prompt, check.expected);

        if answer != check.expected {
            report.push_str(&format!("[ ] {}\n", check.item));
            all_checks_done = false;
        }
    }
    if all_checks_done {
        report.push_str("All Checks Covered.\n\n");
    } else {
        report.push_str("\n");
    }
}

fn handle_sleep_and_login(report: &mut String) {
    let wakeup_time = input("When did you wake up today? ");
    let sleep_time  = input("When did you logoff diary today? ");
    
    report.push_str("## Nums\n");
    report.push_str(
        &format!(
            "Wakeup time: {wakeup_time}.\nlogoff time: {sleep_time}.\n\n"
        )
    );
}

fn get_thought() -> String {
    let text = input("any thoughts?");
    let timestamp = Local::now().format("%H:%M").to_string();
    let result = format!("[{}]: {}\n", timestamp, text);
    
    return result;
}

fn handle_thought(report: &mut String) {
    let thought = get_thought();

    report.push_str("## Thoughts\n");
    report.push_str(&thought);
}

fn main() {
    let cli = Cli::parse();
    let diary_dir = format!(
        "{}/diary",
        var("HOME").expect("No HOME dir was set")
    );
    let current_date = Local::now()
        .format("%Y-%m-%d")
        .to_string();

    match cli.command {
        Commands::Today => {
            let mut report = String::new();
            let template: Template = serde_json::from_value(
                serde_json::from_str(
                    &read_to_string(
                        &format!("{}/template.json", diary_dir)
                    ).expect("No template file was found")
                ).expect("Invalid Json")
            ).expect("Missing fields or wrong types");

            handle_checks(template.checks.as_deref(), &mut report);
            handle_sleep_and_login(&mut report);
            handle_thought(&mut report);
            write(
                &format!("{}/{}", &diary_dir, &current_date),
                &report
            ).expect("Failed to write today's diary");
        },
        Commands::AddThought => {
            let mut report = read_to_string(
                &format!("{}/{}", &diary_dir, &current_date)
            ).expect("Unable to find today's notes").trim().to_string();
            let thought = get_thought();
            
            report.push_str("\n");
            report.push_str(&format!("{}\n\n", thought));
            write(
                &format!("{}/{}", &diary_dir, &current_date),
                &report
            ).expect("Failed to write today's diary");
        }
    }
}

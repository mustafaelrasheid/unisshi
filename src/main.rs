mod args;
mod utils;
mod template;

use std::fs::{write, read_to_string, create_dir_all};
use std::env::var;
use std::process::exit;
use chrono::Local;
use clap::Parser;
use crate::args::{Cli, Commands};
use crate::template::{Template, Check, Query};
use crate::utils::{input, input_y_or_n};

fn handle_checks(checks: Option<&[Check]>, report: &mut String) {
    let checks = if let Some(val) = checks { val } else { return; };
    let mut all_checks_done = true;
    
    report.push_str("## Checks\n");
    for check in checks {
        if input_y_or_n(&check.prompt, check.expected) != check.expected {
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

fn handle_queries(queries: Option<&[Query]>, report: &mut String) {
    let queries = if let Some(val) = queries { val } else { return; };
    
    report.push_str("## Queries\n");
    for query in queries {
        report.push_str(
            &format!(
                "{}: {}\n",
                query.item,
                input(&query.prompt)
            )
        )
    }
    report.push_str("\n");
}

fn get_thought() -> String {
    let text = input("any thoughts?");
    let timestamp = Local::now()
        .format("%H:%M")
        .to_string();
    let result = format!("[{}]: {}\n", timestamp, text);
    
    return result;
}

fn handle_thought(report: &mut String) {
    let thought = get_thought();

    report.push_str(&format!("## Thoughts\n{}", thought));
}

fn get_template(diary_dir: &str) -> Option<Template> {
    if let Ok(val) = &read_to_string(
        &format!("{}/template.json", diary_dir)
    ) {
        return serde_json::from_value(
            serde_json::from_str(val).unwrap_or_else(|e| {
                eprintln!("Failed to parse template file due to {}", e);
                exit(1);
            })
        ).unwrap_or_else(|e| {
            eprintln!("Failed to use template file due to {}", e);
            exit(1);
        });
    } else {
        return None;
    };
}

fn get_report(diary_dir: &str, current_date: &str) -> String {
    return read_to_string(
        &format!("{}/{}.md", &diary_dir, &current_date)
    ).unwrap_or_else(|e| {
        eprintln!("Unable to find today's report due to {}", e);
        exit(1);
    }).to_string();
}

fn write_report(report: &str, diary_dir: &str, current_date: &str) {
    write(
        &format!("{}/{}.md", &diary_dir, &current_date),
        &report
    ).expect("Failed to write today's diary");
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
    
    create_dir_all(&diary_dir).expect("Failed to initalize diary dir");
    match cli.command {
        Commands::Today => {
            let mut report = String::new();
            
            if let Some(template) = get_template(&diary_dir) {
                handle_checks(template.checks.as_deref(), &mut report);
                handle_queries(template.queries.as_deref(), &mut report);
            } 
            handle_thought(&mut report);
            write_report(&report, &diary_dir, &current_date);
        },
        Commands::AddThought => {
            let mut report = get_report(&diary_dir, &current_date);
            let thought = get_thought();
            
            report.push_str(&thought);
            write_report(&report, &diary_dir, &current_date);
        },
        Commands::Recheck => {
            let mut buffer = String::new();
            let template = get_template(&diary_dir).unwrap_or_else(|| {
                eprintln!("No template config was found");
                exit(1);
            });
            let template_checks = template.checks.unwrap_or_else(|| {
                eprintln!("Template has no checks field");
                exit(1)
            });
            let report = get_report(&diary_dir, &current_date);
            if !report.starts_with("## Checks") {
                eprintln!("No checks section found in report");
                exit(1);
            }
            let (first, rest) = report.split_once("\n\n")
                .unwrap_or_else(|| {
                    eprintln!("Invalid report format: no separator");
                    exit(1);
                });
            let checks = first
                .split("\n")
                .filter_map(|check|
                    Some(
                        check
                        .to_string()
                        .strip_prefix("[ ] ")?
                        .trim()
                        .to_string()
                    )
                )
                .map(|check| template_checks
                    .iter()
                    .find(|entry| entry.item == check)
                    .unwrap_or_else(||{
                        eprintln!(
                            "Couldn't match report checksto template checks");
                        exit(1);
                    }).clone()
                )
                .collect::<Vec<Check>>();

            handle_checks(Some(checks.as_ref()), &mut buffer);
            buffer.push_str(rest);
            write_report(&buffer, &diary_dir, &current_date);
        }
    }
}

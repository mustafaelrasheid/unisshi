# Unisshi
Unisshi is a minimal CLI app in Rust designed to help you write your diaries.
especially if you need to check a few things off your mind before ending the
day, and would log internal thoughts in easy-to-map formats.

## Usage
 - `unisshi today`: Asks you for information about your day.
 - `unisshi add-thought`: asks you for your current thought and adds it to
    your day report.
 - `unisshi recheck`: asks you the checks you missed.

## Configuration
 - To configure, run `mkdir ~/diary/` and create `~/diary/template.json` with
   contents `{}` to start.
 - To add checks, create an array in `template.json` called `checks` with each
   item being a Check item, a Check containing three
   fields: `item` (String), `expected` (bool), and `prompt` (String).
 - To add queries, create an array in `template.json` called `queries` with
   each item being a Query item, each Query containing two fields:
   `item` (String), `prompt` (String).

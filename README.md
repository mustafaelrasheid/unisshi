# unisshi
Unisshi is a minimal cli app in rust designed to help you write your diaries,
specially if you need to check a few things off of your mind before ending the
day, and would log internal thoughts in easy to map formats.

## Usage
 - `unisshi today`: Asks you information about your day
 - `unisshi add-thought`: asks you your current thought and adds it to your day
    report
 - `unisshi recheck`: asks you the checks you missed

## Configuration
 - to configure, run `mkdir ~/diary/` and create `~/diary/template.json` with
    contents `{}` to start.
 - to add checks, create an array in `template.json` called `checks` with each
    item being a Check item, a Check containing three
    fields: `item` (String), `expected` (bool), and `prompt` (String).
 - to add queries, create an array in `template.json` called `queries` with
   each item being a Query item, each Query containing two fields:
   `item` (String), `prompt` (String).

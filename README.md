# Unisshi
A personal diary and logging cli app.
Specially if you need to check a few things off of your mind before ending
the day, and log internal thoughts in easy-to-map formats.

## Usage
 - `unisshi today`: asks you for information about your day.
 - `unisshi thought`: asks you for your current thought and adds it to
    your day report.
 - `unisshi recheck`: asks you the checks you missed.

## Configuration
 - All your diaries would be placed in `~/diary/`. Optionally, you can
   create `~/diary/template.json` with contents `{}` to start.
 - To add checks, create an array in `template.json` called `checks` with each
   item being a Check item, a Check containing three
   fields: `item` (String), `expected` (bool), and `prompt` (String).
 - To add queries, create an array in `template.json` called `queries` with
   each item being a Query item, each Query containing two fields:
   `item` (String), `prompt` (String).

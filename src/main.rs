use chrono::{Duration, Local, Datelike};
use std::collections::HashMap;
use std::process::Command;
use std::io::{self, Write};

fn get_font_map() -> HashMap<char, Vec<&'static str>> {
    let mut f = HashMap::new();
    // (A-Z definitions remain the same as before...)
    f.insert('A', vec![".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]);
    f.insert('B', vec!["####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."]);
    f.insert('C', vec![".####", "#....", "#....", "#....", "#....", "#....", ".####"]);
    f.insert('D', vec!["###..", "#..#.", "#...#", "#...#", "#...#", "#..#.", "###.."]);
    f.insert('E', vec!["#####", "#....", "#....", "####.", "#....", "#....", "#####"]);
    f.insert('F', vec!["#####", "#....", "#....", "####.", "#....", "#....", "#...."]);
    f.insert('G', vec![".####", "#....", "#....", "#.###", "#...#", "#...#", ".###."]);
    f.insert('H', vec!["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]);
    f.insert('I', vec![".###.", "..#..", "..#..", "..#..", "..#..", "..#..", ".###."]);
    f.insert('J', vec!["..###", "...#.", "...#.", "...#.", "...#.", "#..#.", ".##.."]);
    f.insert('K', vec!["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"]);
    f.insert('L', vec!["#....", "#....", "#....", "#....", "#....", "#....", "#####"]);
    f.insert('M', vec!["#...#", "##.##", "#.#.#", "#...#", "#...#", "#...#", "#...#"]);
    f.insert('N', vec!["#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#"]);
    f.insert('O', vec![".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]);
    f.insert('P', vec!["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."]);
    f.insert('Q', vec![".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#"]);
    f.insert('R', vec!["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"]);
    f.insert('S', vec![".####", "#....", ".###.", "....#", "....#", "....#", "####."]);
    f.insert('T', vec!["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."]);
    f.insert('U', vec!["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]);
    f.insert('V', vec!["#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#.."]);
    f.insert('W', vec!["#...#", "#...#", "#...#", "#.#.#", "#.#.#", "##.##", "#...#"]);
    f.insert('X', vec!["#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#"]);
    f.insert('Y', vec!["#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#.."]);
    f.insert('Z', vec!["#####", "....#", "...#.", "..#..", ".#...", "#....", "#####"]);
    f.insert(' ', vec![".....", ".....", ".....", ".....", ".....", ".....", "....."]);
    f
}

fn main() {
    let font = get_font_map();

    print!("🎨 Enter text to paint: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let word = input.trim().to_uppercase();

    print!("🌿 Fill all other days with 1 commit for a solid green background? (y/n): ");
    io::stdout().flush().unwrap();
    let mut fill_input = String::new();
    io::stdin().read_line(&mut fill_input).unwrap();
    let should_fill = fill_input.trim().to_lowercase() == "y";

    println!("\nPreviewing grid:");
    for row in 0..7 {
        for c in word.chars() {
            if let Some(glyph) = font.get(&c) {
                let row_str = glyph[row].replace('#', "█").replace('.', "░");
                print!("{} ", row_str);
            }
        }
        println!();
    }

    print!("\nProceed with commits? (y/n): ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().to_lowercase() != "y" { return; }

    let now = Local::now();
    let mut start_date = now - Duration::weeks(50);
    while start_date.weekday().number_from_sunday() != 1 {
        start_date = start_date - Duration::days(1);
    }

    let mut painted_dates = std::collections::HashSet::new();

    let mut col_offset = 0;
    for c in word.chars() {
        if let Some(glyph) = font.get(&c) {
            for col in 0..5 {
                for row in 0..7 {
                    if glyph[row].chars().nth(col) == Some('#') {
                        let commit_date = start_date + Duration::weeks(col_offset + col as i64) + Duration::days(row as i64);
                        painted_dates.insert(commit_date.date_naive());
                        make_commits(commit_date, 15); // Dark Green
                    }
                }
            }
            col_offset += 6;
        }
    }

    if should_fill {
        println!("Filling in the gaps...");
        let mut d = start_date;
        while d <= now {
            if !painted_dates.contains(&d.date_naive()) {
                make_commits(d, 1); // Light Green
            }
            d = d + Duration::days(1);
        }
    }

    println!("\n✨ Painting complete! Run 'git push origin main' to publish.");
}

fn make_commits(date: chrono::DateTime<Local>, count: i32) {
    let date_str = date.format("%Y-%m-%dT12:00:00").to_string();
    for _ in 0..count {
        let _ = Command::new("git")
            .args(["commit", "--allow-empty", "--date", &date_str, "-m", "pixel"])
            .env("GIT_AUTHOR_DATE", &date_str)
            .env("GIT_COMMITTER_DATE", &date_str)
            .output();
    }
}
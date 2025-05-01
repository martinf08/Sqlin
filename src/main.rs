use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    let contents = clipboard.get_text().expect("Failed to read clipboard content");

    let lines: Vec<String> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let escaped = line.trim().replace('\'', "''");
            format!("'{}'", escaped)
        })
        .collect();

    let count = lines.len();

    let result = lines.join(",\n");

    clipboard.set_text(result).expect("Failed to write to clipboard");

    println!("✅ {} item(s) transformed and copied to clipboard.", count);
}

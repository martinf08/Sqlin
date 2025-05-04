use arboard::Clipboard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cb = Clipboard::new()?;
    let lines: Vec<_> = cb.get_text()?
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            (!l.is_empty()).then(|| format!("'{}'", l.replace('\'', "''")))
        })
        .collect();

    cb.set_text(&lines.join(",\n"))?;
    println!("✅ {} item(s) transformed and copied to clipboard.", lines.len());
    Ok(())
}

use clap::Parser;
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number_of_tables: u16,
    #[arg(short, long)]
    file_name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let x = 10;
    let mut y = 10;
    let mut i = 0;
    let mut f = File::create(args.file_name)?;
    let mut s: String = String::new();

    s.push_str(&format!("#N canvas 382 105 757 524 10;\n"));

    while i < args.number_of_tables {
        s.push_str(&format!("#X obj {} {} table \\$0-{}-l;\n", x, y, i));
        s.push_str(&format!("#X obj {} {} table \\$0-{}-r;\n", 20 * x, y, i));
        y += 20;
        i += 1;
    }

    s.push_str(&format!("#X obj {} {} \\$0;\n", x, y + 30));
    s.push_str(&format!("#X obj {} {} loadbang;\n", x, y));
    s.push_str(&format!("#X obj {} {} outlet;\n", x, y + 60));
    s.push_str(&format!("#X connect {} 0 {} 0;\n", 2 * i, 2 * i + 2));
    s.push_str(&format!("#X connect {} 0 {} 0;\n", 2 * i + 1, 2 * i));
    f.write_all(s.as_bytes())?;
    Ok(())
}

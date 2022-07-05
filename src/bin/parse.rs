use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::io::Write;

fn main() {
    let markdown_input = std::fs::read_to_string("questions/left-right.md").unwrap_or_else(|e| {
        eprintln!("md file read error(s): {}", e);
        std::process::exit(1);
    });
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // Set up parser. We can treat is as any other iterator. We replace Peter by John
    // and image by its alt text.
    let mut md_parse_options = Options::empty();
    md_parse_options.insert(Options::ENABLE_TABLES);
    md_parse_options.insert(Options::ENABLE_FOOTNOTES);
    md_parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    md_parse_options.insert(Options::ENABLE_TASKLISTS);
    md_parse_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    md_parse_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(&markdown_input, md_parse_options);

    for item in parser {
        println!("{:?}", item);
    }

    // // Write to anything implementing the `Write` trait. This could also be a file
    // // or network socket.
    // let stdout = std::io::stdout();
    // let mut handle = stdout.lock();
    // handle.write_all(b"\nHTML output:\n").unwrap();
    // html::write_html(&mut handle, parser).unwrap();
}

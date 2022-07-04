use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use std::{self, path::PathBuf};
use tera::Context;
use tera::Tera;

const TEMPLATES_GLOB: &'static str = "templates/*.html";
const QUESTIONS_GLOB: &'static str = "questions/*.md";
const HTML_OUTPUT_DIR: &'static str = "docs/";

#[derive(Serialize)]
struct Question<'a> {
    id: usize,
    question: &'a str,
    options: Vec<&'a str>,
    answer: Vec<bool>,
    hint: &'a str,
    explaination: &'a str,
}

fn main() {
    let tera = Tera::new(TEMPLATES_GLOB).unwrap_or_else(|e| {
        eprintln!(
            "template parsing error(s) for glob {}: {}",
            TEMPLATES_GLOB, e
        );
        std::process::exit(1);
    });
    let md_glob = glob(QUESTIONS_GLOB).unwrap_or_else(|e| {
        eprintln!("glob parsing error(s) for glob {}: {}", QUESTIONS_GLOB, e);
        std::process::exit(1);
    });
    let md_paths = md_glob.into_iter().map(|entry| {
        entry.unwrap_or_else(|e| {
            eprintln!("glob collection error(s): {}", e);
            std::process::exit(1);
        })
    });
    let mut md_parse_options = Options::empty();
    md_parse_options.insert(Options::ENABLE_TABLES);
    md_parse_options.insert(Options::ENABLE_FOOTNOTES);
    md_parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    md_parse_options.insert(Options::ENABLE_TASKLISTS);
    md_parse_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    md_parse_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    for md_path in md_paths {
        println!("generating html for {:?}", md_path);
        let html_path = {
            let file_name = md_path.file_name().unwrap_or_else(|| {
                eprintln!("md file name read error(s)");
                std::process::exit(1);
            });
            let mut html_path = PathBuf::from(HTML_OUTPUT_DIR);
            html_path.push(&file_name);
            let extension_success = html_path.set_extension("html");
            if !extension_success {
                eprintln!(".md -> .html extension failed");
                std::process::exit(1);
            }
            html_path
        };
        let md_str = std::fs::read_to_string(md_path).unwrap_or_else(|e| {
            eprintln!("md file read error(s): {}", e);
            std::process::exit(1);
        });
        let html_str = {
            let parser = Parser::new_ext(&md_str, md_parse_options);
            let mut html_str = String::new();
            html::push_html(&mut html_str, parser);
            html_str
        };

        let question = Question {
            id: 1,
            question: "so the question is...",
            options: ["option 1", "option 2"].into(),
            answer: [true, false].into(),
            hint: "so the hint is ...",
            explaination: "so the explaination is ...",
        };
        let tera_context = Context::from_serialize(&question).unwrap_or_else(|e| {
            eprintln!("tera context parse error(s): {}", e);
            std::process::exit(1);
        });
        let html_str = tera
            .render("multi-choice-multi-correct.html", &tera_context)
            .unwrap_or_else(|e| {
                eprintln!("tera template render error(s): {}", e);
                std::process::exit(1);
            });
        std::fs::write(html_path, html_str).unwrap_or_else(|e| {
            eprintln!("html file write error(s): {}", e);
            std::process::exit(1);
        });
    }
}

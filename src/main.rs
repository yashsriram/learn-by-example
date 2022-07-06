use glob::glob;
use std::{self, path::PathBuf};
use tera::Context;
use tera::Tera;

mod parse;
use parse::*;

const TEMPLATES_GLOB: &'static str = "templates/*.html";
const QUESTIONS_GLOB: &'static str = "questions/*.md";
const HTML_OUTPUT_DIR: &'static str = "docs/";

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
        let mcmcq_html = parse(md_str).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
        let tera_context = Context::from_serialize(&mcmcq_html).unwrap_or_else(|e| {
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

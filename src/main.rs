use serde::Serialize;
use std::error::Error;
use std::path::Path;
use std::{self, path::PathBuf};
use tera::Context;
use tera::Tera;
use walkdir::WalkDir;

mod parse;
use parse::*;

#[cfg(test)]
mod tests;

const QUESTIONS_DIR: &'static str = "src/questions";
const TEMPLATE_GLOB: &'static str = "src/templates/*.tera";
const INDEX_TEMPLATE: &'static str = "index.tera";
const MCMQ_TEMPLATE: &'static str = "multi-choice-multi-correct.tera";
const HTML_OUTPUT_DIR: &'static str = "docs";
const HTML_INDEX_FILE: &'static str = "index.html";

#[derive(Serialize, Debug)]
pub struct IndexContext {
    id: PathBuf,
    children: Vec<(bool, String)>,
}

#[derive(Serialize, Debug)]
pub struct MultiChoiceMultiCorrectContext {
    stem_path: PathBuf,
    truth_removed_html: String,
    truth_html: String,
    truth_values: Vec<bool>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let entries: Vec<_> = WalkDir::new(QUESTIONS_DIR).into_iter().collect();
    entries
        .iter()
        .all(|entry| entry.is_ok())
        .then_some(())
        .ok_or("errors found while walking dir")?;
    let entries: Vec<_> = entries.into_iter().filter_map(|entry| entry.ok()).collect();
    entries
        .iter()
        .all(|entry| {
            entry.file_type().is_dir()
                || (entry.file_type().is_file()
                    && entry
                        .path()
                        .extension()
                        .map(|extension| extension == "md")
                        .unwrap_or(false))
        })
        .then_some(())
        .ok_or("non-md file found")?;
    let tera = Tera::new(TEMPLATE_GLOB)?;
    std::fs::remove_dir_all(HTML_OUTPUT_DIR)?;
    for entry in entries {
        let path_rooted_at_questions_dir: PathBuf = entry
            .path()
            .components()
            .skip(Path::new(QUESTIONS_DIR).components().count())
            .collect();
        if entry.file_type().is_dir() {
            print!("generating html for {:?} ... ", entry.path());
            let html_path = {
                let mut html_path = PathBuf::from(HTML_OUTPUT_DIR);
                html_path.push(&path_rooted_at_questions_dir);
                html_path.push(HTML_INDEX_FILE);
                html_path
            };
            let children: Vec<_> = WalkDir::new(entry.path())
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .map(|entry| {
                    let entry = entry.unwrap_or_else(|e| {
                        eprintln!("walk dir error {}", e);
                        std::process::exit(1);
                    });
                    let stem = entry
                        .path()
                        .file_stem()
                        .unwrap_or_else(|| {
                            eprintln!("no file stem");
                            std::process::exit(1);
                        })
                        .to_str()
                        .unwrap_or_else(|| {
                            eprintln!("os string to str problem");
                            std::process::exit(1);
                        })
                        .into();
                    let is_file = entry.file_type().is_file();
                    (is_file, stem)
                })
                .collect();
            let index_context = IndexContext {
                id: path_rooted_at_questions_dir,
                children,
            };
            let tera_context = Context::from_serialize(&index_context)?;
            let html_str = tera.render(INDEX_TEMPLATE, &tera_context)?;
            let html_dir = html_path
                .parent()
                .ok_or("html file parent dir getting error")?;
            std::fs::create_dir_all(html_dir)?;
            std::fs::write(html_path.clone(), html_str)?;
            println!("done. {:?}", html_path);
        } else if entry.file_type().is_file() {
            let md_str = std::fs::read_to_string(entry.path())?;
            print!("generating html for {:?} ... ", entry.path());
            let html_path = {
                let mut path = PathBuf::from(HTML_OUTPUT_DIR);
                path.push(&path_rooted_at_questions_dir);
                path.set_extension("html")
                    .then_some(())
                    .ok_or(".md -> .html extension failed")?;
                path
            };
            let stem_path = {
                let mut path = PathBuf::new();
                path.push(&path_rooted_at_questions_dir);
                path.set_extension("")
                    .then_some(())
                    .ok_or(".md extension removal failed")?;
                path
            };
            let mcmc_context = parse(stem_path, md_str)?;
            let tera_context = Context::from_serialize(&mcmc_context)?;
            let html_str = tera.render(MCMQ_TEMPLATE, &tera_context)?;
            let html_dir = html_path
                .parent()
                .ok_or("html file parent dir getting error")?;
            std::fs::create_dir_all(html_dir)?;
            std::fs::write(html_path.clone(), html_str)?;
            println!("done. {:?}", html_path);
        } else {
            eprintln!(
                "file is neither a regualar file nor a directory {:?}",
                entry.path()
            );
            std::process::exit(1);
        }
    }
    Ok(())
}

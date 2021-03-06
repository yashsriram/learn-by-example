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
    path_rooted_at_questions_dir: PathBuf,
    truth_removed_html: String,
    truth_html: String,
    truth_values: Vec<bool>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let tera = Tera::new(TEMPLATE_GLOB)?;
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
        .ok_or("non dir file or non-md file found")?;
    std::fs::remove_dir_all(HTML_OUTPUT_DIR)?;
    for entry in entries {
        let path_rooted_at_questions_dir: PathBuf = entry
            .path()
            .components()
            .skip(Path::new(QUESTIONS_DIR).components().count())
            .collect();
        if entry.file_type().is_dir() {
            let html_path = {
                let mut html_path = PathBuf::from(HTML_OUTPUT_DIR);
                html_path.push(&path_rooted_at_questions_dir);
                html_path.push(HTML_INDEX_FILE);
                html_path
            };
            let mut children = vec![];
            for child in WalkDir::new(entry.path()).min_depth(1).max_depth(1) {
                let child = child?;
                let stem = child
                    .path()
                    .file_stem()
                    .ok_or("no file stem")?
                    .to_str()
                    .ok_or("os string to str problem")?
                    .into();
                let is_file = child.file_type().is_file();
                children.push((is_file, stem));
            }
            let index_context = IndexContext {
                id: path_rooted_at_questions_dir,
                children,
            };
            print!("generating {:?} for {:?} ... ", html_path, entry.path());
            let tera_context = Context::from_serialize(&index_context)?;
            let html_str = tera.render(INDEX_TEMPLATE, &tera_context)?;
            let html_dir = html_path
                .parent()
                .ok_or("html file parent dir getting error")?;
            std::fs::create_dir_all(html_dir)?;
            std::fs::write(html_path.clone(), html_str)?;
            println!("done.");
        } else if entry.file_type().is_file() {
            let md_str = std::fs::read_to_string(entry.path())?;
            let mut children = vec![];
            for child in WalkDir::new(entry.path()).min_depth(1).max_depth(1) {
                let child = child?;
                let stem: String = child
                    .path()
                    .file_stem()
                    .ok_or("no file stem")?
                    .to_str()
                    .ok_or("os string to str problem")?
                    .into();
                let is_file = child.file_type().is_file();
                children.push((is_file, stem));
            }
            let html_path = {
                let mut path = PathBuf::from(HTML_OUTPUT_DIR);
                path.push(&path_rooted_at_questions_dir);
                path.set_extension("html")
                    .then_some(())
                    .ok_or(".md -> .html extension failed")?;
                path
            };
            print!("generating {:?} for {:?} ... ", html_path, entry.path());
            let mcmc_context = parse(path_rooted_at_questions_dir.clone(), md_str)?;
            let tera_context = Context::from_serialize(&mcmc_context)?;
            let html_str = tera.render(MCMQ_TEMPLATE, &tera_context)?;
            let html_dir = html_path
                .parent()
                .ok_or("html file parent dir getting error")?;
            std::fs::create_dir_all(html_dir)?;
            std::fs::write(html_path.clone(), html_str)?;
            println!("done.");
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

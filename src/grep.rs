use std::{fs, thread};
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::Result;
use regex::Regex;
use colored::*;

struct GrepOutput {
    filename: String,
    line_number: usize,
    matched_string: String,
}

impl GrepOutput {
    pub fn new(filename: String, line_number: usize, matched_string: String) -> Self {
        Self { filename, line_number, matched_string }
    }
}

impl Display for GrepOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let colored_filename = self.filename.green();
        let colored_line_number = format!("{:>2}", self.line_number.to_string()).blue();
        write!(f, "{}: {}: {}", colored_filename, colored_line_number, self.matched_string)
    }
}

fn send_filepath<P>(sender: Sender<PathBuf>, dir_path: P, recursive: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    for dir_entry_result in fs::read_dir(dir_path)? {
        let dir_entry = dir_entry_result?;
        let path = dir_entry.path();

        let metadata = fs::metadata(&path)?;
        match metadata {
            m if m.is_file() || m.is_symlink() => {
                let sender_clone = sender.clone();
                thread::spawn(move || {
                    let _ = sender_clone.send(dir_entry.path());
                });
            }
            m if m.is_dir() && recursive => {
                let _ = send_filepath(sender.clone(), path, recursive);
            }
            _ => {}
        }
    }

    Ok(())
}

fn grep_file_line(filename: &str, line_number: usize, line_str: &str, regex: &Regex) -> Option<GrepOutput> {
    let cap = regex.captures(line_str)?;
    let matched_str = cap.get(1)?;

    let matched_red_string = matched_str.as_str().red().to_string();
    let result = regex.replace_all(line_str, matched_red_string);

    Some(GrepOutput::new(filename.to_string(), line_number + 1, result.trim().to_string()))
}

fn grep_file_content<P>(path: P, regex: &Regex) -> Result<()>
where
    P: AsRef<Path>,
{
    let filename = path.as_ref().display().to_string();
    let content = read_to_string(path)?;

    for (line, line_str) in content.lines().enumerate() {
        let grep_output = grep_file_line(&filename, line, line_str, regex);
        match grep_output {
            Some(output) => {
                println!("{}", output)
            }
            None => {}
        }
    }

    Ok(())
}

fn grep_received_filepath(receiver: Receiver<PathBuf>, regex: &Regex) -> Result<()> {
    for pathbuf in receiver {
        grep_file_content(pathbuf, regex)?;
    }

    Ok(())
}

pub fn call<P>(recursive: bool, dir: P, regex: Regex) -> Result<()>
where
    P: AsRef<Path>,
{
    let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = mpsc::channel();

    send_filepath(tx, dir, recursive)?;
    grep_received_filepath(rx, &regex)?;

    Ok(())
}
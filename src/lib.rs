use errors::NotewormError;
use std::{fs::read_dir, path::{Path, PathBuf}};
use log::info;
use opts::Opts;
use chrono::{NaiveDate, NaiveDateTime, DateTime, Local};


pub mod errors;
pub mod opts;

#[derive(Clone, Copy, Debug)]
pub enum FileType {
    Markdown,
    Image,
    Canvas,
    Excalidraw,
    Json,
    Pdf,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct NoteFileMeta {
    file_path: PathBuf,
    file_relative_path: PathBuf,
    file_size: u64,
    file_created: DateTime<Local>,
    file_modified: DateTime<Local>,
	note_summary: Option<String>,
	note_type: Option<FileType>,
    note_created: Option<NaiveDateTime>,
    note_updated: Option<NaiveDateTime>,
}

pub fn noteworm(opts: &Opts) -> Result<(), NotewormError> {
    println!("Enter Noteworm. Dun dun dun. {:?}", opts.command);

    match opts.command {
        Some(ref c) => {
            match c {
                opts::Command::Backup { 
                    source, 
                    destination, 
                    .. 
                } => {
                    return backup(source, destination, true);
                },
                opts::Command::Clean { } => todo!(),
                _ => todo!(),
            }
        },
        None    => println!("Blah"),
    }
    

    Ok(())
}

pub fn backup(source: &String, destination: &String, dry_run: bool) -> Result<(), NotewormError> {
    info!("Backup from {:?} to {:?} (dry run: {:?}", source, destination, dry_run);
    //let source_path = Path::new(source);
    let source_path: PathBuf = PathBuf::from(source);
    let source_metadata = source_path.metadata()?;
    println!("{:?}", source_metadata.is_dir());

    let source_files = recurse_files(&source_path, &source_path)?;
    for file in source_files {
        println!("File: {:?}", file);
    }

    
    Ok(())
}

fn recurse_files(base_path: &PathBuf, path: &PathBuf) -> Result<Vec<NoteFileMeta>, NotewormError> {
    let mut buf = vec![];
    let entries = read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let meta = entry.metadata()?;
        
        if meta.is_dir() {
            let mut subdir = recurse_files(&base_path, &entry.path())?;
            buf.append(&mut subdir);
        }

        let file_relative_path = entry_path.strip_prefix(base_path).unwrap();

        if meta.is_file() {
            buf.push(NoteFileMeta {
                file_path: entry_path.to_path_buf(),
                file_relative_path: file_relative_path.to_path_buf(),
                file_size: meta.len(),
                file_created: meta.created()?.clone().into(),
                file_modified: meta.modified()?.clone().into(),
                note_summary: None,
                note_type: None,
                note_created: None,
                note_updated: None,
            });
        }
    }
    Ok(buf)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

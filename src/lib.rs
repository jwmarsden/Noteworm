use errors::NotewormError;

use std::os::unix::fs::MetadataExt;
use std::{
    collections::HashMap,
    fmt,
    fs::{File,read_dir},
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use log::info;
use opts::Opts;
use chrono::{NaiveDateTime, DateTime, Local};
use std::ffi::OsStr;

pub mod errors;
pub mod opts;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    Directory,
    Markdown,
    Jpeg,
    Png,
    Svg,
    Canvas,
    Excalidraw,
    Json,
    Pdf,
    Document,
    Javascript,
    Stylesheet,
    Presentation,
    Git,
    Unknown,
}

#[derive(Clone, PartialEq)]
pub struct NoteFileMeta {
    file_path: PathBuf,
    file_relative_path: PathBuf,
    file_size: u64,
    file_type: FileType,
    file_created: DateTime<Local>,
    file_modified: DateTime<Local>,
    file_extension: Option<String>,
	note_summary: Option<String>,
    note_created: Option<NaiveDateTime>,
    note_updated: Option<NaiveDateTime>,
}

impl NoteFileMeta { 
    pub fn new(file_path: PathBuf, file_relative_path: PathBuf, file_size: u64,
        file_created: DateTime<Local>, file_modified: DateTime<Local>) -> Self {
        let file_extension = get_extension_from_path(&file_relative_path);
        let file_type = get_file_type_from_path(&file_relative_path);
        Self { 
            file_path, file_relative_path, file_size,
            file_created, file_modified,
            file_type, file_extension, note_summary: None, 
            note_created: None, note_updated: None,
        }
    }  
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
} 

impl fmt::Debug for NoteFileMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoteFileMeta: {:?} {:?}", self.file_type, self.file_relative_path)
    }
}

#[derive(Clone, PartialEq)]
pub struct MarkdownNoteFile {
    note_file_meta: NoteFileMeta,
    note_len: u32,
    note_contents: String,
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
                opts::Command::Clean { 
                    source, 
                    test_run, 
                    ..
                } => {
                    return clean(source, test_run);
                },
                _ => todo!(),
            }
        },
        None    => println!("Blah"),
    }
    

    Ok(())
}

pub fn clean(source: &String, test_run: &bool) -> Result<(), NotewormError> {
    info!("Clean {:?}  (dry run: {:?})", source, test_run);

    let source_path: PathBuf = PathBuf::from(source);
    let source_metadata = source_path.metadata()?;
    let source_files = recurse_files(&source_path, &source_path)?;
    for ref fileMeta in source_files {
        
        match &fileMeta.file_type {
            FileType::Markdown => {
                //let contents = std::fs::read_to_string(&fileMeta.file_path)?;
                let lines: Vec<String> = lines_from_file(&fileMeta.file_path)?;
                
                println!("Markdown {:?} {:?}", fileMeta.file_relative_path, lines.len());
            }
            _ => {
                println!("Read {:?}", fileMeta);
            },
        }
    }

    Ok(())
}

pub fn backup(source: &String, destination: &String, test_run: bool) -> Result<(), NotewormError> {
    info!("Backup {:?} to {:?} (dry run: {:?}", source, destination, test_run);
    //let source_path = Path::new(source);
    let source_path: PathBuf = PathBuf::from(source);
    //let source_metadata = source_path.metadata()?;
    let source_files: Vec<NoteFileMeta> = recurse_files(&source_path, &source_path)?;

    let mut destination_files_map: HashMap<PathBuf, NoteFileMeta> = HashMap::new();

    for ref file in source_files {
        let mut destination_file_path = PathBuf::from(destination);
        destination_file_path.push(&file.file_relative_path);

        //println!("{:?} -> {:?}", file.file_path, destination_file_path);
        if !destination_file_path.exists() || file_delta_difference_check(&file.file_path, &destination_file_path).unwrap() {
            let destination_prefix = destination_file_path.parent();
            let _ = match destination_prefix {
                Some(prefix) => std::fs::create_dir_all(prefix),
                None => todo!(), 
            };
            println!("{:?} -> {:?}", &file.file_path, destination_file_path);
            std::fs::copy(&file.file_path, &destination_file_path)?;
        }

        destination_files_map.insert(file.file_relative_path.clone(), file.clone());
    }

    print!("Destination Size: {:?}", destination_files_map.len());    
    let destination_path: PathBuf = PathBuf::from(destination);
    let destination_files: Vec<NoteFileMeta> = recurse_files(&destination_path, &destination_path)?;

    let git_prefix: PathBuf = PathBuf::from(".git");

    for ref file in destination_files {
        
        let destination_git_path = file.file_relative_path.clone();
        
        if !destination_git_path.starts_with(&git_prefix) && !destination_files_map.contains_key(&file.file_relative_path) {
            println!("Delete: {:?} {:?}", &file.file_path, destination_git_path);
            std::fs::remove_file(&file.file_path)?;
        }
    }
    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
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

        if meta.is_file() {        
            let file_relative_path = entry_path.strip_prefix(base_path).unwrap();
    
            let note_file_meta = NoteFileMeta::new(
                entry_path.to_path_buf(),
                file_relative_path.to_path_buf(),
                meta.len(),
                meta.created()?.clone().into(),
                meta.modified()?.clone().into(),
            );
            buf.push(note_file_meta);
        }
    }
    Ok(buf)
}

fn file_delta_difference_check(source: &PathBuf, destination: &PathBuf) -> Result<bool, NotewormError> {
    if source.metadata()?.size() != destination.metadata()?.size() {
        return Ok(true);
    }
    if let Result::Ok(source_file) = File::open(source) {
        let mut source_reader = BufReader::new(source_file);
        if let Result::Ok(destination_file) = File::open(destination) {
            let mut destination_reader = BufReader::new(destination_file);
            let mut buf1 = [0; 10000];
            let mut buf2 = [0; 10000];
            loop {
                if let Result::Ok(n1) = source_reader.read(&mut buf1) {
                    if n1 > 0 {
                        if let Result::Ok(n2) = destination_reader.read(&mut buf2) {
                            if n1 == n2 {
                                if buf1 == buf2 {
                                    continue;
                                }
                            }
                            return Ok(true);
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            return Ok(false);
        };
    }
    Ok(false)
}

fn get_extension_from_path(path: &PathBuf) -> Option<String> {
    let extension: &str = path.extension().and_then(OsStr::to_str)?;
    Some(extension.to_string().to_lowercase())
}

fn get_file_type_from_path(path: &PathBuf) -> FileType {
    let mut file_type = get_file_type_from_file_path(path);
    if file_type != FileType::Unknown {
        return file_type;
    }
    let extension_option = get_extension_from_path(path);
    file_type = match &extension_option {
        Some(extension) =>  get_file_type_from_extension(extension),
        None => FileType::Unknown,
    };
    if file_type != FileType::Unknown {
        return file_type;
    }
    file_type = get_file_type_from_file_inspection(path);
    if file_type != FileType::Unknown {
        return file_type;
    }
    return FileType::Unknown;
}

fn get_file_type_from_file_path(path: &PathBuf) -> FileType {
    let file_name = path.file_name().and_then(OsStr::to_str);
    return match file_name {
        Some(file_str) => {
            match file_str {
                ".gitignore" => FileType::Git,
                _ => FileType::Unknown,
            }
        },
        None => FileType::Unknown,
    }
}

fn get_file_type_from_extension(extension: &String) -> FileType {
    return match extension.as_str() {
        "md" | "markdown" => FileType::Markdown,
        "jpg" | "jpeg" => FileType::Jpeg,
        "png" => FileType::Png,
        "svg" => FileType::Svg,
        "pdf" => FileType::Pdf,
        "json" => FileType::Json,
        "js" => FileType::Javascript,
        "canvas" => FileType::Canvas,
        "pptx" | "odp" => FileType::Presentation,
        "docx" => FileType::Document,
        "css" => FileType::Stylesheet,
        "gitignore" => FileType::Git,
        _ => FileType::Unknown 
    }
}

fn get_file_type_from_file_inspection(_path: &PathBuf) -> FileType {
    return FileType::Unknown;
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

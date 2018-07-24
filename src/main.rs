use std::fs;
use std::path::Path;
use std::process;

extern crate clap;
use clap::{App, Arg};

struct Flags {
    show_dot_files: bool,
    show_only_directory: bool,
}

fn main() {
    let app = App::new("rust-tree")
        .version("0.1")
        .author("Kunihiko Tanaka")
        .arg(
            Arg::with_name("path")
                .multiple(true)
                .help("root path")
                .required(false),
        )
        .arg(Arg::with_name("a").short("-a").help("All files are printed. By default tree does not print hidden files (those beginning with a dot '.'). In no event does tree print the file system constructs '.' (current directory) and '..' (previous directory)."))
        .arg(Arg::with_name("d").short("-d").help("List directories only."));

    let matches = app.get_matches();
    let a = matches.is_present("a");
    let d = matches.is_present("d");
    let flags = Flags {
        show_dot_files: a,
        show_only_directory: d,
    };

    if let Some(directory) = matches.values_of("path") {
        directory.for_each(|dir| {
            let path = fs::canonicalize(dir);
            match path {
                Ok(path) => tree(&path.as_path(), "", &flags),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        });
    } else {
        //Display current directory
        let path = std::env::current_dir().unwrap();
        tree(&path.as_path(), "", &flags);
    }
}

fn get_file_metadata(path: &Path) -> std::fs::Metadata {
    let f = fs::metadata(path.to_str().unwrap());
    let md = match f {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    md
}

fn read_dir(path: &Path) -> std::fs::ReadDir {
    let d = fs::read_dir(path);
    let fis = match d {
        Ok(v) => v,
        Err(e) => {
            println!("Could not read directory {}. {}", path.to_str().unwrap(), e);
            process::exit(1);
        }
    };
    fis
}

fn tree(path: &Path, indent: &str, flags: &Flags) {
    let md = get_file_metadata(path);
    println!("{}", path.file_name().unwrap().to_str().unwrap());
    if !md.is_dir() {
        return;
    }
    let fis = read_dir(path);
    let mut p =
        fis.filter(|v| {
            if v.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .starts_with(".") && !flags.show_dot_files
            {
                return false;
            }
            if flags.show_only_directory && !get_file_metadata(&v.as_ref().unwrap().path()).is_dir()
            {
                return false;
            }
            return true;
        }).peekable();

    while let Some(fi) = p.next() {
        let mut add = "│  ";
        if p.peek().is_none() {
            add = "   ";
            print!("{}{}", indent, "└──")
        } else {
            print!("{}{}", indent, "├──")
        }
        tree(&fi.unwrap().path(), &format!("{}{}", indent, add), flags);
    }
}

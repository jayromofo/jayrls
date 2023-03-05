// Take command line arguments 

// search directory

// search target directory

// show undetailed and detailed 

// Change color of things
    // 1. Yellow for a directory 
    // 2. Green for a file

use std::{path::Path, ffi::OsStr};
use std::env;
use std::io;
use clap::Parser;


#[derive(Parser)]
struct CLI {
    command: String,
    path: std::path::PathBuf,

}

// Create a path and use the functions to dissect the paths
fn test_paths(){

    let path = Path::new("/tmp/foo/bar.txt");

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("/tmp/foo")));

    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("bar")));

    let extention = path.extension();
    assert_eq!(extention, Some(OsStr::new("txt")));

    println!("{:?}", path.parent()); 
}

fn main() {

    test_paths();

    

    let current_dir = env::current_dir();


    
    
    println!("Current Directory: {:?}", current_dir);
    
}

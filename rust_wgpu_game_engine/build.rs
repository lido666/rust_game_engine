/*
Accessing files in the res folder
When Cargo builds and runs our program, it sets what's known as the current working directory. 
This directory usually contains your project's root Cargo.toml. The path to our res folder may
 differ depending on the project's structure. In the res folder, the example code for this
  section tutorial is at code/beginner/tutorial9-models/res/. When loading our model, we could use
   this path and just append cube.obj. This is fine, but if we 
   change our project's structure, our code will break.

We're going to fix that by modifying our build script to copy our res folder to where Cargo
 creates our executable, and we'll reference it from there. 

*/


use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::collections::HashMap;
use std::io::BufRead;
use std::{ffi::c_void, fs::File, io::BufReader};

fn main() -> Result<()> {
    // This tells Cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("res/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}

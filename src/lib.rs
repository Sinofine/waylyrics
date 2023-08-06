#![allow(stable_features)]
#![feature(local_key_cell_methods)]
#![feature(is_some_and)]

use std::{cell::RefCell, path::PathBuf};

use regex::RegexSet;

pub mod app;
pub mod config;
pub mod lyric;
pub mod sync;
pub mod utils;

pub const APP_ID: &str = "io.poly000.waylyrics";

thread_local! {
    pub static CONFIG_HOME: RefCell<String> = RefCell::new(String::new());
    pub static CACHE_DIR: RefCell<String> = RefCell::new(String::new());
    pub static THEME_PATH: RefCell<PathBuf> = RefCell::new(PathBuf::new());
    pub static EXCLUDED_REGEXES: RefCell<RegexSet> = RefCell::new(RegexSet::empty());
}

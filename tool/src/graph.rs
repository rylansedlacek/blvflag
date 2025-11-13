use serde::{Serialize, Deserialize};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use dirs;

pub struct Node {
    pub file: String,
    pub line: usize,
    pub function: String,
}

pub struct Graph {
    pub error_type: String,
    pub message: String,
    pub nodes: Vec<ErrorNode>,
    pub edges: Vec<(usize, usize)>,
    pub timestamp: String,
}
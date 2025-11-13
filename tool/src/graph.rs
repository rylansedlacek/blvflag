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

impl Graph {
      pub fn new(error_output: &str) -> Self {
        let re = Regex::new(r#"File "(.+)", line (\d+), in (.+)"#).unwrap();
        let mut nodes = Vec::new();

        for cap in re.captures_iter(error_output) {
            nodes.push(Node {
                file: cap[1].to_string(),
                line: cap[2].parse::<usize>().unwrap_or(0),
                function: cap[3].to_string(),
            });
        }

        let (error_type, message) = Self::extract_error_type(error_output);
        let edges: Vec<(usize, usize)> = (0..nodes.len().saturating_sub(1)).map(|i| (i, i + 1)).collect();

        ErrorGraph {
            error_type,
            message,
            nodes,
            edges,
            timestamp: Local::now().to_rfc3339(),
        }
    }

      fn extract_error_type(stderr: &str) -> (String, String) {



      }

      pub fn save(&self) -> std::io::Result<()> {



        Ok(())
      }
}



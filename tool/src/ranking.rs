use crate::diff;
use crate::buckets::RunRecord;

#[derive(Debug, Clone)]
pub struct RankedCycle {
    pub cycle: Vec<RunRecord>,
    pub error_line_score: f64,
    pub patch_score: f64,
    pub feature_score: f64, // TODO or add
    pub final_score: f64,
}

// error line similarity
fn error_line_score(current_line: &str, historical_line: & str) -> f64 {
    //split by whitespace
    let current_tokens: Vec<&str> = current_line.split_whitespace().collect();
    let historical_tokens: Vec<&str> = historical_line.split_whitespace().collect();
    
    let overlap = current_tokens.iter().filter(|t| historical_tokens.contains(t)).count();
    if current_tokens.is_empty() {return 0.0;}

    let mut over2 = overlap as f64;
    let mut cur2 = current_tokens.len() as f64;

    over2 / cur2
}

// error line similarity part 2 - filtering
fn error_line_filter(current_line: &str, cycles: Vec<Vec<RunRecord>>,) -> Vec<Vec<RunRecord>> {
    cycles.into_iter().filter(|cycle| {
        if cycle.len() < 2 { return false; }
        let historical_line = &cycle[cycle.len() - 2].run_contents;
        error_line_score(current_line, historical_line) > 0.2
    }).collect()
}

// computer the patch score using diffs - diff.rs line 22 added
fn compute_patch_score(pre_fix: &str, post_fix: &str, current_script: &str,) -> f64 {
    let hist_changes = diff::count_changes(pre_fix, post_fix);
    let curr_changes = diff::count_changes(pre_fix, current_script);

    if hist_changes == 0 { return 0.0; }

    let mut hist = hist_changes as f64;
    let mut curr = curr_changes as f64;

    1.0 / (1.0 + (hist-curr).abs())
}

// math Related ****
// create script vector based on common attributes
fn extract_vector(script: &str) -> Vec<f64> {
    vec![
        script.lines().count() as f64, script.matches("def ").count() as f64, // line, function count
        script.matches("if ").count() as f64, script.len() as f64, // branching, size count
    ]
}

fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() { return 0.0; }
    a.iter().zip(b.iter()).map(|(x,y)| x*y).sum()
}






// TODO
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::MAX;
    }

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

// TODO
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot = dot_product(a, b);

    let norm_a = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}
// math Related ****



/*
// MASTER PIPELINE
pub fn select_best_cycles(current_error_line: &str,current_script: &str, 
    cycles: Vec<Vec<RunRecord>>,) -> Vec<Vec<RunRecord>> {
}
    */
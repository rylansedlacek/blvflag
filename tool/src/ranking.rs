use crate::diff;
use crate::buckets::RunRecord;

#[derive(Debug, Clone)]
pub struct RankedCycle {
    pub cycle: Vec<RunRecord>,
    pub error_line_score: f64,
    pub patch_score: f64,
    pub feature_score: f64,
    pub final_score: f64,
}

// ERROR LINE SIMILARITY SCORING
fn error_line_score(current_line: &str, historical_line: &str) -> f64 {

    // split by whitespace
    let current_tokens: Vec<&str> = current_line.split_whitespace().collect();
    let historical_tokens: Vec<&str> = historical_line.split_whitespace().collect();

    let overlap = current_tokens.iter().filter(|t| historical_tokens.contains(t)).count();

    if current_tokens.is_empty() {
        return 0.0;
    }
    overlap as f64 / current_tokens.len() as f64
}

// ERROR LINE SIMILARITY SCORING
// filter out
fn error_line_filter(current_line: &str, cycles: Vec<Vec<RunRecord>>,) -> Vec<Vec<RunRecord>> {
    cycles
        .into_iter()
        .filter(|cycle| {
            if cycle.len() < 2 {
                return false;
            }

            let historical_line = &cycle[cycle.len() - 2].run_contents;
            error_line_score(current_line, historical_line) > 0.2
        })
        .collect()
}

// PATCH BASED SIMILARITY SCORING
fn compute_patch_score(pre_fix: &str, post_fix: &str, current_script: &str,) -> f64 {
    let hist_changes = diff::count_changes(pre_fix, post_fix);
    let curr_changes = diff::count_changes(pre_fix, current_script);

    if hist_changes == 0 {
        return 0.0;
    }

    1.0 / (1.0 + (hist_changes as f64 - curr_changes as f64).abs())
}

// *** TODO TODO TODO ***

// FEATURE VECTOR SIMILARITY


// MASTER PIPELINE
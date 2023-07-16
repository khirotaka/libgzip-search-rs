use std::cmp::{max, min};
use std::collections::HashMap;
use crate::utils;

pub fn search(query: &str, candidate_chunks: &Vec<String>, top_k: usize) -> Vec<(String, f32)> {
    if top_k <= 0 {
        panic!("top_k must be greater than 0.");
    }

    let q = utils::compress_by_gzip(query);
    let mut distance_from_q: HashMap<String, f32> = HashMap::new();

    for chunk in candidate_chunks {
        let c = utils::compress_by_gzip(chunk.as_str());

        let query_chunk = format!("{} {}", query, chunk);
        let q_c = utils::compress_by_gzip(query_chunk.as_str());

        let normalized_distance: f32 = (q_c.len() - min(q.len(), c.len())) as f32 / (max(q.len(), c.len()) as f32);
        distance_from_q.insert(chunk.clone(), normalized_distance);
    }

    utils::sort_by_key(distance_from_q)[..top_k].to_vec()
}
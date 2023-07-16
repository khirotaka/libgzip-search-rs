use std::collections::HashMap;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

pub fn compress_by_gzip(query: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(query.as_bytes())
        .expect("Failed to write a query");

    let result = encoder.finish()
        .expect("Failed to finish compression");

    result
}

pub fn sort_by_key(dict: HashMap<String, f32>) -> Vec<(String, f32)> {
    let mut dict_vec = dict.into_iter().collect::<Vec<_>>();
    dict_vec.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    dict_vec
}
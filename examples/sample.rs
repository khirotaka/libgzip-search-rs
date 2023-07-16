use libgzip_search_rs::by_gzip;

fn main() {
    let candidate_chunks = vec![
        "『北斗の拳』（ほくとのけん）は、武論尊（原作）、原哲夫（作画）による日本の漫画作品。".to_string(),
        "『ちいかわ なんか小さくてかわいいやつ』（通称「ちいかわ」）は、稀代の天才漫画家ナガノによる作品。".to_string(),
        "国会議事堂（こっかいぎじどう、英: National Diet Building）は、日本の国会が開催される議事堂。".to_string(),
        "色覚異常（しきかくいじょう）とは、ヒトの色覚が正常色覚ではない事を示す診断名である。".to_string(),
        "ライツアウトは、5×5の形に並んだライトをある法則にしたがってすべて消灯 (lights out) させることを目的としたパズル。".to_string()
    ];

    let query = "ちいかわって誰の作品ですか？";
    println!("{}", query);
    let (ans, dist) = match by_gzip::search(query, &candidate_chunks, 1).get(0) {
        Some((a, d)) => (a.clone(), d.clone()),
        None => ("見つかりませんでした".to_string(), f32::NAN),
    };
    println!("{} {}", ans, dist);
    println!("----");

    let query = "国会議事堂について教えて？";

    let (ans, dist) = match by_gzip::search(query, &candidate_chunks, 1).get(0) {
        Some((a, d)) => (a.clone(), d.clone()),
        None => ("見つかりませんでした".to_string(), f32::NAN),
    };
    println!("{} {}", ans, dist);
    println!("----");

    for (ans, dist) in by_gzip::search("牛乳パック", &candidate_chunks, 5) {
        println!("{} {}", ans, dist);
    }
}
use simple_search::levenshtein::incremental::IncrementalLevenshtein;
use simple_search::stateful::search_engine::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new::<IncrementalLevenshtein>()
        .with_values(vec![
            "hell".to_string(),
            "world".to_string(),
            "hallo, wie geht es denn heute so?".to_string(),
            "welt".to_string(),
        ])
        .with_key_fn(|state, _, q| state.weighted_similarity(q));

    let result = engine.par_search("hallo");

    println!("found: {:?}", result);
}

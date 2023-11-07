use simple_search::levenshtein::incremental::IncrementalLevenshtein;
use simple_search::stateful::search_engine::SearchEngine;
use simple_search::stateful::state::SearchEngineState;

struct BookSimilarityState {
    title: IncrementalLevenshtein,
    description: IncrementalLevenshtein,
    author: IncrementalLevenshtein,
}

impl SearchEngineState<Book> for BookSimilarityState {
    fn new(value: &Book) -> Self {
        Self {
            title: IncrementalLevenshtein::new("", &value.title),
            description: IncrementalLevenshtein::new("", &value.description),
            author: IncrementalLevenshtein::new("", &value.author),
        }
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    description: String,
    author: String,
}

fn main() {
    let book1 = Book {
        title: "The Winds of Winter".to_string(),
        description: "The long-awaited sixth book in the A Song of Ice and Fire series, where the fate of Westeros will be decided amidst snow and ice.".to_string(),
        author: "George R. R. Martin".to_string(),
    };

    let book2 = Book {
        title: "The Great Gatsby".to_string(),
        description: "A classic novel of the roaring twenties, showcasing the decadence, beauty, and turmoil of the American dream.".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
    };

    let book3 = Book {
        title: "Brave New World".to_string(),
        description: "A visionary and disturbing novel about a dystopian future where society is regimented and controlled by the state.".to_string(),
        author: "Aldous Huxley".to_string(),
    };

    let book4 = Book {
        title: "To Kill a Mockingbird".to_string(),
        description: "A profound novel that deals with serious issues like racial injustice and moral growth through the eyes of a young girl.".to_string(),
        author: "Harper Lee".to_string(),
    };

    let mut engine = SearchEngine::new::<BookSimilarityState>()
        .with_values(vec![book1, book2, book3, book4])
        .with_key_fn(|state, _, query: &str| state.title.similarity(&query))
        .with_key_fn_weighted(0.5, |state, _, query: &str| {
            state.description.similarity(&query)
        })
        .with_key_fn_weighted(0.7, |state, _, query: &str| state.author.similarity(&query));

    let results = engine.similarities("Fire and Ice");

    println!("search for Fire and Ice:");
    for result in results {
        println!("{:?}", result);
    }

    println!();

    let results = engine.similarities("Fitzgerald");

    println!("search for Fitzgerald:");
    for result in results {
        println!("{:?}", result);
    }
}

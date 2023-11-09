use simple_search::levenshtein::base::weighted_levenshtein_similarity;
use simple_search::stateless::search_engine::SearchEngine;
use std::marker::PhantomData;

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

    let mut engine = SearchEngine::new().with_values(vec![book1, book2, book3, book4]);

    let mut query = "Fire and Ice".to_string();

    let results = engine.similarities(query.as_str());

    query.push_str("test");

    println!("search for Fire and Ice:");
    for result in results {
        println!("{:?}", result);
    }

    println!();

    let results = engine.similarities("Fitzgerald");

    let mut query = "Fire and Ice".to_string();

    let test: Test<_> = Test {
        phantom: Default::default(),
    };

    test.test(&query);

    query.push_str("test");

    test.test(&query)
}

struct Test<T> {
    phantom: PhantomData<T>,
}

impl<T> Test<T> {
    fn test(&self, t: T) {}
}

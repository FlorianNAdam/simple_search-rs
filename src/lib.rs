//! A simple library for searching objects.
//! # Basic Usage
//!
//! ```rust
//! use simple_search::search_engine::SearchEngine;
//! use simple_search::levenshtein::base::weighted_levenshtein_similarity;
//!
//!fn main() {
//!     let engine = SearchEngine::new()
//!         .with_values(vec!["hello", "world", "foo", "bar"])
//!         .with(|v, q| weighted_levenshtein_similarity(v, q));
//!
//!     let results = engine.search("hallo");
//!
//!     println!("search for hallo: {:?}", results);
//!}
//! ```
//!
//! # Advanced Usage
//! The following example shows how to use the library with a custom type.
//! The [`SearchEngine`](crate::search_engine::SearchEngine) is configured to search for books by title, author and description.
//! Each of those is weighted differently and the [`IncrementalLevenshtein`](crate::levenshtein::incremental::IncrementalLevenshtein) is used to calculate the similarity.
//!
//!```rust
//!use simple_search::search_engine::SearchEngine;
//!use simple_search::levenshtein::incremental::IncrementalLevenshtein;
//!
//!#[derive(Debug)]
//!struct Book {
//!    title: String,
//!    description: String,
//!    author: String,
//!}
//!
//!fn main() {
//!    let book1 = Book {
//!        title: "The Winds of Winter".to_string(),
//!        description: "The sixth book in the A Song of Ice and Fire series.".to_string(),
//!        author: "George R. R. Martin".to_string(),
//!    };
//!
//!    let book2 = Book {
//!        title: "The Great Gatsby".to_string(),
//!        description: "A classic novel of the roaring twenties.".to_string(),
//!        author: "F. Scott Fitzgerald".to_string(),
//!    };
//!
//!    let book3 = Book {
//!        title: "Brave New World".to_string(),
//!        description: "A visionary and disturbing novel about a dystopian future.".to_string(),
//!        author: "Aldous Huxley".to_string(),
//!    };
//!
//!    let book4 = Book {
//!        title: "To Kill a Mockingbird".to_string(),
//!        description: "A novel that deals with issues like injustice and moral growth.".to_string(),
//!        author: "Harper Lee".to_string(),
//!    };
//!
//!    let mut engine = SearchEngine::new()
//!        .with_values(vec![book1, book2, book3, book4])
//!        .with_state(
//!            |book| IncrementalLevenshtein::new("", &book.title),
//!            |s, _, q| s.weighted_similarity(q),
//!        )
//!        .with_state_and_weight(
//!            0.8,
//!            |book| IncrementalLevenshtein::new("", &book.author),
//!            |s, _, q| s.weighted_similarity(q),
//!        )
//!        .with_state_and_weight(
//!            0.5,
//!            |book| IncrementalLevenshtein::new("", &book.description),
//!            |s, _, q| s.weighted_similarity(q),
//!        );
//!
//!    let results = engine.similarities("Fire adn water");
//!
//!    println!("search for Fire adn water:");
//!    for result in results {
//!        println!("{:?}", result);
//!    }
//!
//!    println!();
//!
//!    let results = engine.similarities("Fitzereld");
//!
//!    println!("Fitzereld");
//!    for result in results {
//!        println!("{:?}", result);
//!    }
//!
//!    println!();
//!}
//!```
//!
//! # Storing an engine
//!
//! The [`SearchEngine`](crate::search_engine::SearchEngine) most often has a very complicated type, that can't easily be expressed.
//! To work around this, the [`type_erasure`](crate::type_erasure) module provides a way to store the engine, by using a trait object in a [Box](std::boxed::Box). \
//! This solution is not ideal, as it requires dynamic dispatch, but the overhead is minimal
//! Once the approved [RFC 2515](https://rust-lang.github.io/impl-trait-initiative/RFC.html) is part of stable rust, this will be replaced with a more elegant solution.
//! For more details on this see the [`type_erasure`](crate::type_erasure) module.
//!
//!```rust
//! use simple_search::search_engine::SearchEngine;
//! use simple_search::levenshtein::incremental::IncrementalLevenshtein;
//! use simple_search::type_erasure::non_cloneable::MutableSearchEngine;
//!
//! fn main() {
//!     let engine = SearchEngine::new()
//!         .with_values(vec!["hello", "world", "foo", "bar"])
//!         .with_state(
//!                 |v| IncrementalLevenshtein::new("", v),
//!                 |s, _, q| s.weighted_similarity(q),
//!         );
//!
//!     let mut engine: MutableSearchEngine<&str, str> = engine.erase_type();
//!     
//!     let results = engine.search("hallo");
//!
//!     println!("search for hallo: {:?}", results);
//! }
//! ```
//!
//! # Parallelization
//!
//! The [`SearchEngine`](crate::search_engine::SearchEngine) can be used in parallel, using [rayon](https://docs.rs/rayon/latest/rayon/) iterators.
//! This simply involved calling the parallel version of the respective function \
//! (As long as the values and query are [Send] + [Sync]).
//!
//! ```rust
//! use simple_search::search_engine::SearchEngine;
//! use simple_search::levenshtein::base::weighted_levenshtein_similarity;
//!
//!fn main() {
//!     let engine = SearchEngine::new()
//!         .with_values(vec!["hello", "world", "foo", "bar"])
//!         .with(|v, q| weighted_levenshtein_similarity(v, q));
//!
//!     let results = engine.par_search("hallo");
//!
//!     println!("search for hallo: {:?}", results);
//!}
//! ```

pub mod levenshtein;
pub mod search_engine;

#[doc(hidden)]
pub mod similarity;
pub mod type_erasure;

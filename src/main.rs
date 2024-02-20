
enum Publication {
    Book(Book),
    Magazine(Magazine),
}


struct Book {
    title: String,
    author: String,
    page_count: u32,
}


struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}


fn create_publications() -> Vec<Publication> {
    vec![
        Publication::Book(Book {
            title: "Hello World".to_string(),
            author: "Alp Eren Gürle".to_string(),
            page_count: 552,
        }),
        Publication::Magazine(Magazine {
            title: "The Rust Times".to_string(),
            issue: 10,
            topic: "Systems Programming".to_string(),
        }),
    ]
}

// Function to display publications in the library.
fn display_publication_info(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => println!(
                "Kitap: {} yazar: {}, {} sayfa",
                book.title, book.author, book.page_count
            ),
            Publication::Magazine(magazine) => println!(
                "Dergi: {} - Sayı: {}, Konu: {}",
                magazine.title, magazine.issue, magazine.topic
            ),
        }
    }
}

fn main() {
    let publications = create_publications();
    display_publication_info(publications);
}

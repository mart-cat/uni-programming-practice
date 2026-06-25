#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Genre {
    Fiction,
    Science,
    History,
}

impl Genre {
    fn title(&self) -> &'static str {
        match self {
            Genre::Fiction => "Fiction",
            Genre::Science => "Science",
            Genre::History => "History",
        }
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
    genre: Genre,
    pages: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16, genre: Genre, pages: u16) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
            genre,
            pages,
        }
    }

    fn describe(&self) -> String {
        format!(
            "{} — {}, {}, {}, {} pages",
            self.title,
            self.author,
            self.year,
            self.genre.title(),
            self.pages
        )
    }
}

fn total_pages(books: &[Book]) -> u32 {
    books.iter().map(|book| u32::from(book.pages)).sum()
}

fn count_by_genre(books: &[Book], genre: Genre) -> usize {
    books.iter().filter(|book| book.genre == genre).count()
}

fn newest_book(books: &[Book]) -> Option<&Book> {
    books.iter().max_by_key(|book| book.year)
}

fn main() {
    let books = vec![
        Book::new("The Rust Journey", "I. Ferris", 2024, Genre::Science, 320),
        Book::new("Safe Systems", "A. Memory", 2022, Genre::Science, 280),
        Book::new("Northern Tales", "L. Snow", 2019, Genre::Fiction, 210),
        Book::new("History of Code", "M. Archive", 2021, Genre::History, 260),
    ];

    println!("Library catalog");
    for book in &books {
        println!("- {}", book.describe());
    }
    println!("Total books: {}", books.len());
    println!("Science books: {}", count_by_genre(&books, Genre::Science));
    println!("Total pages: {}", total_pages(&books));
    if let Some(book) = newest_book(&books) {
        println!("Newest book: {} ({})", book.title, book.year);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_books() -> Vec<Book> {
        vec![
            Book::new("A", "Author 1", 2020, Genre::Science, 100),
            Book::new("B", "Author 2", 2022, Genre::Fiction, 150),
            Book::new("C", "Author 3", 2021, Genre::Science, 200),
        ]
    }

    #[test]
    fn counts_books_by_genre() {
        let books = sample_books();
        assert_eq!(count_by_genre(&books, Genre::Science), 2);
        assert_eq!(count_by_genre(&books, Genre::History), 0);
    }

    #[test]
    fn sums_pages() {
        let books = sample_books();
        assert_eq!(total_pages(&books), 450);
    }

    #[test]
    fn finds_newest_book() {
        let books = sample_books();
        let newest = newest_book(&books).expect("sample list is not empty");
        assert_eq!(newest.title, "B");
        assert_eq!(newest.year, 2022);
    }
}

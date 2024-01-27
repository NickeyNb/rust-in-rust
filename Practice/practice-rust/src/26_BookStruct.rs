// 26- Create a struct representing a book with properties like title, author, and page count.

struct Book {
    title:String,
    author:String,
    page_count:u32
}
fn main() {
    let book1: Book = Book {
        title:String::from("Title 1"),
        author:String::from("Author 1"),
        page_count:1
    };
    
    println!("{}, {}, {}", book1.title, book1.author, book1.page_count);
}
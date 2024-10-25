fn main() { 


    let mut database: Vec<Book> = Vec::new();



    register_book("Moby Dick".to_string(), 2500, Catergory::Horror,&mut database);

    get_book_by_name("mb Dick".to_string(), &mut database);
}



struct Movie{

    name:String,
    price:u32,
     
}

#[derive(Debug,Clone)]
struct Book {
    name:String,
    price:u32,
    category:Catergory
}

#[derive(Debug,Clone)]
enum Catergory{
    Horror,
    Science,
    Action,
    Adventure,
    Criminal,
}


fn register_book(name:String,price:u32,category:Catergory,database:&mut Vec<Book>){


    let book = Book{
        name:name,
        price:price,
        category:category

    };

    database.push((book.clone()));



    println!("Kitabınız kaydedildi {:?}",book)

}

fn get_book_by_name(name:String,database:&mut Vec<Book>) {

    for data in database  {

        if name == data.name {

            println!("Kitap bilgileri {:?}",data)
            
        }
        else {
            println!("{} bu isimde kitap bulunamadı",name)
        }
        
    }
    
}



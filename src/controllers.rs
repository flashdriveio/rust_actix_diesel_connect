use actix_web::Responder;

pub async fn get_all_books()-> impl Responder{
    format!("Hello books")

}

pub async fn create_new_book() -> impl Responder {
    format!("New book")
}
use rocket::response::content;

#[get("/test")]
pub fn test() -> content::Json<&'static str> {
    content::Json("{\"test\":\"test\"}")
}

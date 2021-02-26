use tide::{self, Body, Request, Response, StatusCode};
use tide_auth::{Auth, Basic, User};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();
    let auth = Auth::into_inner(Basic::new());

    app.with(auth);

    app.at("/").get(|r: Request<()>| async move {
        let user = r.ext::<User>();
        Ok(format!("{:?}", user))
    });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

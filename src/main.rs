use std::error::Error;

use iitkgp_erp_login::session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut session = Session::new(
        String::from("rollno").into(),
        String::from("pass").into(),
        None,
    );
    dbg!(session.get_sessiontoken().await?);
    dbg!(session.get_secret_question(None).await?);

    dbg!(session.request_otp(None, "answer".into()).await?);
    Ok(())
}

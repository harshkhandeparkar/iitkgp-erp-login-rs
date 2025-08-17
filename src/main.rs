use std::error::Error;

use iitkgp_erp_login::session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut session = Session::new(String::from("21EC37007").into(), None, None);

	dbg!(session.get_secret_question(None).await?);
	Ok(())
}

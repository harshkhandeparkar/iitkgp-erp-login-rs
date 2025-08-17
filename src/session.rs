use reqwest::{
    Client,
    header::{HeaderMap, USER_AGENT},
};
use scraper::{Html, Selector};
use std::{collections::HashMap, error::Error};

use crate::erp::endpoints;

pub struct Session {
    client: Client,
    /// Roll number
    user_id: Option<String>,
    /// ERP password
    password: Option<String>,
    /// The security question for this session
    question: Option<String>,
    /// Secret/security question's answer
    answer: Option<String>,
    /// Session token
    session_token: Option<String>,
    /// The ERP url/path that is requested/will be redirected to.
    requested_url: Option<String>,
    /// OTP if required
    email_otp: Option<String>,
    /// Headers for the post requests
    headers: HeaderMap,
}

struct ErpCreds {
    /// Student Roll Number
    roll_number: String,
    /// ERP Password
    password: String,
    /// Security Question
    security_questions_answers: HashMap<String, String>,
}

fn get_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "timeout",
        "20".parse().expect("Error setting timeout header."),
    );
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/51.0.2704.79 Chrome/51.0.2704.79 Safari/537.36".parse().expect("Error setting user-agent header."));

    headers
}

impl Session {
    pub fn new(
        user_id: Option<String>,
        password: Option<String>,
        headers: Option<HeaderMap>,
    ) -> Session {
        Session {
            client: Client::new(),
            headers: headers.unwrap_or(get_default_headers()),
            user_id,
            password,
            question: None,
            answer: None,
            session_token: None,
            requested_url: None,
            email_otp: None,
        }
    }

    /// Checks if the session is alive
    pub async fn is_alive(&self) -> Result<bool, Box<dyn Error>> {
        let resp = self.client.get(endpoints::WELCOMEPAGE_URL).send().await?;

        return if let Some(len) = resp.content_length() {
            Ok(len == 1034)
        } else {
            Ok(false)
        };
    }

    /// Fetches the session token
    pub async fn get_sessiontoken(&self) -> Result<Option<String>, Box<dyn Error>> {
        let homepage = self
            .client
            .get(endpoints::HOMEPAGE_URL)
            .send()
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&homepage);

        let session_token_selector = Selector::parse("#sessionToken")?;
        let mut elements = document.select(&session_token_selector);

        if let Some(elem) = elements.next() {
            Ok(elem.attr("value").map(|val| val.into()))
        } else {
            Ok(None)
        }
    }

    /// Fetches the secret question given the rollnumber. If the rollnumber is set in the session struct, it is used instead.
    pub async fn get_secret_question(
        &mut self,
        roll_number: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let roll_number = roll_number.unwrap_or(
            self.user_id
                .as_ref()
                .expect("Error: Roll number not found.")
                .clone(),
        );
        self.user_id = roll_number.clone().into();

        let mut map = HashMap::new();
        map.insert("user_id", roll_number);

        let resp = self
            .client
            .post(endpoints::SECRET_QUESTION_URL)
            .form(&map)
            .headers(self.headers.clone())
            .send()
            .await?
            .text()
            .await?;

        if resp == "FALSE" {
            Err(String::from("Error: Invalid roll number.").into())
        } else {
            self.question = resp.clone().into();

            Ok(resp)
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

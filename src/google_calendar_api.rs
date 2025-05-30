use std::error::Error;
use google_calendar3::{
    hyper_rustls::{self, HttpsConnector},
    hyper_util::{self, client::legacy::connect::HttpConnector},
    yup_oauth2,
    CalendarHub,
};
use dirs::home_dir;

pub struct GoogleCalendarAPI {
    hub: CalendarHub<HttpsConnector<HttpConnector>>,
}

impl GoogleCalendarAPI {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let mut secret_path = home_dir().ok_or("Unable to determine home directory")?;
        secret_path.push(".ultima/secret.json");
        let secret = yup_oauth2::read_application_secret(secret_path)
            .await?;
        let auth_builder = yup_oauth2::InstalledFlowAuthenticator::builder(secret, yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect);
        let mut token_path = home_dir().ok_or("Unable to determine home directory")?;
        token_path.push(".ultima/token.json");
        let auth = auth_builder.persist_tokens_to_disk(&token_path).build().await?;
        let scopes = &[
            "https://www.googleapis.com/auth/calendar",
            "https://www.googleapis.com/auth/calendar.events",
            "https://www.googleapis.com/auth/calendar.readonly",
            "https://www.googleapis.com/auth/calendar.events.readonly",
        ];

        auth.token(scopes).await?;

        let client = hyper_util::client::legacy::Client::builder(
            hyper_util::rt::TokioExecutor::new()
        )
            .build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_or_http()
                    .enable_http1()
                    .build()
            );

        let hub = CalendarHub::new(client, auth);
        Ok(Self {
            hub
        })
    }

    pub async fn get_event_names(&self) -> Option<Vec<String>> {
        let result = self.hub.events().list("xtremejames1@gmail.com").doit().await;
        match result {
            Ok((_, events)) => {
                if let Some(items) = events.items {
                    let mut names = Vec::new();
                    for item in items {
                        if let Some(title) = item.summary {
                            names.push(title);
                        }
                    }
                    Some(names)
                }
                else {
                    None
                }
            }
            Err(_) => {
                None
            }
        }
    }
}

use std::error::Error;
use google_calendar3::{
    hyper_rustls::{self, HttpsConnector}, hyper_util::{self, client::legacy::connect::HttpConnector}, yup_oauth2, CalendarHub
};
use dirs::home_dir;

use crate::event::{CalendarEvent, GcalCalendar};

pub struct GoogleCalendarAPI {
    hub: CalendarHub<HttpsConnector<HttpConnector>>,
    sync_token: Option<String>,
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
        let sync_token = None;
        Ok(Self {
            hub,
            sync_token,
        })
    }

    pub async fn get_calendars(&mut self) -> Result<Vec<GcalCalendar>, ()> {
        let result = self.hub.calendar_list().list().doit().await;
        let mut calendars = Vec::new();
        let (_, mut calendar_list) = result.expect("Invalid calendar list");
        let mut page_token = calendar_list.next_page_token;
        for entry in calendar_list.items.expect("Invalid calendar list items") {
            calendars.push(GcalCalendar::from_calendar_list_entry(entry).expect("Unable to convert calendar into GcalCalendar struct"));
        }

        while page_token.is_some() {
            let result = self.hub.calendar_list()
                .list()
                .page_token(page_token.as_ref().unwrap().as_str())
                .doit().await;
            (_, calendar_list) = result.expect("Invalid calendar list");
            page_token = calendar_list.next_page_token;
            for entry in calendar_list.items.expect("Invalid calendar list items") {
                calendars.push(GcalCalendar::from_calendar_list_entry(entry).expect("Unable to convert calendar into GcalCalendar struct"));
            }
        }
        self.sync_token = calendar_list.next_sync_token;
        //TODO get sync token
        Ok(calendars)
    }

    pub async fn get_events(&self, calendar: GcalCalendar) -> Result<Vec<CalendarEvent>, ()> {
        let calendar_id = calendar.id;
        let result = self.hub.events().list(&calendar_id.clone().as_str()).doit().await;
        let mut events = Vec::new();
        let (_, event_list) = result.expect("Invalid event list");
        let mut page_token = event_list.next_page_token;
        for entry in event_list.items.expect("Invalid event list items") {
            events.push(CalendarEvent::from_gcal_api(entry, calendar_id.clone()).expect("Unable to convert event into CalendarEvent"));
        }

        while page_token.is_some() {
            let result = self.hub.events().list(&calendar_id.clone().as_str()).page_token(page_token.as_ref().unwrap().as_str()).doit().await;
            let mut events = Vec::new();
            let (_, event_list) = result.expect("Invalid event list");
            page_token = event_list.next_page_token;
            for entry in event_list.items.expect("Invalid event list items") {
                events.push(CalendarEvent::from_gcal_api(entry, calendar_id.clone()).expect("Unable to convert event into CalendarEvent"));
            }
        }
        Ok(events)
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

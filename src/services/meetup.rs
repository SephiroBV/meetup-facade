use anyhow::Context;
use serde::Deserialize;

use crate::{responses::meetup::EventInfo, AnyResult};

const URL_BASE: &str = r#"https://api.meetup.com"#;

#[tracing::instrument(
    name = "Requesting event info from meetup.com",
    skip(group_name, event_id)
)]
pub async fn event_status(group_name: &str, event_id: u32) -> AnyResult<MeetupEventResponse> {
    let event_url = format!("{}/{}/events/{}", URL_BASE, group_name, event_id);

    let event = send_get_request(&event_url)
        .await?
        .error_for_status()
        .context("Event request to meetup.com was unsuccessful")?
        .json::<MeetupEventResponse>()
        .await
        .context("Unable to parse event response from meetup.com to MeetupEventResponse")?;

    Ok(event)
}

#[tracing::instrument(name = "Sending GET request to meetup.com", fields(peer.service = "meetup.com", span.kind = "client"))]
async fn send_get_request(url: &str) -> AnyResult<reqwest::Response> {
    let response = reqwest::get(url)
        .await
        .context("Unable to send event request to meetup.com")?;
    Ok(response)
}

#[derive(Clone, Debug, Deserialize)]
pub struct MeetupEventResponse {
    pub name: String,
    pub venue: Venue,
    pub yes_rsvp_count: u8,
}

impl From<MeetupEventResponse> for EventInfo {
    fn from(response: MeetupEventResponse) -> Self {
        Self {
            name: response.name,
            venue: response.venue.name,
            attendees: response.yes_rsvp_count,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Venue {
    pub name: String,
}

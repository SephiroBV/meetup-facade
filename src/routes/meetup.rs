use actix_web::{
    get,
    web::{Json, Path},
    Responder,
};
use futures_util::TryFutureExt;
use serde::Deserialize;

use crate::{responses::meetup::EventInfo, services::meetup};

#[derive(Deserialize)]
pub struct Info {
    group_name: String,
    event_id: u32,
}

#[get("/meetup/{group_name}/{event_id}")]
#[tracing::instrument(
name = "Handling Get Event Info Request",
skip(info),
fields(
group_name = % info.group_name,
event_id = % info.event_id,
)
)]
pub async fn get_event_info(info: Path<Info>) -> impl Responder {
    meetup::event_status(&info.group_name, info.event_id)
        .map_ok(|response| Json(EventInfo::from(response)))
        .await
}

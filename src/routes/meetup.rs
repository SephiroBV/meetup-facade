use actix_web::{
    get,
    web::{Json, Path},
    Responder,
};
use futures_util::TryFutureExt;

use crate::{responses::meetup::EventInfo, services::meetup};

#[get("/meetup/{group_name}/{event_id}")]
#[tracing::instrument(
name = "Handling Get Event Info Request",
skip(path_vars),
fields(
group_name = % path_vars.0,
event_id = % path_vars.1
)
)]
pub async fn get_event_info(path_vars: Path<(String, u32)>) -> impl Responder {
    meetup::event_status(&path_vars.0, path_vars.1)
        .map_ok(|response| Json(EventInfo::from(response)))
        .await
}

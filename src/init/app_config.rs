use actix_web::web;

use crate::routes::{heartbeat::get_heartbeat, meetup::get_event_info};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_heartbeat).service(get_event_info);
}

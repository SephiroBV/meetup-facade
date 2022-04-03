use actix_web::web;

use meetup_facade::init::app_config;

pub fn test_config(cfg: &mut web::ServiceConfig) {
    app_config::config(cfg);
}

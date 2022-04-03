use actix_web::{test, App};

use meetup_facade::responses::meetup::EventInfo;

use crate::utils::test_init::test_config;

#[actix_web::test]
async fn get_event_info() {
    // Arrange
    let app = test::init_service(App::new().configure(test_config)).await;
    let req = test::TestRequest::get()
        .uri("/meetup/rust-amsterdam-group/284647946")
        .to_request();

    // Act & Assert
    let _response: EventInfo = test::call_and_read_body_json(&app, req).await;
}

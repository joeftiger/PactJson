use pact_consumer::{json_pattern, like};
use pact_consumer::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, Eq, PartialEq)]
struct Data {
    id: usize,
}

#[tokio::test]
async fn post_test() {
    let pact = PactBuilder::new_v4("SHIT", "TEST")
        .interaction("posting a Test", "post", |mut i| {
            i.test_name("post_test");
            i.request
                .post()
                .path("/")
                .json_utf8()
                .json_body(json_pattern!(like!(json!(Data::default()))));
            i.response
                .created()
                .json_utf8()
                .json_body(json_pattern!(like!(json!(Data::default()))));
            i.clone()
        })
        .start_mock_server(None);

    let response = reqwest::Client::new().post(pact.path("/"))
        .json(&Data::default())
        .send()
        .await
        .expect("could not fetch URL");
    println!("response: {:#?}", response);
    // response: 500
    assert_eq!(StatusCode::CREATED, response.status());
    // "x-pact": "Request-Mismatch"

    let body = response.json::<Data>().await.expect("could not read response body");
    assert_eq!(Data::default(), body);
}
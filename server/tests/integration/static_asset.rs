// tests/integration/static_asset.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn static_asset_works() {
    let api = TestApi::spawn().await;

    let filenames = ["text.css", "image.ico", "script.js"];

    for file in filenames.iter().enumerate() {
        let response = api.get_static_asset(file.1).await;

        assert_eq!(response.status(), StatusCode::OK);

        let response_header = response
            .headers()
            .get("Content-Type")
            .expect("Expected Content-Type header in response.");

        let response_header_str = response_header
            .to_str()
            .expect("Unable to get the response header text");

        let expected_header_parts: Vec<&str> = file.1.split('.').collect();
        let expected_header_str = format!("{}/{}", expected_header_parts[0], expected_header_parts[1]);

        assert_eq!(response_header_str, &expected_header_str);
    }
}

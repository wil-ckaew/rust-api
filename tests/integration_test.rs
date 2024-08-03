// tests/integration_test.rs
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_create_document() {
    let client = Client::new();
    let url = "http://localhost:8080/api/documents";

    let new_document = json!({
        "user_id": "123e4567-e89b-12d3-a456-426614174000",
        "doc_type": "passport",
        "filename": "test_document.jpg"
    });

    let response_result = timeout(Duration::from_secs(10), async {
        client
            .post(url)
            .header("Content-Type", "application/json")
            .body(new_document.to_string())
            .send()
            .await
    })
    .await
    .expect("Request timed out");

    let response = response_result.expect("Failed to send request");

    // Obtenha o status da resposta antes de consumir o conteúdo
    let status = response.status();

    // Obtenha o texto da resposta
    let response_text = response.text().await.expect("Failed to read response text");

    // Verifique o status da resposta
    assert!(status.is_success());

    // Parse o texto da resposta como JSON
    let response_body: Value = serde_json::from_str(&response_text).expect("Failed to parse response text to JSON");

    // Verifique o conteúdo do JSON
    assert_eq!(response_body["status"], "success");
    assert_eq!(response_body["document"]["user_id"], "123e4567-e89b-12d3-a456-426614174000");
}

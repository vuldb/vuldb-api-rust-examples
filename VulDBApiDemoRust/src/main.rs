/*
	vuldb_api_demo - Rust VulDB API Demo

	License: GPL-3.0
    	Required Dependencies: 
        * reqwest
        * std::error
    	Optional Dependencies: None
*/

use reqwest::{Client, header};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // API URL
    let url = "https://vuldb.com/?api";

    // Headers for authentication
    let personal_api_key = ""; // Enter your personal API key here
    let user_agent = "VulDB API Advanced Rust Demo Agent";

    // Request body parameters
    let recent = "5";   // Default is 5
    let details = "0";  // Default is 0
    let id = "";        // Example: "290848", Default: ""
    let cve = "";       // Example: "CVE-2024-1234", Default: ""

    // Construct the request body
    let request_body = if id.is_empty() && cve.is_empty() {
        format!("recent={}&details={}", recent, details)
    } else if !cve.is_empty() {
        format!("search={}&details={}", cve, details)
    } else {
        format!("id={}&details={}", id, details)
    };

    // Create HTTP client
    let client = Client::new();

    // Set up headers
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, user_agent.parse()?);
    headers.insert("X-VulDB-ApiKey", personal_api_key.parse()?);
    headers.insert(header::CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);

    // Send the request
    let response = client
        .post(url)
        .headers(headers)
        .body(request_body)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with response code: {}", response.status());
    }

    Ok(())
}

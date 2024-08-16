//!
//! This example showcases the Google OAuth2 process for requesting access to the Google Calendar features
//! and the user's profile.
//!
//! Before running it, you'll need to generate your own Google OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run --example google
//! ```
//!
//! ...and follow the instructions.
//!
use gcal::*;

#[tokio::main]
async fn main() {
    let access_key = easy_google_oauth::access_token().await;
    let http_client = http_client::h1::H1Client::new();
    let client = GCalClient::new(http_client, access_key).unwrap();
    let calendar_list = CalendarListClient::new(client);
    let list = calendar_list.list(true, CalendarAccessRole::Reader).await.unwrap();
    for event in &list {
        eprintln!("{} {}", event.id, event.summary);
    }
}

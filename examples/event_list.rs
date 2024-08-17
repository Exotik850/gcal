use chrono::Local;
use gcal::*;

#[tokio::main]
async fn main() {
    let http_client = http_client::h1::H1Client::new();
    let access_key = easy_google_oauth::access_token(&http_client).await;
    let client = GCalClient::new(http_client, access_key).unwrap();
    let calendar_list = CalendarListClient::new(client.clone());
    let list = calendar_list
        .list(true, CalendarAccessRole::Reader)
        .await
        .unwrap();
    let events = EventClient::new(client);
    let start = Local::now();
    let end = Local::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap();
    let mut event_list = Vec::new();
    for calendar in list {
        let events = events
            .list(calendar.id, start, end)
            .await
            .unwrap();
        event_list.extend(events);
    }
    for event in &event_list {
        eprintln!("{:?} {:?}", event.id, event.summary);
    }
}

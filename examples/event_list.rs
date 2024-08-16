use chrono::Local;
use gcal::*;

#[tokio::main]
async fn main() {
    let access_key = easy_google_oauth::access_token().await;
    let client = GCalClient::new(access_key).unwrap();
    let calendar_list = CalendarListClient::new(client.clone());
    let mut list = calendar_list.list(true, CalendarAccessRole::Reader).await.unwrap();
    let events = EventClient::new(client);
    let start = Local::now();
    let end = Local::now().checked_add_signed(chrono::Duration::days(7)).unwrap();
    let list = events.list(list.swap_remove(0).id, start, end).await.unwrap();
    for event in &list {
        eprintln!("{:?} {:?}", event.id, event.summary);
    }
  }

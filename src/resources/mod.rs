/// Calendar List, the normal way to get at the list of calendars available.
pub mod calendar_list;
pub use calendar_list::*;
pub mod calendar;
pub use calendar::*;
pub mod conference_properties;
pub use conference_properties::*;
/// Events, the method you will work with most events in a single calendar.
pub mod events;
pub use events::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SendUpdates {
    #[default]
    All,
    ExternalOnly,
    None,
}

impl ToString for SendUpdates {
    fn to_string(&self) -> String {
        match self {
            Self::All => "all",
            Self::ExternalOnly => "externalOnly",
            Self::None => "none",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CalendarAccessRole {
    FreeBusyReader,
    Reader,
    Writer,
    #[default]
    Owner,
}

impl CalendarAccessRole {
    pub fn to_str(self) -> &'static str {
        match self {
            CalendarAccessRole::Owner => "owner",
            CalendarAccessRole::Writer => "writer",
            CalendarAccessRole::Reader => "reader",
            CalendarAccessRole::FreeBusyReader => "freeBusyReader",
        }
    }
}

impl From<CalendarAccessRole> for String {
    fn from(val: CalendarAccessRole) -> Self {
        val.to_str().into()
    }
}

use extendr_api::prelude::*;

use crate::transit_realtime::FeedMessage;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Positions,
    Updates,
    Alerts,
}

impl MessageType {
    fn to_text(&self) -> &str {
        match self {
            MessageType::Positions => "vehicle positions.",
            MessageType::Updates => "trip updates.",
            MessageType::Alerts => "alerts.",
        }
    }
}

// Issue an R warning if there are unexpected types in the message
pub fn check_types(msg: FeedMessage, expected_type: MessageType) -> Result<()> {
    if expected_type != MessageType::Positions {
        if msg.entity.iter().any(|x| x.vehicle.is_some()) {
            R!(r#"
                cli::cli_warn(c(
                    "!" = paste("File does not contain", {{ expected_type.to_text() }}),
                    "i" = "It does contain vehicle positions.",
                    "v" = "You can read them with {.fn read_gtfsrt_positions}" 
                ))
            "#)?;
        }
    }

    if expected_type != MessageType::Updates {
        if msg.entity.iter().any(|x| x.trip_update.is_some()) {
            R!(r#"
                cli::cli_warn(c(
                    "!" = paste("File does not contain", {{ expected_type.to_text() }}),
                    "i" = "It does contain trip updates",
                    "v" = "You can read them with {.fn read_gtfsrt_trip_updates}" 
                ))
            "#)?;
        }
    }

    if expected_type != MessageType::Alerts {
        if msg.entity.iter().any(|x| x.alert.is_some()) {
            R!(r#"
                cli::cli_warn(c(
                    "!" = paste("File does not contain", {{ expected_type.to_text() }}),
                    "i" = "It does contain alerts",
                    "v" = "You can read them with {.fn read_gtfsrt_alerts}" 
                ))
            "#)?;
        }
    }

    Ok(())
}

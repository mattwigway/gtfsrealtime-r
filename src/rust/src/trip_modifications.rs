use extendr_api::prelude::*;
use extendr_api::error::Result;


use crate::{
    id_deduplicator::IdDeduplicator, read::read_feed, transit_realtime, util::ensure_nonempty,
};

#[derive(IntoDataFrameRow, PartialEq, Debug)]
pub struct RTripModification {
    id: String,
    trip_id: String,
    shape_id: Option<String>,
    start_time: Option<String>,
    service_date: String,
    start_stop_sequence: Option<u32>,
    start_stop_id: Option<String>,
    end_stop_sequence: Option<u32>,
    end_stop_id: Option<String>,
    propagated_modification_delay: Option<i32>,
    replacement_stop_id: Option<String>,
    replacement_travel_time_to_stop: Option<i32>,
    service_alert_id: Option<String>,
    last_modified_time: Option<u64>,
    file_timestamp: Option<u64>,
    file_index: usize,
}

#[extendr]
pub fn read_gtfsrt_trip_modifications_internal(
    file: String,
) -> Result<Dataframe<RTripModification>> {
    // TODO how to handle added shapes and stops

    let msgs: Vec<transit_realtime::FeedMessage> = read_feed(file)?;

    let content = msgs.iter().enumerate().map(|(file_idx, msg)| {
        // IDs expected to be unique only within a feed
        let mut id_deduplicator = IdDeduplicator::new();

        msg.entity
            .iter()
            .filter(|e: &&transit_realtime::FeedEntity| e.trip_modifications.is_some())
            .map(move |entity| {


            
                let modification = entity.trip_modifications.as_ref().unwrap();
                let id = id_deduplicator.deduplicate_id(entity.id.clone());
                                    let _ = R!(r#"
                        cli::cli_warn(c(
                            "!" = paste("Trip Modifications id", {{ id.clone() }}, "exists.")
                            ))
                        "#);
                if modification.service_dates.is_empty() {
                    let _ = R!(r#"
                        cli::cli_warn(c(
                            "!" = paste("Trip Modifications id", {{ id.clone() }}, "does not select any service dates and will not appear in output.")
                            ))
                        "#);
                            }

                                if modification.modifications.is_empty() {
                    let _ = R!(r#"
                        cli::cli_warn(c(
                            "!" = paste("Trip Modifications id", {{ id.clone() }}, "does not contain any modifications and will not appear in output.")
                            ))
                        "#);
                            }

                if modification.selected_trips.is_empty() {
                    let _ = R!(r#"
                        cli::cli_warn(c(
                            "!" = paste("Trip Modifications id", {{ id.clone() }}, "does not select any trips and will not appear in output.")
                        ))
                    "#);
                }

                modification.selected_trips.iter().map(move |selected_trips| {
                    if selected_trips.trip_ids.is_empty() {
                        let _ = R!(r#"
                            cli::cli_warn(c(
                                "!" = paste("Trip Modifications id", {{ id.clone() }}, "shape id", {{ selected_trips.shape_id.clone() }}, "does not select any trips and will not appear in output.")
                            ))
                        "#);
                    }

                    selected_trips.trip_ids.iter().map(|trip_id| {
                        ensure_nonempty(modification.start_times.clone()).iter().map( |start_time| {
                            // if empty, warned above - don't warn for every selected trip
                            modification.service_dates.clone().into_iter().map( |service_date| {
                                modification.modifications.clone().iter().map( |the_mod| {
                                    ensure_nonempty(the_mod.replacement_stops.clone())
                                        .iter()
                                        .map(|replacement_stop| {
                                            RTripModification {
                                                id: id.clone(),
                                                trip_id: trip_id.clone(),
                                                shape_id: selected_trips.shape_id.clone(),
                                                start_time: start_time.clone(),
                                                service_date: service_date.clone(),
                                                start_stop_sequence: match &the_mod.start_stop_selector {
                                                    Some(s) => s.stop_sequence,
                                                    None => None
                                                }, start_stop_id: match &the_mod.start_stop_selector {
                                                    Some(s) => s.stop_id.clone(),
                                                    None => None
                                                },
                                                end_stop_sequence: match &the_mod.end_stop_selector {
                                                    Some(s) => s.stop_sequence,
                                                    None => None
                                                }, 
                                                end_stop_id: match &the_mod.end_stop_selector {
                                                    Some(s) => s.stop_id.clone(),
                                                    None => None
                                                },
                                                propagated_modification_delay: the_mod.propagated_modification_delay,
                                                replacement_stop_id: match replacement_stop {
                                                    Some(s) => s.stop_id.clone(),
                                                    None => None
                                                },
                                                replacement_travel_time_to_stop: match replacement_stop {
                                                    Some(s) => s.travel_time_to_stop,
                                                    None => None
                                                },
                                                service_alert_id: the_mod.service_alert_id.clone(),
                                                last_modified_time: the_mod.last_modified_time,
                                                file_timestamp: msg.header.timestamp,
                                                file_index: file_idx
                                            }
                                        })
                                        .collect::<Vec<RTripModification>>()
                                })
                                .flatten()
                                .collect::<Vec<RTripModification>>()
                            })
                            .flatten()
                            .collect::<Vec<RTripModification>>()
                        })
                        .flatten()
                        .collect::<Vec<RTripModification>>()
                    })
                    .flatten()
                    .collect::<Vec<RTripModification>>()
                })
                .flatten()
                .collect::<Vec<RTripModification>>()
            })
            .flatten()
            .collect::<Vec<RTripModification>>()
    })
    .flatten()
    .collect::<Vec<RTripModification>>();

    // if content.len() == 0 {
    //     check_types(msgs, MessageType::TripModifications)?;
    // }

    content.into_dataframe()
}

extendr_module! {
    mod trip_modifications;
    fn read_gtfsrt_trip_modifications_internal;
}
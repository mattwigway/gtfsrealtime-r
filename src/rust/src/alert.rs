use std::collections::HashSet;

use extendr_api::prelude::*;

use crate::{read::read_feed, transit_realtime::{EntitySelector, TimeRange, TranslatedString}};

#[derive(IntoDataFrameRow, Debug, PartialEq)]
pub struct RAlert {
    id: String,
    start: Option<u64>,
    end: Option<u64>,
    agency_id: Option<String>,
    route_id: Option<String>,
    route_type: Option<i32>,
    direction_id: Option<u32>,
    trip_trip_id: Option<String>,
    trip_route_id: Option<String>,
    trip_direction_id: Option<u32>,
    trip_start_time: Option<String>,
    trip_start_date: Option<String>,
    trip_schedule_relationship: Option<i32>,
    trip_modification_id: Option<String>,
    stop_id: Option<String>,
    cause: Option<i32>,
    language: Option<String>,
    cause_detail: Option<String>,
    effect_detail: Option<String>,
    url: Option<String>,
    header_text: Option<String>,
    description_text: Option<String>,
    tts_header_text: Option<String>,
    tts_description_text: Option<String>,
    severity_level: Option<i32>

    // TODO
    // image_url: Option<String>,
    // image_type: Option<String>,
    // image_alternative_text: Option<String>
}

fn accumulate_languages(set: &mut HashSet<Option<String>>, translated_string: &Option<TranslatedString>) {
    match translated_string {
        None => (),
        Some(str) => {
            for message in &str.translation {
                set.insert(message.language.clone());
            }
        }
    }
}

fn compare_opts(o1: &Option<String>, o2: &Option<String>) -> bool {
    match o1 {
        None => o2.is_none(),
        Some(x) => {
            match o2 {
                None => false,
                Some(y) => x == y
            }
        }
    }
}

fn message_for_language(lang: &Option<String>, translated_string: &Option<TranslatedString>) -> Option<String> {
    match translated_string {
        None => None,
        Some(str) => {
            for message in &str.translation {
                if compare_opts(&message.language, lang) {
                    return Some(message.text.clone());
                }
            }

            return None;
        }
    }
}

/// Read a GTFS-rt service alerts feed.
/// GTFS-rt alerts support translations. If there is more than one translation
/// in an alert, there will be one row for that alert in each language. Alerts
/// in all languages will share a feed_index.
#[extendr]
pub fn read_gtfsrt_alerts_internal(file: String) -> Result<Dataframe<RAlert>> {
    let msg = read_feed(file)?;

    let content = msg.entity.iter()
        .filter(|e| e.alert.is_some())
        .map(|entity| {
            let alert = entity.alert.as_ref().unwrap();

            // figure out what languages we have
            let mut languages: HashSet<Option<String>> = HashSet::new();
            accumulate_languages(&mut languages, &alert.cause_detail);
            accumulate_languages(&mut languages, &alert.effect_detail);
            accumulate_languages(&mut languages, &alert.url);
            accumulate_languages(&mut languages, &alert.header_text);
            accumulate_languages(&mut languages, &alert.description_text);
            accumulate_languages(&mut languages, &alert.tts_header_text);
            accumulate_languages(&mut languages, &alert.tts_description_text);
            // todo the TranslatedImage, but hopefully it has the same translations...
            accumulate_languages(&mut languages, &alert.image_alternative_text);

            // if an alert doesn't have any of the translated strings, make sure we still get a record for it
            if languages.is_empty() {
                languages.insert(None);
            }

            // duplicate alert for each time range, but since time range is optional make sure there is at least one.
            let ranges: Vec<Option<&TimeRange>> = if alert.active_period.is_empty() {
                vec![None]
            } else {
                alert.active_period.iter().map(Some).collect()
            };

            // also duplicate alert for each informed entity. there should always be 1+ per spec,
            // but allow for the possibility there is not.
            let informed_entities: Vec<Option<&EntitySelector>> = if alert.informed_entity.is_empty() {
                vec![None]
            } else {
                alert.informed_entity.iter().map(Some).collect()
            };

            ranges.iter().map(|range| {
                    informed_entities.iter().map(|informed_entity| {
                        languages
                            .iter()
                            .map(|lang| {
                                let trip = informed_entity.map_or(None, |e| e.trip.clone());

                                RAlert {
                                    id: entity.id.clone(),
                                    start: range.map_or(None, |r| r.start),
                                    end: range.map_or(None, |r| r.end),
                                    agency_id: informed_entity.map_or(None, |e| e.agency_id.clone()),
                                    route_id: informed_entity.map_or(None, |e| e.route_id.clone()),
                                    route_type: informed_entity.map_or(None, |e| e.route_type),
                                    direction_id: informed_entity.map_or(None, |e| e.direction_id),
                                    // trip? works here because it is inside a function that returns an option (the closure in map_or)
                                    trip_trip_id: trip.as_ref().map_or(None, |t| t.trip_id.clone()),
                                    trip_route_id: trip.as_ref().map_or(None, |t| t.route_id.clone()),
                                    trip_direction_id: trip.as_ref().map_or(None, |t| t.direction_id.clone()),
                                    trip_start_time: trip.as_ref().map_or(None, |t| t.start_time.clone()),
                                    trip_start_date: trip.as_ref().map_or(None, |t| t.start_date.clone()),
                                    trip_schedule_relationship: trip.as_ref().map_or(None, |t| t.schedule_relationship.clone()),
                                    trip_modification_id: trip.as_ref().map_or(None, |t| t.modified_trip.clone()?.modifications_id.clone()),
                                    stop_id: informed_entity.map_or(None, |e| e.stop_id.clone()),
                                    cause: alert.cause,
                                    language: lang.clone(),
                                    cause_detail: message_for_language(&lang, &alert.cause_detail),
                                    effect_detail: message_for_language(&lang, &alert.effect_detail),
                                    url: message_for_language(&lang, &alert.url),
                                    header_text: message_for_language(&lang, &alert.header_text),
                                    description_text: message_for_language(&lang, &alert.description_text),
                                    tts_header_text: message_for_language(&lang, &alert.tts_header_text),
                                    tts_description_text: message_for_language(&lang, &alert.tts_description_text),
                                    severity_level: alert.severity_level
                                }
                            })
                            .collect::<Vec<RAlert>>()
                        })
                        .flatten()
                        .collect::<Vec<RAlert>>()
                    })
                    .flatten()
                    .collect::<Vec<RAlert>>()

        })
        .flatten()
        .collect::<Vec<RAlert>>();

    return content.into_dataframe();
}

extendr_module! {
    mod alert;
    fn read_gtfsrt_alerts_internal;
}
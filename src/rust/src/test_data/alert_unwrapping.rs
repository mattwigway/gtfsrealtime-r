use extendr_api::prelude::*;

use crate::{
    test_data::write_alerts,
    transit_realtime::{
        alert::{Cause, Effect, SeverityLevel},
        translated_string::Translation,
        trip_descriptor, Alert, EntitySelector, TimeRange, TranslatedString, TripDescriptor,
    },
};

/// Alerts are hierarchical: a single alert can have multiple applicability periods, multiple affected entities,
/// and translations to multiple languages. The read function flattens all of that to a tabular format, with one row
/// for every combination of applicability, entity, and language.
///
/// This has four alerts. The first one has all fields filled out, and has two each of applicability periods,
/// entities informed, and language - so it should become 2*2*2 = 8 rows.
/// The second one is identical but is missing some (but not all) Spanish translations
/// The third one does not have Spanish translations, so it should become 2*2 = 4 rows
/// The fourth one has no time ranges, entities, or languages so should just be one row
/// the fifth one is all NA
///
/// @noRd
#[extendr]
pub fn test_data_alert_unwrapping(filename: &str) -> Result<()> {
    let alerts = vec![
        Alert {
            active_period: vec![
                TimeRange {
                    start: Some(1775079486),
                    end: Some(1775080486),
                },
                TimeRange {
                    start: Some(1775179486),
                    end: Some(1775180486),
                },
            ],
            informed_entity: vec![
                EntitySelector {
                    agency_id: Some("agency1".to_owned()),
                    route_id: Some("route1".to_owned()),
                    route_type: Some(1),
                    trip: Some(TripDescriptor {
                        trip_id: Some("trip1".to_owned()),
                        route_id: Some("trip1_route1".to_owned()),
                        direction_id: Some(0),
                        start_time: Some("20:00:00".to_owned()),
                        start_date: Some("20260311".to_owned()),
                        schedule_relationship: Some(
                            trip_descriptor::ScheduleRelationship::Canceled as i32,
                        ),
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: Some("stop1".to_owned()),
                    direction_id: Some(1),
                },
                EntitySelector {
                    agency_id: Some("agency2".to_owned()),
                    route_id: Some("route2".to_owned()),
                    route_type: Some(2),
                    trip: Some(TripDescriptor {
                        trip_id: Some("trip2".to_owned()),
                        route_id: Some("trip2_route2".to_owned()),
                        direction_id: Some(1),
                        start_time: Some("22:00:00".to_owned()),
                        start_date: Some("20260312".to_owned()),
                        schedule_relationship: Some(
                            trip_descriptor::ScheduleRelationship::Scheduled as i32,
                        ),
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: Some("stop2".to_owned()),
                    direction_id: Some(0),
                },
            ],
            cause: Some(Cause::Weather as i32),
            effect: Some(Effect::NoService as i32),
            url: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "https://example.com/test".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "https://example.com/test".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            header_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "The bus is not working".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "El autobús no se funciona".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            description_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "The bus is not working because it broke".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "El autobús no se funciona por que se fuera".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            tts_header_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "Problem: The bus is not working".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "Problema: El autobús no se funciona".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            tts_description_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "Problem: The bus is not working because it broke".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "Problema: El autobús no se funciona por que se fuera".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            severity_level: Some(SeverityLevel::Severe as i32),
            cause_detail: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "it broke".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "se fuera".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            effect_detail: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "No bus now".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "No hay un autobús ahora".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            // unused
            image: None,
            image_alternative_text: None,
        },
        ///////////////////////// id 2
        Alert {
            active_period: vec![
                TimeRange {
                    start: Some(1775079486),
                    end: Some(1775080486),
                },
                TimeRange {
                    start: Some(1775179486),
                    end: Some(1775180486),
                },
            ],
            informed_entity: vec![
                EntitySelector {
                    agency_id: Some("agency1".to_owned()),
                    route_id: Some("route1".to_owned()),
                    route_type: Some(1),
                    trip: Some(TripDescriptor {
                        trip_id: Some("trip1".to_owned()),
                        route_id: Some("trip1_route1".to_owned()),
                        direction_id: Some(0),
                        start_time: Some("20:00:00".to_owned()),
                        start_date: Some("20260311".to_owned()),
                        schedule_relationship: Some(
                            trip_descriptor::ScheduleRelationship::Canceled as i32,
                        ),
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: Some("stop1".to_owned()),
                    direction_id: Some(1),
                },
                EntitySelector {
                    agency_id: Some("agency2".to_owned()),
                    route_id: Some("route2".to_owned()),
                    route_type: Some(2),
                    trip: Some(TripDescriptor {
                        trip_id: Some("trip2".to_owned()),
                        route_id: Some("trip2_route2".to_owned()),
                        direction_id: Some(1),
                        start_time: Some("22:00:00".to_owned()),
                        start_date: Some("20260312".to_owned()),
                        schedule_relationship: Some(
                            trip_descriptor::ScheduleRelationship::Scheduled as i32,
                        ),
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: Some("stop2".to_owned()),
                    direction_id: Some(0),
                },
            ],
            cause: Some(Cause::Weather as i32),
            effect: Some(Effect::NoService as i32),
            url: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "https://example.com/test".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "https://example.com/test".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            header_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "The bus is not working".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "El autobús no se funciona".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            description_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "The bus is not working because it broke".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "El autobús no se funciona por que se fuera".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            tts_header_text: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "Problem: The bus is not working".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "Problema: El autobús no se funciona".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            tts_description_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "Problem: The bus is not working because it broke".to_owned(),
                    language: Some("en".to_owned()),
                }],
            }),
            severity_level: Some(SeverityLevel::Severe as i32),
            cause_detail: Some(TranslatedString {
                translation: vec![Translation {
                    text: "it broke".to_owned(),
                    language: Some("en".to_owned()),
                }],
            }),
            effect_detail: Some(TranslatedString {
                translation: vec![
                    Translation {
                        text: "No bus now".to_owned(),
                        language: Some("en".to_owned()),
                    },
                    Translation {
                        text: "No hay un autobús ahora".to_owned(),
                        language: Some("es".to_owned()),
                    },
                ],
            }),
            // unused
            image: None,
            image_alternative_text: None,
        },
        ///////////////////////// id 3
        Alert {
            active_period: vec![
                TimeRange {
                    start: None,
                    end: None,
                },
                TimeRange {
                    start: None,
                    end: None,
                },
            ],
            informed_entity: vec![
                EntitySelector {
                    agency_id: None,
                    route_id: None,
                    route_type: None,
                    trip: Some(TripDescriptor {
                        trip_id: None,
                        route_id: None,
                        direction_id: None,
                        start_time: None,
                        start_date: None,
                        schedule_relationship: None,
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: None,
                    direction_id: None,
                },
                EntitySelector {
                    agency_id: None,
                    route_id: None,
                    route_type: None,
                    trip: Some(TripDescriptor {
                        trip_id: None,
                        route_id: None,
                        direction_id: None,
                        start_time: None,
                        start_date: None,
                        schedule_relationship: None,
                        modified_trip: None, // not used, experimental
                    }),
                    stop_id: None,
                    direction_id: None,
                },
            ],
            cause: None,
            effect: None,
            url: Some(TranslatedString {
                translation: vec![Translation {
                    text: "https://example.com/test".to_owned(),
                    language: None,
                }],
            }),
            header_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "The bus is not working".to_owned(),
                    language: None,
                }],
            }),
            description_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "The bus is not working because it broke".to_owned(),
                    language: None,
                }],
            }),
            tts_header_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "Problem: The bus is not working".to_owned(),
                    language: None,
                }],
            }),
            tts_description_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "Problem: The bus is not working because it broke".to_owned(),
                    language: None,
                }],
            }),
            severity_level: None,
            cause_detail: Some(TranslatedString {
                translation: vec![Translation {
                    text: "it broke".to_owned(),
                    language: None,
                }],
            }),
            effect_detail: Some(TranslatedString {
                translation: vec![Translation {
                    text: "No bus now".to_owned(),
                    language: None,
                }],
            }),
            // unused
            image: None,
            image_alternative_text: None,
        },
        ////////////////////////// id 4
        Alert {
            active_period: vec![],
            informed_entity: vec![],
            cause: None,
            effect: None,
            url: Some(TranslatedString {
                translation: vec![Translation {
                    text: "https://example.com/test".to_owned(),
                    language: None,
                }],
            }),
            header_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "The bus is not working".to_owned(),
                    language: None,
                }],
            }),
            description_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "The bus is not working because it broke".to_owned(),
                    language: None,
                }],
            }),
            tts_header_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "Problem: The bus is not working".to_owned(),
                    language: None,
                }],
            }),
            tts_description_text: Some(TranslatedString {
                translation: vec![Translation {
                    text: "Problem: The bus is not working because it broke".to_owned(),
                    language: None,
                }],
            }),
            severity_level: None,
            cause_detail: Some(TranslatedString {
                translation: vec![Translation {
                    text: "it broke".to_owned(),
                    language: None,
                }],
            }),
            effect_detail: Some(TranslatedString {
                translation: vec![Translation {
                    text: "No bus now".to_owned(),
                    language: None,
                }],
            }),
            // unused
            image: None,
            image_alternative_text: None,
        },
        ////////////////////////// id 5: everything is NA
        Alert {
            active_period: vec![],
            informed_entity: vec![],
            cause: None,
            effect: None,
            url: None,
            header_text: None,
            description_text: None,
            tts_header_text: None,
            tts_description_text: None,
            severity_level: None,
            cause_detail: None,
            effect_detail: None,
            // unused
            image: None,
            image_alternative_text: None,
        },
    ];

    write_alerts(filename, alerts)?;

    Ok(())
}

extendr_module! {
    mod alert_unwrapping;
    fn test_data_alert_unwrapping;
}

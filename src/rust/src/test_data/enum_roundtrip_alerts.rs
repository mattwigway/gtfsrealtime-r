use extendr_api::extendr_module;
use extendr_api::prelude::*;
use std::string::ToString;
use strum::VariantArray;

use crate::test_data::enum_roundtrip::get_or_none;
use crate::test_data::write_alerts;
use crate::transit_realtime::{
    alert::{Cause, Effect, SeverityLevel},
    trip_descriptor,
    Alert, EntitySelector, TimeRange, TripDescriptor,
};

#[extendr]
pub fn test_data_enum_roundtrip_alerts(filename: &str) -> Result<List> {
    let n = [
        Cause::VARIANTS.len(),
        Effect::VARIANTS.len(),
        SeverityLevel::VARIANTS.len(),
        trip_descriptor::ScheduleRelationship::VARIANTS.len(),
    ]
    .iter()
    .max()
    .unwrap()
        + 1;

    let values: Vec<Alert> = (0..n)
        .into_iter()
        .map(|i| {
            let mut alert = Alert {
                active_period: vec![TimeRange {
                    start: Some(1774957578),
                    end: Some(1774967578),
                }],
                informed_entity: vec![EntitySelector {
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
                        modified_trip: None,
                    }),
                    stop_id: None,
                    direction_id: None,
                }],
                cause: None,
                effect: None,
                url: None,
                header_text: None,
                description_text: None,
                tts_header_text: None,
                tts_description_text: None,
                severity_level: None,
                image: None,
                image_alternative_text: None,
                cause_detail: None,
                effect_detail: None,
            };

            match get_or_none::<trip_descriptor::ScheduleRelationship>(i) {
                Some(c) => alert.informed_entity[0]
                    .trip
                    .as_mut()
                    .unwrap()
                    .set_schedule_relationship(c),
                None => (),
            };

            match get_or_none::<Cause>(i) {
                Some(c) => alert.set_cause(c),
                None => (),
            };

            match get_or_none::<Effect>(i) {
                Some(c) => alert.set_effect(c),
                None => (),
            };

            match get_or_none::<SeverityLevel>(i) {
                Some(c) => alert.set_severity_level(c),
                None => (),
            };

            alert
        })
        .collect();

    let l = list!(
        trip_schedule_relationship = values
            .iter()
            .map(|a| {
                match a.informed_entity[0]
                    .trip
                    .as_ref()
                    .unwrap()
                    .schedule_relationship
                {
                    Some(_) => Some(
                        a.informed_entity[0]
                            .trip
                            .as_ref()
                            .unwrap()
                            .schedule_relationship()
                            .as_str_name(),
                    ),
                    None => None,
                }
            })
            .collect::<Vec<Option<&str>>>(),
        cause = values
            .iter()
            .map(|a| match a.cause {
                Some(_) => Some(a.cause().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        effect = values
            .iter()
            .map(|a| match a.effect {
                Some(_) => Some(a.effect().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        severity_level = values
            .iter()
            .map(|a| match a.severity_level {
                Some(_) => Some(a.severity_level().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>()
    );

    write_alerts(filename, values)?;

    Ok(l)
}

extendr_module! {
    mod enum_roundtrip_alerts;
    fn test_data_enum_roundtrip_alerts;
}

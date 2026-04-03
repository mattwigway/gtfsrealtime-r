use std::{collections::HashSet, hash::Hash};

use extendr_api::prelude::*;

/// An IdDeduplicator is used to ensure that GTFS-realtime IDs are unique
/// Particularly for alerts and trip updates this is very important because it
/// is how we put together rows from the same alert/update.
pub struct IdDeduplicator {
    set: HashSet<String>,
}

impl IdDeduplicator {
    /// deduplicate_id will check to see if `id` has been used before.
    /// if it has not, it will add it to the set of used IDs, and return it.
    /// If it has, it will deduplicate it and issue a warning.
    pub fn deduplicate_id(&mut self, id: String) -> String {
        if self.set.contains(&id) {
            // it is duplicated
            let mut dupecount = 1;
            loop {
                // so if there is an duplicated id "a" and an id in the input "a_duplicated_1",
                // this will convert the second "a" to "a_duplicated_1", and then will convert
                // a_duplicated_1 to a_duplicated_1_duplicated_1 even if it is not duplicated.
                // This is such a corner case I'm not worried about about, especially since it
                // will still lead to unique IDs and will only come up in feeds that are broken
                // anyhow.
                let new_id = format!("{}_duplicated_{}", id, dupecount);

                if self.set.contains(&new_id) {
                    dupecount += 1;
                } else {
                    // I checked and this is safe - it doesn't just naively interpolate the strings, it
                    // creates an Robj from them, and then replaces it with param.0, param.1, etc.
                    // this should not fail and it's just a warning anyways, just ignore if it fails
                    // rather than dealing with the result further up.
                    let _ = R!(r#"
                        cli::cli_warn(c(
                            "!" = paste("ID", {{ id }}, "is duplicated. Replacing with", {{ new_id.clone() }})
                        ))
                    "#);

                    self.set.insert(new_id.clone());

                    return new_id;
                }
            }
        } else {
            self.set.insert(id.clone());
            return id;
        }
    }

    pub fn new() -> IdDeduplicator {
        IdDeduplicator {
            set: HashSet::new(),
        }
    }
}

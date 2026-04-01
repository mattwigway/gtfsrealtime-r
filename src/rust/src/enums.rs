use extendr_api::prelude::*;
use strum::VariantArray;
use std::any::type_name;

// this is a trait that we add to all of the Prost generated code that just wraps as_str_name()
// so that it is provided through a trait.
pub trait AsStrName {
    fn as_str_name_t(&self) -> &'static str;
}

pub fn enum_to_list<T: VariantArray + Copy + AsStrName>() -> Result<List>
    where &'static str: From<T>, i32: From<T> {
    let labels: Vec<&'static str> = T::VARIANTS.iter().map(|&x| x.as_str_name_t()).collect();
    let levels: Vec<i32> = T::VARIANTS.iter().map(|&x| x.into()).collect();

    Ok(list!(
        levels = levels,
        labels = labels,
        typ = type_name::<T>()
    ))
}
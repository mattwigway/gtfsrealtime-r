use extendr_api::prelude::*;
use strum::VariantArray;
use std::any::type_name;

pub fn enum_to_list<T: VariantArray + Copy>() -> Result<List>
    where &'static str: From<T>, i32: From<T> {
    let labels: Vec<&'static str> = T::VARIANTS.iter().map(|&x| x.into()).collect();
    let levels: Vec<i32> = T::VARIANTS.iter().map(|&x| x.into()).collect();

    Ok(list!(
        levels = levels,
        labels = labels,
        typ = type_name::<T>()
    ))
}
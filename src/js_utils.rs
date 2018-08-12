use chrono::{DateTime, FixedOffset};
use stdweb::unstable::TryInto;

pub fn random() -> f64 {
    (js! {return Math.random()}).try_into().unwrap()
}

pub fn get_date() -> DateTime<FixedOffset> {
    let iso_str: String = (js! { return (new Date()).toISOString(); }).try_into().unwrap();
    DateTime::parse_from_rfc3339(iso_str.as_ref()).unwrap()
}
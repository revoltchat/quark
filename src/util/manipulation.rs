use std::collections::HashMap;

use serde::Serialize;

/// Removes null keys from an arbitrary object
///
/// This is by no means a good solution but it is the easiest.
/// TODO: Pass-through annotations on keys through to Partial (in optional struct library)
pub fn remove_null_keys<T: Serialize>(t: &T) -> HashMap<String, serde_json::Value> {
    let v: String = serde_json::to_string(t).unwrap();
    let v: HashMap<String, serde_json::Value> = serde_json::from_str(&v).unwrap();
    v.into_iter().filter(|(_k, v)| !v.is_null()).collect()
}

/// Prefix keys on an arbitrary object
pub fn prefix_keys<T: Serialize>(t: &T, prefix: &str) -> HashMap<String, serde_json::Value> {
    let v: String = serde_json::to_string(t).unwrap();
    let v: HashMap<String, serde_json::Value> = serde_json::from_str(&v).unwrap();
    v.into_iter()
        .filter(|(_k, v)| !v.is_null())
        .map(|(k, v)| (prefix.to_owned() + &k, v))
        .collect()
}

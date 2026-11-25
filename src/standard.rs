//! The codes ASTM F3411-22a and ASD-STAN EN 4709-002 name for the fields a
//! ground station broadcasts. Copied by hand from the crate `remoteid` 0.4.0,
//! Copyright (c) 2026 Matthew Busel, MIT (`LICENSE-remoteid.txt`), so that
//! nothing is downloaded to build this plugin and every line can be read here.

/// One coded field: where it stands in the identity, what it is called, and
/// the meaning of each code from 0 on.
pub struct Field {
    pub group: &'static str,
    pub key: &'static str,
    pub label: &'static str,
    pub names: &'static [&'static str],
}

impl Field {
    pub fn name(&self, code: i64) -> Option<&'static str> {
        self.names.get(usize::try_from(code).ok()?).copied()
    }
}

const fn f(group: &'static str, key: &'static str, label: &'static str, names: &'static [&'static str]) -> Field {
    Field {
        group,
        key,
        label,
        names,
    }
}

pub const FIELDS: [Field; 8] = [
    f(
        "basic_id",
        "id_type",
        "ID type",
        &[
            "None",
            "Serial number",
            "CAA registration",
            "UTM assigned UUID",
            "Specific session ID",
        ],
    ),
    f(
        "basic_id",
        "ua_type",
        "Aircraft type",
        &[
            "Not declared",
            "Aeroplane",
            "Helicopter or multirotor",
            "Gyroplane",
            "Hybrid lift",
            "Ornithopter",
            "Glider",
            "Kite",
            "Free balloon",
            "Captive balloon",
            "Airship",
            "Free fall or parachute",
            "Rocket",
            "Tethered powered aircraft",
            "Ground obstacle",
            "Other",
        ],
    ),
    // The Operator ID message has no sub type: 0 is its only code.
    f("operator_id", "operator_id_type", "Operator ID type", &["Operator ID"]),
    f(
        "self_id",
        "description_type",
        "Description type",
        &["Text", "Emergency", "Extended status"],
    ),
    f(
        "system",
        "operator_location_type",
        "Operator position",
        &["Takeoff point", "Live GNSS", "Fixed position"],
    ),
    f(
        "system",
        "classification_type",
        "Classification",
        &["Not declared", "European Union"],
    ),
    f(
        "system",
        "category_eu",
        "EU category",
        &["Not declared", "Open", "Specific", "Certified"],
    ),
    f(
        "system",
        "class_eu",
        "EU class",
        &["Not declared", "C0", "C1", "C2", "C3", "C4", "C5", "C6"],
    ),
];

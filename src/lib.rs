//! Remote ID Check: what this station sends as Remote ID, held against the
//! tables of the standard (`standard.rs`). It reads `GET /api/remoteid` and
//! changes nothing: not the identity, not the broadcast, not the aircraft.

pub mod standard;

use eframe::egui::{self, Pos2, Rect, Stroke, Vec2};
use serde_json::Value;

use corvus_api::i18n::{tr, tr_fmt};
use corvus_api::plugin::{Host, Info, Plugin, Reply};
use corvus_api::ui::kit::{self, Button, Variant};
use corvus_api::ui::message::{self, Tone};
use corvus_api::ui::style::{self, Weight};
use standard::FIELDS;

const NOTE: &str = "Compares the codes this station sends with the tables of ASTM F3411 and ASD-STAN. It reads only and changes nothing. It is no proof that the identity is valid.";

/// One field as it is sent, and what is to say about it.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub label: &'static str,
    /// What goes out, when the station answered with a code.
    pub sent: Option<i64>,
    /// What the standard calls it.
    pub name: Option<&'static str>,
}

impl Line {
    pub fn fine(&self) -> bool {
        self.name.is_some()
    }

    /// The problem in words, for the operator.
    fn problem(&self) -> Option<String> {
        if self.fine() {
            return None;
        }
        let label = tr(self.label).into_owned();
        Some(match self.sent {
            None => tr_fmt("{field}: the station gave no code.", &[("field", &label)]),
            Some(sent) => tr_fmt(
                "{field}: the standard names no code {a}.",
                &[("field", &label), ("a", &sent.to_string())],
            ),
        })
    }
}

/// Every coded field of the answer of `GET /api/remoteid`.
pub fn check(answer: &Value) -> Vec<Line> {
    let identity = &answer["identity"];
    FIELDS
        .iter()
        .map(|f| {
            let sent = identity.get(f.group).and_then(|g| g.get(f.key)).and_then(Value::as_i64);
            Line {
                label: f.label,
                sent,
                name: sent.and_then(|c| f.name(c)),
            }
        })
        .collect()
}

#[derive(Default)]
pub struct RemoteIdCheck {
    asked: Option<Reply>,
    answer: Option<Result<(Vec<Line>, bool), String>>,
    again: bool,
}

impl Plugin for RemoteIdCheck {
    fn info(&self) -> Info {
        Info {
            id: "remote-id-check",
            name: "Remote ID Check",
            icon: "id-card",
            description: "Compares the Remote ID codes this station sends with the tables of the standard",
            version: "1.0.0",
            order: 90,
            can_tab: false,
        }
    }

    fn open(&mut self, h: &mut Host) {
        self.asked = Some(h.get("/api/remoteid"));
    }

    fn tick(&mut self, h: &mut Host) {
        if let Some(r) = self.asked.as_ref().and_then(Reply::take) {
            self.asked = None;
            self.answer = Some(r.map(|v| (check(&v), v["identity"]["enabled"] == Value::Bool(true))));
        }
        if std::mem::take(&mut self.again) && self.asked.is_none() {
            self.asked = Some(h.get("/api/remoteid"));
        }
    }

    fn close(&mut self, _h: &mut Host) {
        self.asked = None;
        self.answer = None;
    }

    fn show(&mut self, ui: &mut egui::Ui, area: Rect, _h: &mut Host) -> f32 {
        let (x, w) = (area.left(), area.width());
        let mut y = area.top();
        let p = ui.painter().clone();
        let g = kit::wrapped_weight(ui, NOTE, 11.0, Weight::W400, style::text_3(), w, 1.5);
        p.galley(Pos2::new(x, y), g.clone(), style::text_3());
        y += g.size().y + 12.0;
        match &self.answer {
            None => y = message::show(ui, x, y, w, "Reading the identity\u{2026}", Tone::Plain),
            Some(Err(e)) => y = message::show(ui, x, y, w, e, Tone::Err),
            Some(Ok((lines, enabled))) => {
                if !enabled {
                    y = message::show(
                        ui,
                        x,
                        y,
                        w,
                        "The broadcast is off. These codes would be sent.",
                        Tone::Plain,
                    );
                    y += 4.0;
                }
                let problems: Vec<String> = lines.iter().filter_map(Line::problem).collect();
                if problems.is_empty() {
                    y = message::show(ui, x, y, w, "Every code is one the standard names.", Tone::Ok);
                }
                for t in &problems {
                    y = message::show(ui, x, y, w, t, Tone::Warn) + 6.0;
                }
                y += 10.0;
                for l in lines {
                    y = row(ui, x, y, w, l);
                }
            }
        }
        y += 12.0;
        let b = Button::new("Check again", Variant::Secondary)
            .icon("refresh-cw")
            .enabled(self.asked.is_none());
        let size = Vec2::new(b.width(ui), b.height());
        if b.show(ui, Rect::from_min_size(Pos2::new(x, y), size), "ridcheckagain") {
            self.again = true;
        }
        y + size.y - area.top()
    }
}

fn row(ui: &egui::Ui, x: f32, y: f32, w: f32, l: &Line) -> f32 {
    let p = ui.painter();
    let label = style::galley(ui, l.label, 12.0, Weight::W500, style::text_2(), 0.0);
    let color = if l.fine() { style::text_1() } else { style::warning() };
    let value = match (l.sent, l.name) {
        (Some(c), Some(n)) => format!("{c} \u{b7} {}", tr(n)),
        (Some(c), None) => c.to_string(),
        (None, _) => "\u{2014}".to_string(),
    };
    let value = kit::wrapped_weight(ui, &value, 12.0, Weight::W400, color, w * 0.6, 1.4);
    let h = label.size().y.max(value.size().y) + 14.0;
    p.galley(Pos2::new(x, y + 7.0), label, style::text_2());
    p.galley(Pos2::new(x + w * 0.4, y + 7.0), value, color);
    p.hline(
        egui::Rangef::new(x, x + w),
        y + h - 0.5,
        Stroke::new(1.0, style::border_soft()),
    );
    y + h
}

/// The language files of this plugin, for the translation layer.
pub const LANG: &[(&str, &str)] = &[
    ("de", include_str!("../lang/de.json")),
    ("fr", include_str!("../lang/fr.json")),
    ("dl", include_str!("../lang/dl.json")),
];

/// One instance of the plugin, for the registry.
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(RemoteIdCheck::default())
}

corvus_api::export_plugin!(plugin, LANG);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_details_the_store_lists_are_the_plugins_own() {
        let manifest = include_str!("../Cargo.toml").replace('\r', "");
        let i = RemoteIdCheck::default().info();
        for (key, want) in [
            ("name", i.name),
            ("description", i.description),
            ("icon", i.icon),
            ("version", i.version),
        ] {
            assert!(
                manifest.contains(&format!("\n{key} = \"{want}\"\n")),
                "[package.metadata.corvus] says something else than the plugin for {key}"
            );
        }
    }

    fn answer(class: i64) -> Value {
        json!({"identity": {
            "enabled": true,
            "basic_id": {"id_type": 1, "ua_type": 2, "uas_id": "x"},
            "operator_id": {"operator_id_type": 0},
            "self_id": {"description_type": 0},
            "system": {"operator_location_type": 0, "classification_type": 1, "category_eu": 1, "class_eu": class},
        }})
    }

    #[test]
    fn codes_the_standard_names_are_fine() {
        let lines = check(&answer(6));
        assert_eq!(lines.len(), 8);
        assert!(lines.iter().all(|l| l.fine() && l.problem().is_none()), "{lines:?}");
        assert_eq!(lines[7].name, Some("C5"));
    }

    #[test]
    fn a_code_without_a_name_is_said() {
        for class in [-1, 8, i64::MAX] {
            let l = check(&answer(class)).remove(7);
            assert_eq!(l.name, None);
            assert_eq!(
                l.problem().unwrap(),
                format!("EU class: the standard names no code {class}.")
            );
        }
    }

    /// `src/core/src/protocol/remote_id.rs` accepts these spans.
    #[test]
    fn the_tables_are_as_long_as_the_spans_of_the_station() {
        let have: Vec<(&str, usize)> = FIELDS.iter().map(|f| (f.key, f.names.len())).collect();
        assert_eq!(
            have,
            [
                ("id_type", 5),
                ("ua_type", 16),
                ("operator_id_type", 1),
                ("description_type", 3),
                ("operator_location_type", 3),
                ("classification_type", 2),
                ("category_eu", 4),
                ("class_eu", 8),
            ]
        );
    }

    #[test]
    fn an_answer_that_is_not_one_is_no_crash() {
        for v in [
            json!(null),
            json!([]),
            json!({"identity": 3}),
            json!({"identity": {"system": "x"}}),
        ] {
            let lines = check(&v);
            assert!(lines.iter().all(|l| l.sent.is_none() && !l.fine()));
            assert!(lines[0].problem().is_some());
        }
    }
}

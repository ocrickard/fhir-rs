#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Search<'a> {
    pub value: &'a Value,
}

impl Bundle_Search<'_> {
    /// Extensions for score
    pub fn _score(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_score") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When searching, the server's search ranking score for the entry.
    pub fn score(&self) -> Option<f64> {
        if let Some(val) = self.value.get("score") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Why this entry is in the result set - whether it's included as a match or
    /// because of an _include requirement, or to convey information or warning
    /// information about the search process.
    pub fn mode(&self) -> Option<Bundle_SearchMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(Bundle_SearchMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._score() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.score() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Bundle_SearchMode {
    Match,
    Include,
    Outcome,
}

impl Bundle_SearchMode {
    pub fn from_string(string: &str) -> Option<Bundle_SearchMode> {
        match string {
            "match" => Some(Bundle_SearchMode::Match),
            "include" => Some(Bundle_SearchMode::Include),
            "outcome" => Some(Bundle_SearchMode::Outcome),
            _ => None,
        }
    }
}

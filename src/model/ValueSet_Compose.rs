#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ValueSet_Include::ValueSet_Include;
use serde_json::value::Value;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Compose<'a> {
    pub value: &'a Value,
}

impl ValueSet_Compose<'_> {
    /// Extensions for inactive
    pub fn _inactive(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inactive") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Whether inactive codes - codes that are not approved for current use - are in
    /// the value set. If inactive = true, inactive codes are to be included in the
    /// expansion, if inactive = false, the inactive codes will not be included in the
    /// expansion. If absent, the behavior is determined by the implementation, or by
    /// the applicable $expand parameters (but generally, inactive codes would be
    /// expected to be included).
    pub fn inactive(&self) -> Option<bool> {
        if let Some(val) = self.value.get("inactive") {
            return Some(val.as_bool().unwrap());
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

    /// Include one or more codes from a code system or other value set(s).
    pub fn include(&self) -> Vec<ValueSet_Include> {
        self.value
            .get("include")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ValueSet_Include { value: e })
            .collect::<Vec<_>>()
    }

    /// Extensions for lockedDate
    pub fn _locked_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lockedDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The Locked Date is  the effective date that is used to determine the version of
    /// all referenced Code Systems and Value Set Definitions included in the compose
    /// that are not already tied to a specific version.
    pub fn locked_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lockedDate") {
            return Some(string);
        }
        return None;
    }

    /// Exclude one or more codes from the value set based on code system filters and/or
    /// other value sets.
    pub fn exclude(&self) -> Option<Vec<ValueSet_Include>> {
        if let Some(Value::Array(val)) = self.value.get("exclude") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Include { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._inactive() {
            _val.validate();
        }
        if let Some(_val) = self.inactive() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.include().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self._locked_date() {
            _val.validate();
        }
        if let Some(_val) = self.locked_date() {}
        if let Some(_val) = self.exclude() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

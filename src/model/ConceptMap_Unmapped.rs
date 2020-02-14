#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap_Unmapped<'a> {
    pub value: &'a Value,
}

impl ConceptMap_Unmapped<'_> {
    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The canonical reference to an additional ConceptMap resource instance to use for
    /// mapping if this ConceptMap resource contains no matching mapping for the source
    /// concept.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
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

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The fixed code to use when the mode = 'fixed'  - all unmapped codes are mapped
    /// to a single fixed code.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Defines which action to take if there is no match for the source concept in the
    /// target system designated for the group. One of 3 actions are possible: use the
    /// unmapped code (this is useful when doing a mapping between versions, and only a
    /// few codes have changed), use a fixed code (a default code), or alternatively, a
    /// reference to a different concept map can be provided (by canonical URL).
    pub fn mode(&self) -> Option<ConceptMap_UnmappedMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(ConceptMap_UnmappedMode::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.display() {}
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self._display() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.mode() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ConceptMap_UnmappedMode {
    Provided,
    Fixed,
    OtherMap,
}

impl ConceptMap_UnmappedMode {
    pub fn from_string(string: &str) -> Option<ConceptMap_UnmappedMode> {
        match string {
            "provided" => Some(ConceptMap_UnmappedMode::Provided),
            "fixed" => Some(ConceptMap_UnmappedMode::Fixed),
            "other-map" => Some(ConceptMap_UnmappedMode::OtherMap),
            _ => None,
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::value::Value;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_Handling<'a> {
    pub value: &'a Value,
}

impl SpecimenDefinition_Handling<'_> {
    /// It qualifies the interval of temperature, which characterizes an occurrence of
    /// handling. Conditions that are not related to temperature may be handled in the
    /// instruction element.
    pub fn temperature_qualifier(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("temperatureQualifier") {
            return Some(CodeableConcept { value: val });
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

    /// The temperature interval for this set of handling instructions.
    pub fn temperature_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("temperatureRange") {
            return Some(Range { value: val });
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

    /// Additional textual instructions for the preservation or transport of the
    /// specimen. For instance, 'Protect from light exposure'.
    pub fn instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instruction") {
            return Some(string);
        }
        return None;
    }

    /// The maximum time interval of preservation of the specimen with these conditions.
    pub fn max_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("maxDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Extensions for instruction
    pub fn _instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instruction") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.temperature_qualifier() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.temperature_range() {
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
        if let Some(_val) = self.instruction() {}
        if let Some(_val) = self.max_duration() {
            _val.validate();
        }
        if let Some(_val) = self._instruction() {
            _val.validate();
        }
        return true;
    }
}

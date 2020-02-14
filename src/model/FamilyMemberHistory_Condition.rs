#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::value::Value;

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.

#[derive(Debug)]
pub struct FamilyMemberHistory_Condition<'a> {
    pub value: &'a Value,
}

impl FamilyMemberHistory_Condition<'_> {
    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("onsetAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("onsetPeriod") {
            return Some(Period { value: val });
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

    /// The actual condition specified. Could be a coded condition (like MI or Diabetes)
    /// or a less specific string like 'cancer' depending on how much is known about the
    /// condition and the capabilities of the creating system.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    /// Indicates what happened following the condition.  If the condition resulted in
    /// death, deceased date is captured on the relation.
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for contributedToDeath
    pub fn _contributed_to_death(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contributedToDeath") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// This condition contributed to the cause of death of the related person. If
    /// contributedToDeath is not populated, then it is unknown.
    pub fn contributed_to_death(&self) -> Option<bool> {
        if let Some(val) = self.value.get("contributedToDeath") {
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

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetString") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for onsetString
    pub fn _onset_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An area where general notes can be placed about this specific condition.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("onsetRange") {
            return Some(Range { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.onset_age() {
            _val.validate();
        }
        if let Some(_val) = self.onset_period() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.code().validate();
        if let Some(_val) = self.outcome() {
            _val.validate();
        }
        if let Some(_val) = self._contributed_to_death() {
            _val.validate();
        }
        if let Some(_val) = self.contributed_to_death() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.onset_string() {}
        if let Some(_val) = self._onset_string() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.onset_range() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

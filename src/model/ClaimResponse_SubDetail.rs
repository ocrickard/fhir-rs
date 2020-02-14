#![allow(unused_imports, non_camel_case_types)]

use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_SubDetail<'a> {
    pub value: &'a Value,
}

impl ClaimResponse_SubDetail<'_> {
    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub fn note_number(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("noteNumber") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
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

    /// The adjudication results.
    pub fn adjudication(&self) -> Option<Vec<ClaimResponse_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Adjudication { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for subDetailSequence
    pub fn _sub_detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subDetailSequence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A number to uniquely reference the claim sub-detail entry.
    pub fn sub_detail_sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("subDetailSequence") {
            return Some(val.as_i64().unwrap());
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

    /// Extensions for noteNumber
    pub fn _note_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_noteNumber") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.note_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.adjudication() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._sub_detail_sequence() {
            _val.validate();
        }
        if let Some(_val) = self.sub_detail_sequence() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._note_number() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

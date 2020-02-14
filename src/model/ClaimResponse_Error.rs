#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Error<'a> {
    pub value: &'a Value,
}

impl ClaimResponse_Error<'_> {
    /// An error code, from a specified code system, which details why the claim could
    /// not be adjudicated.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    /// Extensions for itemSequence
    pub fn _item_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_itemSequence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for detailSequence
    pub fn _detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detailSequence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The sequence number of the detail within the line item submitted which contains
    /// the error. This value is omitted when the error occurs outside of the item
    /// structure.
    pub fn detail_sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("detailSequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The sequence number of the sub-detail within the detail within the line item
    /// submitted which contains the error. This value is omitted when the error occurs
    /// outside of the item structure.
    pub fn sub_detail_sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("subDetailSequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The sequence number of the line item submitted which contains the error. This
    /// value is omitted when the error occurs outside of the item structure.
    pub fn item_sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("itemSequence") {
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

    /// Extensions for subDetailSequence
    pub fn _sub_detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subDetailSequence") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        let _ = self.code().validate();
        if let Some(_val) = self._item_sequence() {
            _val.validate();
        }
        if let Some(_val) = self._detail_sequence() {
            _val.validate();
        }
        if let Some(_val) = self.detail_sequence() {}
        if let Some(_val) = self.sub_detail_sequence() {}
        if let Some(_val) = self.item_sequence() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._sub_detail_sequence() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}

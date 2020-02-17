#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Error<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse_Error<'_> {
    pub fn new(value: &Value) -> ClaimResponse_Error {
        ClaimResponse_Error {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for detailSequence
    pub fn _detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detailSequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for itemSequence
    pub fn _item_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_itemSequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subDetailSequence
    pub fn _sub_detail_sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subDetailSequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An error code, from a specified code system, which details why the claim could
    /// not be adjudicated.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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

    /// The sequence number of the line item submitted which contains the error. This
    /// value is omitted when the error occurs outside of the item structure.
    pub fn item_sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("itemSequence") {
            return Some(val.as_i64().unwrap());
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._detail_sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._item_sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sub_detail_sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.detail_sequence() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item_sequence() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sub_detail_sequence() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponse_ErrorBuilder {
    pub(crate) value: Value,
}

impl ClaimResponse_ErrorBuilder {
    pub fn build(&self) -> ClaimResponse_Error {
        ClaimResponse_Error {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse_Error) -> ClaimResponse_ErrorBuilder {
        ClaimResponse_ErrorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ClaimResponse_ErrorBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ClaimResponse_ErrorBuilder { value: __value };
    }

    pub fn _detail_sequence<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["_detailSequence"] = json!(val.value);
        return self;
    }

    pub fn _item_sequence<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["_itemSequence"] = json!(val.value);
        return self;
    }

    pub fn _sub_detail_sequence<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["_subDetailSequence"] = json!(val.value);
        return self;
    }

    pub fn detail_sequence<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["detailSequence"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item_sequence<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["itemSequence"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sub_detail_sequence<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_ErrorBuilder {
        self.value["subDetailSequence"] = json!(val);
        return self;
    }
}

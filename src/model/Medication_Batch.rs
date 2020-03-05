#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource is primarily used for the identification and definition of a
/// medication for the purposes of prescribing, dispensing, and administering a
/// medication as well as for making statements about medication use.

#[derive(Debug)]
pub struct Medication_Batch<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Medication_Batch<'_> {
    pub fn new(value: &Value) -> Medication_Batch {
        Medication_Batch {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for expirationDate
    pub fn _expiration_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expirationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lotNumber
    pub fn _lot_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lotNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When this specific batch of product will expire.
    pub fn expiration_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expirationDate") {
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

    /// The assigned lot number of a batch of the specified product.
    pub fn lot_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lotNumber") {
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expiration_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._lot_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expiration_date() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.lot_number() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Medication_BatchBuilder {
    pub(crate) value: Value,
}

impl Medication_BatchBuilder {
    pub fn build(&self) -> Medication_Batch {
        Medication_Batch {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Medication_Batch) -> Medication_BatchBuilder {
        Medication_BatchBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Medication_BatchBuilder {
        let mut __value: Value = json!({});
        return Medication_BatchBuilder { value: __value };
    }

    pub fn _expiration_date<'a>(&'a mut self, val: Element) -> &'a mut Medication_BatchBuilder {
        self.value["_expirationDate"] = json!(val.value);
        return self;
    }

    pub fn _lot_number<'a>(&'a mut self, val: Element) -> &'a mut Medication_BatchBuilder {
        self.value["_lotNumber"] = json!(val.value);
        return self;
    }

    pub fn expiration_date<'a>(&'a mut self, val: &str) -> &'a mut Medication_BatchBuilder {
        self.value["expirationDate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Medication_BatchBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Medication_BatchBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn lot_number<'a>(&'a mut self, val: &str) -> &'a mut Medication_BatchBuilder {
        self.value["lotNumber"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Medication_BatchBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

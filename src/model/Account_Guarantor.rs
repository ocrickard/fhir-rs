#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.

#[derive(Debug)]
pub struct Account_Guarantor<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Account_Guarantor<'_> {
    pub fn new(value: &Value) -> Account_Guarantor {
        Account_Guarantor {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for onHold
    pub fn _on_hold(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onHold") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// A guarantor may be placed on credit hold or otherwise have their role
    /// temporarily suspended.
    pub fn on_hold(&self) -> Option<bool> {
        if let Some(val) = self.value.get("onHold") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The entity who is responsible.
    pub fn party(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["party"]),
        }
    }

    /// The timeframe during which the guarantor accepts responsibility for the account.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._on_hold() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.on_hold() {}
        if !self.party().validate() {
            return false;
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Account_GuarantorBuilder {
    pub(crate) value: Value,
}

impl Account_GuarantorBuilder {
    pub fn build(&self) -> Account_Guarantor {
        Account_Guarantor {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Account_Guarantor) -> Account_GuarantorBuilder {
        Account_GuarantorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(party: Reference) -> Account_GuarantorBuilder {
        let mut __value: Value = json!({});
        __value["party"] = json!(party.value);
        return Account_GuarantorBuilder { value: __value };
    }

    pub fn _on_hold<'a>(&'a mut self, val: Element) -> &'a mut Account_GuarantorBuilder {
        self.value["_onHold"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Account_GuarantorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Account_GuarantorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Account_GuarantorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_hold<'a>(&'a mut self, val: bool) -> &'a mut Account_GuarantorBuilder {
        self.value["onHold"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut Account_GuarantorBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }
}

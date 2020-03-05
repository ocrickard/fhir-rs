#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.

#[derive(Debug)]
pub struct Account_Coverage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Account_Coverage<'_> {
    pub fn new(value: &Value) -> Account_Coverage {
        Account_Coverage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The party(s) that contribute to payment (or part of) of the charges applied to
    /// this account (including self-pay).    A coverage may only be responsible for
    /// specific types of charges, and the sequence of the coverages in the account
    /// could be important when processing billing.
    pub fn coverage(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["coverage"]),
        }
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

    /// The priority of the coverage in the context of this account.
    pub fn priority(&self) -> Option<i64> {
        if let Some(val) = self.value.get("priority") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._priority() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.coverage().validate() {
            return false;
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
        if let Some(_val) = self.priority() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Account_CoverageBuilder {
    pub(crate) value: Value,
}

impl Account_CoverageBuilder {
    pub fn build(&self) -> Account_Coverage {
        Account_Coverage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Account_Coverage) -> Account_CoverageBuilder {
        Account_CoverageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(coverage: Reference) -> Account_CoverageBuilder {
        let mut __value: Value = json!({});
        __value["coverage"] = json!(coverage.value);
        return Account_CoverageBuilder { value: __value };
    }

    pub fn _priority<'a>(&'a mut self, val: Element) -> &'a mut Account_CoverageBuilder {
        self.value["_priority"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Account_CoverageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Account_CoverageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Account_CoverageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: i64) -> &'a mut Account_CoverageBuilder {
        self.value["priority"] = json!(val);
        return self;
    }
}

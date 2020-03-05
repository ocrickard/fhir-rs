#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Contact<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_Contact<'_> {
    pub fn new(value: &Value) -> InsurancePlan_Contact {
        InsurancePlan_Contact {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Visiting or postal addresses for the contact.
    pub fn address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("address") {
            return Some(Address {
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

    /// A name associated with the contact.
    pub fn name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("name") {
            return Some(HumanName {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates a purpose for which the contact can be reached.
    pub fn purpose(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("purpose") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A contact detail (e.g. a telephone number or an email address) by which the
    /// party may be contacted.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.address() {
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
        if let Some(_val) = self.name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.purpose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.telecom() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct InsurancePlan_ContactBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_ContactBuilder {
    pub fn build(&self) -> InsurancePlan_Contact {
        InsurancePlan_Contact {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_Contact) -> InsurancePlan_ContactBuilder {
        InsurancePlan_ContactBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> InsurancePlan_ContactBuilder {
        let mut __value: Value = json!({});
        return InsurancePlan_ContactBuilder { value: __value };
    }

    pub fn address<'a>(&'a mut self, val: Address) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["address"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: HumanName) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["name"] = json!(val.value);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: CodeableConcept) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["purpose"] = json!(val.value);
        return self;
    }

    pub fn telecom<'a>(
        &'a mut self,
        val: Vec<ContactPoint>,
    ) -> &'a mut InsurancePlan_ContactBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

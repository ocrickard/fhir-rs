#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient_Contact<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Patient_Contact<'_> {
    pub fn new(value: &Value) -> Patient_Contact {
        Patient_Contact {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Address for the contact person.
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

    /// Administrative Gender - the gender that the contact person is considered to have
    /// for administration and record keeping purposes.
    pub fn gender(&self) -> Option<Patient_ContactGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(Patient_ContactGender::from_string(&val).unwrap());
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

    /// A name associated with the contact person.
    pub fn name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("name") {
            return Some(HumanName {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Organization on behalf of which the contact is acting or for which the contact
    /// is working.
    pub fn organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("organization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period during which this contact person or organization is valid to be
    /// contacted relating to this patient.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The nature of the relationship between the patient and the contact person.
    pub fn relationship(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("relationship") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A contact detail for the person, e.g. a telephone number or an email address.
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
        if let Some(_val) = self._gender() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.gender() {}
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
        if let Some(_val) = self.organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct Patient_ContactBuilder {
    pub(crate) value: Value,
}

impl Patient_ContactBuilder {
    pub fn build(&self) -> Patient_Contact {
        Patient_Contact {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Patient_Contact) -> Patient_ContactBuilder {
        Patient_ContactBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Patient_ContactBuilder {
        let mut __value: Value = json!({});
        return Patient_ContactBuilder { value: __value };
    }

    pub fn _gender<'a>(&'a mut self, val: Element) -> &'a mut Patient_ContactBuilder {
        self.value["_gender"] = json!(val.value);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: Address) -> &'a mut Patient_ContactBuilder {
        self.value["address"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Patient_ContactBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(&'a mut self, val: Patient_ContactGender) -> &'a mut Patient_ContactBuilder {
        self.value["gender"] = json!(val.to_string());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Patient_ContactBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Patient_ContactBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: HumanName) -> &'a mut Patient_ContactBuilder {
        self.value["name"] = json!(val.value);
        return self;
    }

    pub fn organization<'a>(&'a mut self, val: Reference) -> &'a mut Patient_ContactBuilder {
        self.value["organization"] = json!(val.value);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut Patient_ContactBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn relationship<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Patient_ContactBuilder {
        self.value["relationship"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut Patient_ContactBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum Patient_ContactGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl Patient_ContactGender {
    pub fn from_string(string: &str) -> Option<Patient_ContactGender> {
        match string {
            "male" => Some(Patient_ContactGender::Male),
            "female" => Some(Patient_ContactGender::Female),
            "other" => Some(Patient_ContactGender::Other),
            "unknown" => Some(Patient_ContactGender::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Patient_ContactGender::Male => "male".to_string(),
            Patient_ContactGender::Female => "female".to_string(),
            Patient_ContactGender::Other => "other".to_string(),
            Patient_ContactGender::Unknown => "unknown".to_string(),
        }
    }
}

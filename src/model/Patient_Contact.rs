#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient_Contact<'a> {
    pub value: &'a Value,
}

impl Patient_Contact<'_> {
    /// Administrative Gender - the gender that the contact person is considered to have
    /// for administration and record keeping purposes.
    pub fn gender(&self) -> Option<Patient_ContactGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(Patient_ContactGender::from_string(&val).unwrap());
        }
        return None;
    }

    /// A contact detail for the person, e.g. a telephone number or an email address.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Organization on behalf of which the contact is acting or for which the contact
    /// is working.
    pub fn organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("organization") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Address for the contact person.
    pub fn address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("address") {
            return Some(Address { value: val });
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

    /// The nature of the relationship between the patient and the contact person.
    pub fn relationship(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("relationship") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The period during which this contact person or organization is valid to be
    /// contacted relating to this patient.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A name associated with the contact person.
    pub fn name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("name") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.gender() {}
        if let Some(_val) = self.telecom() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._gender() {
            _val.validate();
        }
        if let Some(_val) = self.organization() {
            _val.validate();
        }
        if let Some(_val) = self.address() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.relationship() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.name() {
            _val.validate();
        }
        return true;
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
}

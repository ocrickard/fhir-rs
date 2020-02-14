#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use serde_json::value::Value;

/// A formally or informally recognized grouping of people or organizations formed
/// for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.

#[derive(Debug)]
pub struct Organization_Contact<'a> {
    pub value: &'a Value,
}

impl Organization_Contact<'_> {
    /// A contact detail (e.g. a telephone number or an email address) by which the
    /// party may be contacted.
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

    /// Visiting or postal addresses for the contact.
    pub fn address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("address") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// Indicates a purpose for which the contact can be reached.
    pub fn purpose(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("purpose") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A name associated with the contact.
    pub fn name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("name") {
            return Some(HumanName { value: val });
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
        if let Some(_val) = self.telecom() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.address() {
            _val.validate();
        }
        if let Some(_val) = self.purpose() {
            _val.validate();
        }
        if let Some(_val) = self.name() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

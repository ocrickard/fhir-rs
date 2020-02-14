#![allow(unused_imports, non_camel_case_types)]

use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A contributor to the content of a knowledge asset, including authors, editors,
/// reviewers, and endorsers.

#[derive(Debug)]
pub struct Contributor<'a> {
    pub value: &'a Value,
}

impl Contributor<'_> {
    /// The name of the individual or organization responsible for the contribution.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// contributor.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of contributor.
    pub fn fhir_type(&self) -> Option<ContributorType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(ContributorType::from_string(&val).unwrap());
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.id() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ContributorType {
    Author,
    Editor,
    Reviewer,
    Endorser,
}

impl ContributorType {
    pub fn from_string(string: &str) -> Option<ContributorType> {
        match string {
            "author" => Some(ContributorType::Author),
            "editor" => Some(ContributorType::Editor),
            "reviewer" => Some(ContributorType::Reviewer),
            "endorser" => Some(ContributorType::Endorser),
            _ => None,
        }
    }
}

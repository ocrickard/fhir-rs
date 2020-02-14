#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Demographics and administrative information about a person independent of a
/// specific health-related context.

#[derive(Debug)]
pub struct Person_Link<'a> {
    pub value: &'a Value,
}

impl Person_Link<'_> {
    /// The resource to which this actual person is associated.
    pub fn target(&self) -> Reference {
        Reference {
            value: &self.value["target"],
        }
    }

    /// Level of assurance that this link is associated with the target resource.
    pub fn assurance(&self) -> Option<Person_LinkAssurance> {
        if let Some(Value::String(val)) = self.value.get("assurance") {
            return Some(Person_LinkAssurance::from_string(&val).unwrap());
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

    /// Extensions for assurance
    pub fn _assurance(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_assurance") {
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
        let _ = self.target().validate();
        if let Some(_val) = self.assurance() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._assurance() {
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

#[derive(Debug)]
pub enum Person_LinkAssurance {
    Level1,
    Level2,
    Level3,
    Level4,
}

impl Person_LinkAssurance {
    pub fn from_string(string: &str) -> Option<Person_LinkAssurance> {
        match string {
            "level1" => Some(Person_LinkAssurance::Level1),
            "level2" => Some(Person_LinkAssurance::Level2),
            "level3" => Some(Person_LinkAssurance::Level3),
            "level4" => Some(Person_LinkAssurance::Level4),
            _ => None,
        }
    }
}

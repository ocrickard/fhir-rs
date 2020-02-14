#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// An identifier - identifies some entity uniquely and unambiguously. Typically
/// this is used for business identifiers.

#[derive(Debug)]
pub struct Identifier<'a> {
    pub value: &'a Value,
}

impl Identifier<'_> {
    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
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

    /// Establishes the namespace for the value - that is, a URL that describes a set
    /// values that are unique.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// Time period during which identifier is/was valid for use.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Organization that issued/manages the identifier.
    pub fn assigner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("assigner") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The portion of the identifier typically relevant to the user and which is unique
    /// within the context of the system.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
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

    /// The purpose of this identifier.
    pub fn fhir_use(&self) -> Option<IdentifierUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(IdentifierUse::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A coded type for the identifier that can be used to determine which identifier
    /// to use for a specific purpose.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._system() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self._use() {
            _val.validate();
        }
        if let Some(_val) = self.assigner() {
            _val.validate();
        }
        if let Some(_val) = self.value() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.fhir_use() {}
        if let Some(_val) = self._value() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum IdentifierUse {
    Usual,
    Official,
    Temp,
    Secondary,
    Old,
}

impl IdentifierUse {
    pub fn from_string(string: &str) -> Option<IdentifierUse> {
        match string {
            "usual" => Some(IdentifierUse::Usual),
            "official" => Some(IdentifierUse::Official),
            "temp" => Some(IdentifierUse::Temp),
            "secondary" => Some(IdentifierUse::Secondary),
            "old" => Some(IdentifierUse::Old),
            _ => None,
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A set of healthcare-related information that is assembled together into a single
/// logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to who
/// is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).

#[derive(Debug)]
pub struct Composition_Attester<'a> {
    pub value: &'a Value,
}

impl Composition_Attester<'_> {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for time
    pub fn _time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_time") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// When the composition was attested by the party.
    pub fn time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("time") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Who attested the composition in the specified way.
    pub fn party(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("party") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The type of attestation the authenticator offers.
    pub fn mode(&self) -> Option<Composition_AttesterMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(Composition_AttesterMode::from_string(&val).unwrap());
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
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._time() {
            _val.validate();
        }
        if let Some(_val) = self.time() {}
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        if let Some(_val) = self.party() {
            _val.validate();
        }
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Composition_AttesterMode {
    Personal,
    Professional,
    Legal,
    Official,
}

impl Composition_AttesterMode {
    pub fn from_string(string: &str) -> Option<Composition_AttesterMode> {
        match string {
            "personal" => Some(Composition_AttesterMode::Personal),
            "professional" => Some(Composition_AttesterMode::Professional),
            "legal" => Some(Composition_AttesterMode::Legal),
            "official" => Some(Composition_AttesterMode::Official),
            _ => None,
        }
    }
}

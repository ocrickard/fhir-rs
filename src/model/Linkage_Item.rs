#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Identifies two or more records (resource instances) that refer to the same real-
/// world "occurrence".

#[derive(Debug)]
pub struct Linkage_Item<'a> {
    pub value: &'a Value,
}

impl Linkage_Item<'_> {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Distinguishes which item is "source of truth" (if any) and which items are no
    /// longer considered to be current representations.
    pub fn fhir_type(&self) -> Option<Linkage_ItemType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Linkage_ItemType::from_string(&val).unwrap());
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

    /// The resource instance being linked as part of the group.
    pub fn resource(&self) -> Reference {
        Reference {
            value: &self.value["resource"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.resource().validate();
        return true;
    }
}

#[derive(Debug)]
pub enum Linkage_ItemType {
    Source,
    Alternate,
    Historical,
}

impl Linkage_ItemType {
    pub fn from_string(string: &str) -> Option<Linkage_ItemType> {
        match string {
            "source" => Some(Linkage_ItemType::Source),
            "alternate" => Some(Linkage_ItemType::Alternate),
            "historical" => Some(Linkage_ItemType::Historical),
            _ => None,
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_Storage<'a> {
    pub value: &'a Value,
}

impl BiologicallyDerivedProduct_Storage<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Description of storage.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Storage temperature.
    pub fn temperature(&self) -> Option<f64> {
        if let Some(val) = self.value.get("temperature") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for temperature
    pub fn _temperature(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_temperature") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for scale
    pub fn _scale(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_scale") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Temperature scale used.
    pub fn scale(&self) -> Option<BiologicallyDerivedProduct_StorageScale> {
        if let Some(Value::String(val)) = self.value.get("scale") {
            return Some(BiologicallyDerivedProduct_StorageScale::from_string(&val).unwrap());
        }
        return None;
    }

    /// Storage timeperiod.
    pub fn duration(&self) -> Option<Period> {
        if let Some(val) = self.value.get("duration") {
            return Some(Period { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.temperature() {}
        if let Some(_val) = self._temperature() {
            _val.validate();
        }
        if let Some(_val) = self._scale() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.scale() {}
        if let Some(_val) = self.duration() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum BiologicallyDerivedProduct_StorageScale {
    Farenheit,
    Celsius,
    Kelvin,
}

impl BiologicallyDerivedProduct_StorageScale {
    pub fn from_string(string: &str) -> Option<BiologicallyDerivedProduct_StorageScale> {
        match string {
            "farenheit" => Some(BiologicallyDerivedProduct_StorageScale::Farenheit),
            "celsius" => Some(BiologicallyDerivedProduct_StorageScale::Celsius),
            "kelvin" => Some(BiologicallyDerivedProduct_StorageScale::Kelvin),
            _ => None,
        }
    }
}

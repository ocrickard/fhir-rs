#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.

#[derive(Debug)]
pub struct VisionPrescription_Prism<'a> {
    pub value: &'a Value,
}

impl VisionPrescription_Prism<'_> {
    /// Extensions for amount
    pub fn _amount(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amount") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The relative base, or reference lens edge, for the prism.
    pub fn base(&self) -> Option<VisionPrescription_PrismBase> {
        if let Some(Value::String(val)) = self.value.get("base") {
            return Some(VisionPrescription_PrismBase::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for base
    pub fn _base(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_base") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Amount of prism to compensate for eye alignment in fractional units.
    pub fn amount(&self) -> Option<f64> {
        if let Some(val) = self.value.get("amount") {
            return Some(val.as_f64().unwrap());
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
        if let Some(_val) = self._amount() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.base() {}
        if let Some(_val) = self._base() {
            _val.validate();
        }
        if let Some(_val) = self.amount() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum VisionPrescription_PrismBase {
    Up,
    Down,
    In,
    Out,
}

impl VisionPrescription_PrismBase {
    pub fn from_string(string: &str) -> Option<VisionPrescription_PrismBase> {
        match string {
            "up" => Some(VisionPrescription_PrismBase::Up),
            "down" => Some(VisionPrescription_PrismBase::Down),
            "in" => Some(VisionPrescription_PrismBase::In),
            "out" => Some(VisionPrescription_PrismBase::Out),
            _ => None,
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_UdiDeviceIdentifier<'a> {
    pub value: &'a Value,
}

impl DeviceDefinition_UdiDeviceIdentifier<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The organization that assigns the identifier algorithm.
    pub fn issuer(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issuer") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for issuer
    pub fn _issuer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issuer") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for jurisdiction
    pub fn _jurisdiction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_jurisdiction") {
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

    /// The identifier that is to be associated with every Device that references this
    /// DeviceDefintiion for the issuer and jurisdication porvided in the
    /// DeviceDefinition.udiDeviceIdentifier.
    pub fn device_identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deviceIdentifier") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for deviceIdentifier
    pub fn _device_identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deviceIdentifier") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The jurisdiction to which the deviceIdentifier applies.
    pub fn jurisdiction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("jurisdiction") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.issuer() {}
        if let Some(_val) = self._issuer() {
            _val.validate();
        }
        if let Some(_val) = self._jurisdiction() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.device_identifier() {}
        if let Some(_val) = self._device_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {}
        return true;
    }
}

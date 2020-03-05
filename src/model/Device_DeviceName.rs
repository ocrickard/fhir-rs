#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.

#[derive(Debug)]
pub struct Device_DeviceName<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Device_DeviceName<'_> {
    pub fn new(value: &Value) -> Device_DeviceName {
        Device_DeviceName {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the device.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The type of deviceName.  UDILabelName | UserFriendlyName | PatientReportedName |
    /// ManufactureDeviceName | ModelName.
    pub fn fhir_type(&self) -> Option<Device_DeviceNameType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Device_DeviceNameType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Device_DeviceNameBuilder {
    pub(crate) value: Value,
}

impl Device_DeviceNameBuilder {
    pub fn build(&self) -> Device_DeviceName {
        Device_DeviceName {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Device_DeviceName) -> Device_DeviceNameBuilder {
        Device_DeviceNameBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Device_DeviceNameBuilder {
        let mut __value: Value = json!({});
        return Device_DeviceNameBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut Device_DeviceNameBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut Device_DeviceNameBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Device_DeviceNameBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Device_DeviceNameBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Device_DeviceNameBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut Device_DeviceNameBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Device_DeviceNameType,
    ) -> &'a mut Device_DeviceNameBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Device_DeviceNameType {
    UdiLabelName,
    UserFriendlyName,
    PatientReportedName,
    ManufacturerName,
    ModelName,
    Other,
}

impl Device_DeviceNameType {
    pub fn from_string(string: &str) -> Option<Device_DeviceNameType> {
        match string {
            "udi-label-name" => Some(Device_DeviceNameType::UdiLabelName),
            "user-friendly-name" => Some(Device_DeviceNameType::UserFriendlyName),
            "patient-reported-name" => Some(Device_DeviceNameType::PatientReportedName),
            "manufacturer-name" => Some(Device_DeviceNameType::ManufacturerName),
            "model-name" => Some(Device_DeviceNameType::ModelName),
            "other" => Some(Device_DeviceNameType::Other),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Device_DeviceNameType::UdiLabelName => "udi-label-name".to_string(),
            Device_DeviceNameType::UserFriendlyName => "user-friendly-name".to_string(),
            Device_DeviceNameType::PatientReportedName => "patient-reported-name".to_string(),
            Device_DeviceNameType::ManufacturerName => "manufacturer-name".to_string(),
            Device_DeviceNameType::ModelName => "model-name".to_string(),
            Device_DeviceNameType::Other => "other".to_string(),
        }
    }
}

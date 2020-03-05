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
pub struct Device_UdiCarrier<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Device_UdiCarrier<'_> {
    pub fn new(value: &Value) -> Device_UdiCarrier {
        Device_UdiCarrier {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for carrierAIDC
    pub fn _carrier_a_i_d_c(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_carrierAIDC") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for carrierHRF
    pub fn _carrier_h_r_f(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_carrierHRF") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deviceIdentifier
    pub fn _device_identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deviceIdentifier") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for entryType
    pub fn _entry_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_entryType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issuer
    pub fn _issuer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issuer") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for jurisdiction
    pub fn _jurisdiction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_jurisdiction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The full UDI carrier of the Automatic Identification and Data Capture (AIDC)
    /// technology representation of the barcode string as printed on the packaging of
    /// the device - e.g., a barcode or RFID.   Because of limitations on character sets
    /// in XML and the need to round-trip JSON data through XML, AIDC Formats *SHALL* be
    /// base64 encoded.
    pub fn carrier_a_i_d_c(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("carrierAIDC") {
            return Some(string);
        }
        return None;
    }

    /// The full UDI carrier as the human readable form (HRF) representation of the
    /// barcode string as printed on the packaging of the device.
    pub fn carrier_h_r_f(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("carrierHRF") {
            return Some(string);
        }
        return None;
    }

    /// The device identifier (DI) is a mandatory, fixed portion of a UDI that
    /// identifies the labeler and the specific version or model of a device.
    pub fn device_identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deviceIdentifier") {
            return Some(string);
        }
        return None;
    }

    /// A coded entry to indicate how the data was entered.
    pub fn entry_type(&self) -> Option<Device_UdiCarrierEntryType> {
        if let Some(Value::String(val)) = self.value.get("entryType") {
            return Some(Device_UdiCarrierEntryType::from_string(&val).unwrap());
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

    /// Organization that is charged with issuing UDIs for devices.  For example, the US
    /// FDA issuers include :  1) GS1:   http://hl7.org/fhir/NamingSystem/gs1-di,   2)
    /// HIBCC:  http://hl7.org/fhir/NamingSystem/hibcc-dI,   3) ICCBBA for blood
    /// containers:  http://hl7.org/fhir/NamingSystem/iccbba-blood-di,   4) ICCBA for
    /// other devices:  http://hl7.org/fhir/NamingSystem/iccbba-other-di.
    pub fn issuer(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issuer") {
            return Some(string);
        }
        return None;
    }

    /// The identity of the authoritative source for UDI generation within a
    /// jurisdiction.  All UDIs are globally unique within a single namespace with the
    /// appropriate repository uri as the system.  For example,  UDIs of devices managed
    /// in the U.S. by the FDA, the value is  http://hl7.org/fhir/NamingSystem/fda-udi.
    pub fn jurisdiction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("jurisdiction") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._carrier_a_i_d_c() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._carrier_h_r_f() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._device_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._entry_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issuer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._jurisdiction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.carrier_a_i_d_c() {}
        if let Some(_val) = self.carrier_h_r_f() {}
        if let Some(_val) = self.device_identifier() {}
        if let Some(_val) = self.entry_type() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.issuer() {}
        if let Some(_val) = self.jurisdiction() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Device_UdiCarrierBuilder {
    pub(crate) value: Value,
}

impl Device_UdiCarrierBuilder {
    pub fn build(&self) -> Device_UdiCarrier {
        Device_UdiCarrier {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Device_UdiCarrier) -> Device_UdiCarrierBuilder {
        Device_UdiCarrierBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Device_UdiCarrierBuilder {
        let mut __value: Value = json!({});
        return Device_UdiCarrierBuilder { value: __value };
    }

    pub fn _carrier_a_i_d_c<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_carrierAIDC"] = json!(val.value);
        return self;
    }

    pub fn _carrier_h_r_f<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_carrierHRF"] = json!(val.value);
        return self;
    }

    pub fn _device_identifier<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_deviceIdentifier"] = json!(val.value);
        return self;
    }

    pub fn _entry_type<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_entryType"] = json!(val.value);
        return self;
    }

    pub fn _issuer<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_issuer"] = json!(val.value);
        return self;
    }

    pub fn _jurisdiction<'a>(&'a mut self, val: Element) -> &'a mut Device_UdiCarrierBuilder {
        self.value["_jurisdiction"] = json!(val.value);
        return self;
    }

    pub fn carrier_a_i_d_c<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["carrierAIDC"] = json!(val);
        return self;
    }

    pub fn carrier_h_r_f<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["carrierHRF"] = json!(val);
        return self;
    }

    pub fn device_identifier<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["deviceIdentifier"] = json!(val);
        return self;
    }

    pub fn entry_type<'a>(
        &'a mut self,
        val: Device_UdiCarrierEntryType,
    ) -> &'a mut Device_UdiCarrierBuilder {
        self.value["entryType"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Device_UdiCarrierBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn issuer<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["issuer"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(&'a mut self, val: &str) -> &'a mut Device_UdiCarrierBuilder {
        self.value["jurisdiction"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Device_UdiCarrierBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum Device_UdiCarrierEntryType {
    Barcode,
    Rfid,
    Manual,
    Card,
    SelfReported,
    Unknown,
}

impl Device_UdiCarrierEntryType {
    pub fn from_string(string: &str) -> Option<Device_UdiCarrierEntryType> {
        match string {
            "barcode" => Some(Device_UdiCarrierEntryType::Barcode),
            "rfid" => Some(Device_UdiCarrierEntryType::Rfid),
            "manual" => Some(Device_UdiCarrierEntryType::Manual),
            "card" => Some(Device_UdiCarrierEntryType::Card),
            "self-reported" => Some(Device_UdiCarrierEntryType::SelfReported),
            "unknown" => Some(Device_UdiCarrierEntryType::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Device_UdiCarrierEntryType::Barcode => "barcode".to_string(),
            Device_UdiCarrierEntryType::Rfid => "rfid".to_string(),
            Device_UdiCarrierEntryType::Manual => "manual".to_string(),
            Device_UdiCarrierEntryType::Card => "card".to_string(),
            Device_UdiCarrierEntryType::SelfReported => "self-reported".to_string(),
            Device_UdiCarrierEntryType::Unknown => "unknown".to_string(),
        }
    }
}

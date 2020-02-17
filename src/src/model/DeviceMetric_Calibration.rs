#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a measurement, calculation or setting capability of a medical device.

#[derive(Debug)]
pub struct DeviceMetric_Calibration<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceMetric_Calibration<'_> {
    pub fn new(value: &Value) -> DeviceMetric_Calibration {
        DeviceMetric_Calibration {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for state
    pub fn _state(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_state") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for time
    pub fn _time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_time") {
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

    /// Describes the state of the calibration.
    pub fn state(&self) -> Option<DeviceMetric_CalibrationState> {
        if let Some(Value::String(val)) = self.value.get("state") {
            return Some(DeviceMetric_CalibrationState::from_string(&val).unwrap());
        }
        return None;
    }

    /// Describes the time last calibration has been performed.
    pub fn time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("time") {
            return Some(string);
        }
        return None;
    }

    /// Describes the type of the calibration method.
    pub fn fhir_type(&self) -> Option<DeviceMetric_CalibrationType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(DeviceMetric_CalibrationType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._time() {
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
        if let Some(_val) = self.state() {}
        if let Some(_val) = self.time() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceMetric_CalibrationBuilder {
    pub(crate) value: Value,
}

impl DeviceMetric_CalibrationBuilder {
    pub fn build(&self) -> DeviceMetric_Calibration {
        DeviceMetric_Calibration {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceMetric_Calibration) -> DeviceMetric_CalibrationBuilder {
        DeviceMetric_CalibrationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceMetric_CalibrationBuilder {
        let mut __value: Value = json!({});
        return DeviceMetric_CalibrationBuilder { value: __value };
    }

    pub fn _state<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["_state"] = json!(val.value);
        return self;
    }

    pub fn _time<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["_time"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn state<'a>(
        &'a mut self,
        val: DeviceMetric_CalibrationState,
    ) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["state"] = json!(val.to_string());
        return self;
    }

    pub fn time<'a>(&'a mut self, val: &str) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["time"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: DeviceMetric_CalibrationType,
    ) -> &'a mut DeviceMetric_CalibrationBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum DeviceMetric_CalibrationState {
    NotCalibrated,
    CalibrationRequired,
    Calibrated,
    Unspecified,
}

impl DeviceMetric_CalibrationState {
    pub fn from_string(string: &str) -> Option<DeviceMetric_CalibrationState> {
        match string {
            "not-calibrated" => Some(DeviceMetric_CalibrationState::NotCalibrated),
            "calibration-required" => Some(DeviceMetric_CalibrationState::CalibrationRequired),
            "calibrated" => Some(DeviceMetric_CalibrationState::Calibrated),
            "unspecified" => Some(DeviceMetric_CalibrationState::Unspecified),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceMetric_CalibrationState::NotCalibrated => "not-calibrated".to_string(),
            DeviceMetric_CalibrationState::CalibrationRequired => {
                "calibration-required".to_string()
            }
            DeviceMetric_CalibrationState::Calibrated => "calibrated".to_string(),
            DeviceMetric_CalibrationState::Unspecified => "unspecified".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DeviceMetric_CalibrationType {
    Unspecified,
    Offset,
    Gain,
    TwoPoint,
}

impl DeviceMetric_CalibrationType {
    pub fn from_string(string: &str) -> Option<DeviceMetric_CalibrationType> {
        match string {
            "unspecified" => Some(DeviceMetric_CalibrationType::Unspecified),
            "offset" => Some(DeviceMetric_CalibrationType::Offset),
            "gain" => Some(DeviceMetric_CalibrationType::Gain),
            "two-point" => Some(DeviceMetric_CalibrationType::TwoPoint),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceMetric_CalibrationType::Unspecified => "unspecified".to_string(),
            DeviceMetric_CalibrationType::Offset => "offset".to_string(),
            DeviceMetric_CalibrationType::Gain => "gain".to_string(),
            DeviceMetric_CalibrationType::TwoPoint => "two-point".to_string(),
        }
    }
}

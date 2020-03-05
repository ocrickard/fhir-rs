#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.

#[derive(Debug)]
pub struct Location_Position<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Location_Position<'_> {
    pub fn new(value: &Value) -> Location_Position {
        Location_Position {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for altitude
    pub fn _altitude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_altitude") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for latitude
    pub fn _latitude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_latitude") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for longitude
    pub fn _longitude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_longitude") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Altitude. The value domain and the interpretation are the same as for the text
    /// of the altitude element in KML (see notes below).
    pub fn altitude(&self) -> Option<f64> {
        if let Some(val) = self.value.get("altitude") {
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

    /// Latitude. The value domain and the interpretation are the same as for the text
    /// of the latitude element in KML (see notes below).
    pub fn latitude(&self) -> Option<f64> {
        if let Some(val) = self.value.get("latitude") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Longitude. The value domain and the interpretation are the same as for the text
    /// of the longitude element in KML (see notes below).
    pub fn longitude(&self) -> Option<f64> {
        if let Some(val) = self.value.get("longitude") {
            return Some(val.as_f64().unwrap());
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
        if let Some(_val) = self._altitude() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._latitude() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._longitude() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.altitude() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.latitude() {}
        if let Some(_val) = self.longitude() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Location_PositionBuilder {
    pub(crate) value: Value,
}

impl Location_PositionBuilder {
    pub fn build(&self) -> Location_Position {
        Location_Position {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Location_Position) -> Location_PositionBuilder {
        Location_PositionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Location_PositionBuilder {
        let mut __value: Value = json!({});
        return Location_PositionBuilder { value: __value };
    }

    pub fn _altitude<'a>(&'a mut self, val: Element) -> &'a mut Location_PositionBuilder {
        self.value["_altitude"] = json!(val.value);
        return self;
    }

    pub fn _latitude<'a>(&'a mut self, val: Element) -> &'a mut Location_PositionBuilder {
        self.value["_latitude"] = json!(val.value);
        return self;
    }

    pub fn _longitude<'a>(&'a mut self, val: Element) -> &'a mut Location_PositionBuilder {
        self.value["_longitude"] = json!(val.value);
        return self;
    }

    pub fn altitude<'a>(&'a mut self, val: f64) -> &'a mut Location_PositionBuilder {
        self.value["altitude"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Location_PositionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Location_PositionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn latitude<'a>(&'a mut self, val: f64) -> &'a mut Location_PositionBuilder {
        self.value["latitude"] = json!(val);
        return self;
    }

    pub fn longitude<'a>(&'a mut self, val: f64) -> &'a mut Location_PositionBuilder {
        self.value["longitude"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Location_PositionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

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
pub struct Location_HoursOfOperation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Location_HoursOfOperation<'_> {
    pub fn new(value: &Value) -> Location_HoursOfOperation {
        Location_HoursOfOperation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for allDay
    pub fn _all_day(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allDay") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for closingTime
    pub fn _closing_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_closingTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for daysOfWeek
    pub fn _days_of_week(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_daysOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for openingTime
    pub fn _opening_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_openingTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The Location is open all day.
    pub fn all_day(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allDay") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Time that the Location closes.
    pub fn closing_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("closingTime") {
            return Some(string);
        }
        return None;
    }

    /// Indicates which days of the week are available between the start and end Times.
    pub fn days_of_week(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("daysOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Time that the Location opens.
    pub fn opening_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("openingTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._all_day() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._closing_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._days_of_week() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._opening_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.all_day() {}
        if let Some(_val) = self.closing_time() {}
        if let Some(_val) = self.days_of_week() {
            _val.into_iter().for_each(|_e| {});
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
        if let Some(_val) = self.opening_time() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Location_HoursOfOperationBuilder {
    pub(crate) value: Value,
}

impl Location_HoursOfOperationBuilder {
    pub fn build(&self) -> Location_HoursOfOperation {
        Location_HoursOfOperation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Location_HoursOfOperation) -> Location_HoursOfOperationBuilder {
        Location_HoursOfOperationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Location_HoursOfOperationBuilder {
        let mut __value: Value = json!({});
        return Location_HoursOfOperationBuilder { value: __value };
    }

    pub fn _all_day<'a>(&'a mut self, val: Element) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["_allDay"] = json!(val.value);
        return self;
    }

    pub fn _closing_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["_closingTime"] = json!(val.value);
        return self;
    }

    pub fn _days_of_week<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["_daysOfWeek"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _opening_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["_openingTime"] = json!(val.value);
        return self;
    }

    pub fn all_day<'a>(&'a mut self, val: bool) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["allDay"] = json!(val);
        return self;
    }

    pub fn closing_time<'a>(&'a mut self, val: &str) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["closingTime"] = json!(val);
        return self;
    }

    pub fn days_of_week<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["daysOfWeek"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn opening_time<'a>(&'a mut self, val: &str) -> &'a mut Location_HoursOfOperationBuilder {
        self.value["openingTime"] = json!(val);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement_DateFilter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DataRequirement_DateFilter<'_> {
    pub fn new(value: &Value) -> DataRequirement_DateFilter {
        DataRequirement_DateFilter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for searchParam
    pub fn _search_param(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_searchParam") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
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

    /// The date-valued attribute of the filter. The specified path SHALL be a FHIRPath
    /// resolveable on the specified type of the DataRequirement, and SHALL consist only
    /// of identifiers, constant indexers, and .resolve(). The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath
    /// Profile](fhirpath.html#simple) for full details). Note that the index must be an
    /// integer constant. The path must resolve to an element of type date, dateTime,
    /// Period, Schedule, or Timing.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// A date parameter that refers to a search parameter defined on the specified type
    /// of the DataRequirement, and which searches on elements of type date, dateTime,
    /// Period, Schedule, or Timing.
    pub fn search_param(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("searchParam") {
            return Some(string);
        }
        return None;
    }

    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return only
    /// those data items that are equal to the specified dateTime. If a Duration is
    /// specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return only
    /// those data items that are equal to the specified dateTime. If a Duration is
    /// specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub fn value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("valueDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the filter. If period is specified, the filter will return only
    /// those data items that fall within the bounds determined by the Period, inclusive
    /// of the period boundaries. If dateTime is specified, the filter will return only
    /// those data items that are equal to the specified dateTime. If a Duration is
    /// specified, the filter will return only those data items that fall within
    /// Duration before now.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._search_param() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date_time() {
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
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.search_param() {}
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DataRequirement_DateFilterBuilder {
    pub(crate) value: Value,
}

impl DataRequirement_DateFilterBuilder {
    pub fn build(&self) -> DataRequirement_DateFilter {
        DataRequirement_DateFilter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DataRequirement_DateFilter) -> DataRequirement_DateFilterBuilder {
        DataRequirement_DateFilterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DataRequirement_DateFilterBuilder {
        let mut __value: Value = json!({});
        return DataRequirement_DateFilterBuilder { value: __value };
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn _search_param<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["_searchParam"] = json!(val.value);
        return self;
    }

    pub fn _value_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["_valueDateTime"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn search_param<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["searchParam"] = json!(val);
        return self;
    }

    pub fn value_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["valueDateTime"] = json!(val);
        return self;
    }

    pub fn value_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["valueDuration"] = json!(val.value);
        return self;
    }

    pub fn value_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut DataRequirement_DateFilterBuilder {
        self.value["valuePeriod"] = json!(val.value);
        return self;
    }
}

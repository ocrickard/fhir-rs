#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement_DateFilter<'a> {
    pub value: &'a Value,
}

impl DataRequirement_DateFilter<'_> {
    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
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

    /// Extensions for searchParam
    pub fn _search_param(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_searchParam") {
            return Some(Element { value: val });
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

    /// A date parameter that refers to a search parameter defined on the specified type
    /// of the DataRequirement, and which searches on elements of type date, dateTime,
    /// Period, Schedule, or Timing.
    pub fn search_param(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("searchParam") {
            return Some(string);
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

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
            return Some(Element { value: val });
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
            return Some(Duration { value: val });
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
            return Some(Period { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._path() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._search_param() {
            _val.validate();
        }
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.search_param() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.value_duration() {
            _val.validate();
        }
        if let Some(_val) = self.value_period() {
            _val.validate();
        }
        return true;
    }
}

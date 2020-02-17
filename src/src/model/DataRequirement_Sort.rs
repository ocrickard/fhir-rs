#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement_Sort<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DataRequirement_Sort<'_> {
    pub fn new(value: &Value) -> DataRequirement_Sort {
        DataRequirement_Sort {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for direction
    pub fn _direction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_direction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// The direction of the sort, ascending or descending.
    pub fn direction(&self) -> Option<DataRequirement_SortDirection> {
        if let Some(Value::String(val)) = self.value.get("direction") {
            return Some(DataRequirement_SortDirection::from_string(&val).unwrap());
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

    /// The attribute of the sort. The specified path must be resolvable from the type
    /// of the required data. The path is allowed to contain qualifiers (.) to traverse
    /// sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-
    /// elements. Note that the index must be an integer constant.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._direction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.direction() {}
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
        return true;
    }
}

#[derive(Debug)]
pub struct DataRequirement_SortBuilder {
    pub(crate) value: Value,
}

impl DataRequirement_SortBuilder {
    pub fn build(&self) -> DataRequirement_Sort {
        DataRequirement_Sort {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DataRequirement_Sort) -> DataRequirement_SortBuilder {
        DataRequirement_SortBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DataRequirement_SortBuilder {
        let mut __value: Value = json!({});
        return DataRequirement_SortBuilder { value: __value };
    }

    pub fn _direction<'a>(&'a mut self, val: Element) -> &'a mut DataRequirement_SortBuilder {
        self.value["_direction"] = json!(val.value);
        return self;
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut DataRequirement_SortBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn direction<'a>(
        &'a mut self,
        val: DataRequirement_SortDirection,
    ) -> &'a mut DataRequirement_SortBuilder {
        self.value["direction"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DataRequirement_SortBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_SortBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DataRequirement_SortBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_SortBuilder {
        self.value["path"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum DataRequirement_SortDirection {
    Ascending,
    Descending,
}

impl DataRequirement_SortDirection {
    pub fn from_string(string: &str) -> Option<DataRequirement_SortDirection> {
        match string {
            "ascending" => Some(DataRequirement_SortDirection::Ascending),
            "descending" => Some(DataRequirement_SortDirection::Descending),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataRequirement_SortDirection::Ascending => "ascending".to_string(),
            DataRequirement_SortDirection::Descending => "descending".to_string(),
        }
    }
}

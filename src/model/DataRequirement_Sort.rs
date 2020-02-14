#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement_Sort<'a> {
    pub value: &'a Value,
}

impl DataRequirement_Sort<'_> {
    /// Extensions for direction
    pub fn _direction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_direction") {
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

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._path() {
            _val.validate();
        }
        if let Some(_val) = self.direction() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.path() {}
        return true;
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
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement_CodeFilter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DataRequirement_CodeFilter<'_> {
    pub fn new(value: &Value) -> DataRequirement_CodeFilter {
        DataRequirement_CodeFilter {
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

    /// The codes for the code filter. If values are given, the filter will return only
    /// those data items for which the code-valued attribute specified by the path has a
    /// value that is one of the specified codes. If codes are specified in addition to
    /// a value set, the filter returns items matching a code in the value set or one of
    /// the specified codes.
    pub fn code(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
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

    /// The code-valued attribute of the filter. The specified path SHALL be a FHIRPath
    /// resolveable on the specified type of the DataRequirement, and SHALL consist only
    /// of identifiers, constant indexers, and .resolve(). The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath
    /// Profile](fhirpath.html#simple) for full details). Note that the index must be an
    /// integer constant. The path must resolve to an element of type code, Coding, or
    /// CodeableConcept.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// A token parameter that refers to a search parameter defined on the specified
    /// type of the DataRequirement, and which searches on elements of type code,
    /// Coding, or CodeableConcept.
    pub fn search_param(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("searchParam") {
            return Some(string);
        }
        return None;
    }

    /// The valueset for the code filter. The valueSet and code elements are additive.
    /// If valueSet is specified, the filter will return only those data items for which
    /// the value of the code-valued element specified in the path is a member of the
    /// specified valueset.
    pub fn value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueSet") {
            return Some(string);
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
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.value_set() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DataRequirement_CodeFilterBuilder {
    pub(crate) value: Value,
}

impl DataRequirement_CodeFilterBuilder {
    pub fn build(&self) -> DataRequirement_CodeFilter {
        DataRequirement_CodeFilter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DataRequirement_CodeFilter) -> DataRequirement_CodeFilterBuilder {
        DataRequirement_CodeFilterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DataRequirement_CodeFilterBuilder {
        let mut __value: Value = json!({});
        return DataRequirement_CodeFilterBuilder { value: __value };
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn _search_param<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["_searchParam"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn search_param<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["searchParam"] = json!(val);
        return self;
    }

    pub fn value_set<'a>(&'a mut self, val: &str) -> &'a mut DataRequirement_CodeFilterBuilder {
        self.value["valueSet"] = json!(val);
        return self;
    }
}

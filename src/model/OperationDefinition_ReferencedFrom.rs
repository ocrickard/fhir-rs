#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).

#[derive(Debug)]
pub struct OperationDefinition_ReferencedFrom<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl OperationDefinition_ReferencedFrom<'_> {
    pub fn new(value: &Value) -> OperationDefinition_ReferencedFrom {
        OperationDefinition_ReferencedFrom {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for source
    pub fn _source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_source") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sourceId
    pub fn _source_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceId") {
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

    /// The name of the parameter or dot-separated path of parameter names pointing to
    /// the resource parameter that is expected to contain a reference to this resource.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// The id of the element in the referencing resource that is expected to resolve to
    /// this resource.
    pub fn source_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceId") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source_id() {
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
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.source_id() {}
        return true;
    }
}

#[derive(Debug)]
pub struct OperationDefinition_ReferencedFromBuilder {
    pub(crate) value: Value,
}

impl OperationDefinition_ReferencedFromBuilder {
    pub fn build(&self) -> OperationDefinition_ReferencedFrom {
        OperationDefinition_ReferencedFrom {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: OperationDefinition_ReferencedFrom,
    ) -> OperationDefinition_ReferencedFromBuilder {
        OperationDefinition_ReferencedFromBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> OperationDefinition_ReferencedFromBuilder {
        let mut __value: Value = json!({});
        return OperationDefinition_ReferencedFromBuilder { value: __value };
    }

    pub fn _source<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["_source"] = json!(val.value);
        return self;
    }

    pub fn _source_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["_sourceId"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["source"] = json!(val);
        return self;
    }

    pub fn source_id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut OperationDefinition_ReferencedFromBuilder {
        self.value["sourceId"] = json!(val);
        return self;
    }
}

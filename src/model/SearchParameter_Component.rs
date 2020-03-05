#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A search parameter that defines a named search item that can be used to
/// search/filter on a resource.

#[derive(Debug)]
pub struct SearchParameter_Component<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SearchParameter_Component<'_> {
    pub fn new(value: &Value) -> SearchParameter_Component {
        SearchParameter_Component {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The definition of the search parameter that describes this part.
    pub fn definition(&self) -> &str {
        self.value.get("definition").unwrap().as_str().unwrap()
    }

    /// A sub-expression that defines how to extract values for this component from the
    /// output of the main SearchParameter.expression.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expression() {}
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
        return true;
    }
}

#[derive(Debug)]
pub struct SearchParameter_ComponentBuilder {
    pub(crate) value: Value,
}

impl SearchParameter_ComponentBuilder {
    pub fn build(&self) -> SearchParameter_Component {
        SearchParameter_Component {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SearchParameter_Component) -> SearchParameter_ComponentBuilder {
        SearchParameter_ComponentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(definition: &str) -> SearchParameter_ComponentBuilder {
        let mut __value: Value = json!({});
        __value["definition"] = json!(definition);
        return SearchParameter_ComponentBuilder { value: __value };
    }

    pub fn _expression<'a>(&'a mut self, val: Element) -> &'a mut SearchParameter_ComponentBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: &str) -> &'a mut SearchParameter_ComponentBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SearchParameter_ComponentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SearchParameter_ComponentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SearchParameter_ComponentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

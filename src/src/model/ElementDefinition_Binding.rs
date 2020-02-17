#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Binding<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Binding<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Binding {
        ElementDefinition_Binding {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for strength
    pub fn _strength(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_strength") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the intended use of this particular set of codes.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Indicates the degree of conformance expectations associated with this binding -
    /// that is, the degree to which the provided value set must be adhered to in the
    /// instances.
    pub fn strength(&self) -> Option<ElementDefinition_BindingStrength> {
        if let Some(Value::String(val)) = self.value.get("strength") {
            return Some(ElementDefinition_BindingStrength::from_string(&val).unwrap());
        }
        return None;
    }

    /// Refers to the value set that identifies the set of codes the binding refers to.
    pub fn value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueSet") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._strength() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.strength() {}
        if let Some(_val) = self.value_set() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_BindingBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_BindingBuilder {
    pub fn build(&self) -> ElementDefinition_Binding {
        ElementDefinition_Binding {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ElementDefinition_Binding) -> ElementDefinition_BindingBuilder {
        ElementDefinition_BindingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_BindingBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_BindingBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _strength<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["_strength"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength<'a>(
        &'a mut self,
        val: ElementDefinition_BindingStrength,
    ) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["strength"] = json!(val.to_string());
        return self;
    }

    pub fn value_set<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_BindingBuilder {
        self.value["valueSet"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_BindingStrength {
    Required,
    Extensible,
    Preferred,
    Example,
}

impl ElementDefinition_BindingStrength {
    pub fn from_string(string: &str) -> Option<ElementDefinition_BindingStrength> {
        match string {
            "required" => Some(ElementDefinition_BindingStrength::Required),
            "extensible" => Some(ElementDefinition_BindingStrength::Extensible),
            "preferred" => Some(ElementDefinition_BindingStrength::Preferred),
            "example" => Some(ElementDefinition_BindingStrength::Example),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ElementDefinition_BindingStrength::Required => "required".to_string(),
            ElementDefinition_BindingStrength::Extensible => "extensible".to_string(),
            ElementDefinition_BindingStrength::Preferred => "preferred".to_string(),
            ElementDefinition_BindingStrength::Example => "example".to_string(),
        }
    }
}

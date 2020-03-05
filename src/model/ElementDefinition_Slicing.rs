#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ElementDefinition_Discriminator::ElementDefinition_Discriminator;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Slicing<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Slicing<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Slicing {
        ElementDefinition_Slicing {
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

    /// Extensions for ordered
    pub fn _ordered(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ordered") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for rules
    pub fn _rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable text description of how the slicing works. If there is no
    /// discriminator, this is required to be present to provide whatever information is
    /// possible about how the slices can be differentiated.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Designates which child elements are used to discriminate between the slices when
    /// processing an instance. If one or more discriminators are provided, the value of
    /// the child elements in the instance data SHALL completely distinguish which slice
    /// the element in the resource matches based on the allowed values for those
    /// elements in each of the slices.
    pub fn discriminator(&self) -> Option<Vec<ElementDefinition_Discriminator>> {
        if let Some(Value::Array(val)) = self.value.get("discriminator") {
            return Some(
                val.into_iter()
                    .map(|e| ElementDefinition_Discriminator {
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

    /// If the matching elements have to occur in the same order as defined in the
    /// profile.
    pub fn ordered(&self) -> Option<bool> {
        if let Some(val) = self.value.get("ordered") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether additional slices are allowed or not. When the slices are ordered,
    /// profile authors can also say that additional slices are only allowed at the end.
    pub fn rules(&self) -> Option<ElementDefinition_SlicingRules> {
        if let Some(Value::String(val)) = self.value.get("rules") {
            return Some(ElementDefinition_SlicingRules::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._ordered() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.discriminator() {
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
        if let Some(_val) = self.ordered() {}
        if let Some(_val) = self.rules() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_SlicingBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_SlicingBuilder {
    pub fn build(&self) -> ElementDefinition_Slicing {
        ElementDefinition_Slicing {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ElementDefinition_Slicing) -> ElementDefinition_SlicingBuilder {
        ElementDefinition_SlicingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_SlicingBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_SlicingBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _ordered<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["_ordered"] = json!(val.value);
        return self;
    }

    pub fn _rules<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["_rules"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn discriminator<'a>(
        &'a mut self,
        val: Vec<ElementDefinition_Discriminator>,
    ) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["discriminator"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn ordered<'a>(&'a mut self, val: bool) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["ordered"] = json!(val);
        return self;
    }

    pub fn rules<'a>(
        &'a mut self,
        val: ElementDefinition_SlicingRules,
    ) -> &'a mut ElementDefinition_SlicingBuilder {
        self.value["rules"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_SlicingRules {
    Closed,
    Open,
    OpenAtEnd,
}

impl ElementDefinition_SlicingRules {
    pub fn from_string(string: &str) -> Option<ElementDefinition_SlicingRules> {
        match string {
            "closed" => Some(ElementDefinition_SlicingRules::Closed),
            "open" => Some(ElementDefinition_SlicingRules::Open),
            "openAtEnd" => Some(ElementDefinition_SlicingRules::OpenAtEnd),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ElementDefinition_SlicingRules::Closed => "closed".to_string(),
            ElementDefinition_SlicingRules::Open => "open".to_string(),
            ElementDefinition_SlicingRules::OpenAtEnd => "openAtEnd".to_string(),
        }
    }
}

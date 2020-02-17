#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap_DependsOn<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConceptMap_DependsOn<'_> {
    pub fn new(value: &Value) -> ConceptMap_DependsOn {
        ConceptMap_DependsOn {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for property
    pub fn _property(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_property") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// A reference to an element that holds a coded value that corresponds to a code
    /// system property. The idea is that the information model carries an element
    /// somewhere that is labeled to correspond with a code system property.
    pub fn property(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("property") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that identifies the code system of the dependency code (if the
    /// source/dependency is a value set that crosses code systems).
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// Identity (code or path) or the element/item/ValueSet/text that the map depends
    /// on / refers to.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._property() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.display() {}
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
        if let Some(_val) = self.property() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ConceptMap_DependsOnBuilder {
    pub(crate) value: Value,
}

impl ConceptMap_DependsOnBuilder {
    pub fn build(&self) -> ConceptMap_DependsOn {
        ConceptMap_DependsOn {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConceptMap_DependsOn) -> ConceptMap_DependsOnBuilder {
        ConceptMap_DependsOnBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ConceptMap_DependsOnBuilder {
        let mut __value: Value = json!({});
        return ConceptMap_DependsOnBuilder { value: __value };
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _property<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["_property"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["property"] = json!(val);
        return self;
    }

    pub fn system<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["system"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_DependsOnBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}

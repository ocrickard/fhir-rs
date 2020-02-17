#![allow(unused_imports, non_camel_case_types)]

use crate::model::ConceptMap_Element::ConceptMap_Element;
use crate::model::ConceptMap_Unmapped::ConceptMap_Unmapped;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap_Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConceptMap_Group<'_> {
    pub fn new(value: &Value) -> ConceptMap_Group {
        ConceptMap_Group {
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

    /// Extensions for sourceVersion
    pub fn _source_version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceVersion") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for target
    pub fn _target(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_target") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for targetVersion
    pub fn _target_version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetVersion") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Mappings for an individual concept in the source to one or more concepts in the
    /// target.
    pub fn element(&self) -> Vec<ConceptMap_Element> {
        self.value
            .get("element")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ConceptMap_Element {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// An absolute URI that identifies the source system where the concepts to be
    /// mapped are defined.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// The specific version of the code system, as determined by the code system
    /// authority.
    pub fn source_version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceVersion") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that identifies the target system that the concepts will be
    /// mapped to.
    pub fn target(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("target") {
            return Some(string);
        }
        return None;
    }

    /// The specific version of the code system, as determined by the code system
    /// authority.
    pub fn target_version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetVersion") {
            return Some(string);
        }
        return None;
    }

    /// What to do when there is no mapping for the source concept. "Unmapped" does not
    /// include codes that are unmatched, and the unmapped element is ignored in a code
    /// is specified to have equivalence = unmatched.
    pub fn unmapped(&self) -> Option<ConceptMap_Unmapped> {
        if let Some(val) = self.value.get("unmapped") {
            return Some(ConceptMap_Unmapped {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source_version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target_version() {
            if !_val.validate() {
                return false;
            }
        }
        if !self
            .element()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
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
        if let Some(_val) = self.source_version() {}
        if let Some(_val) = self.target() {}
        if let Some(_val) = self.target_version() {}
        if let Some(_val) = self.unmapped() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConceptMap_GroupBuilder {
    pub(crate) value: Value,
}

impl ConceptMap_GroupBuilder {
    pub fn build(&self) -> ConceptMap_Group {
        ConceptMap_Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConceptMap_Group) -> ConceptMap_GroupBuilder {
        ConceptMap_GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(element: Vec<ConceptMap_Element>) -> ConceptMap_GroupBuilder {
        let mut __value: Value = json!({});
        __value["element"] = json!(element.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return ConceptMap_GroupBuilder { value: __value };
    }

    pub fn _source<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_GroupBuilder {
        self.value["_source"] = json!(val.value);
        return self;
    }

    pub fn _source_version<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_GroupBuilder {
        self.value["_sourceVersion"] = json!(val.value);
        return self;
    }

    pub fn _target<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_GroupBuilder {
        self.value["_target"] = json!(val.value);
        return self;
    }

    pub fn _target_version<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_GroupBuilder {
        self.value["_targetVersion"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConceptMap_GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConceptMap_GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_GroupBuilder {
        self.value["source"] = json!(val);
        return self;
    }

    pub fn source_version<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_GroupBuilder {
        self.value["sourceVersion"] = json!(val);
        return self;
    }

    pub fn target<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_GroupBuilder {
        self.value["target"] = json!(val);
        return self;
    }

    pub fn target_version<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_GroupBuilder {
        self.value["targetVersion"] = json!(val);
        return self;
    }

    pub fn unmapped<'a>(&'a mut self, val: ConceptMap_Unmapped) -> &'a mut ConceptMap_GroupBuilder {
        self.value["unmapped"] = json!(val.value);
        return self;
    }
}

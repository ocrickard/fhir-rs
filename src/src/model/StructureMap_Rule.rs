#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Dependent::StructureMap_Dependent;
use crate::model::StructureMap_Source::StructureMap_Source;
use crate::model::StructureMap_Target::StructureMap_Target;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Rule<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Rule<'_> {
    pub fn new(value: &Value) -> StructureMap_Rule {
        StructureMap_Rule {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Which other rules to apply in the context of this rule.
    pub fn dependent(&self) -> Option<Vec<StructureMap_Dependent>> {
        if let Some(Value::Array(val)) = self.value.get("dependent") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Dependent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Documentation for this instance of data.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
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

    /// Name of the rule for internal references.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Rules contained in this rule.
    pub fn rule(&self) -> Option<Vec<StructureMap_Rule>> {
        if let Some(Value::Array(val)) = self.value.get("rule") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Rule {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Source inputs to the mapping.
    pub fn source(&self) -> Vec<StructureMap_Source> {
        self.value
            .get("source")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Source {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// Content to create because of this mapping rule.
    pub fn target(&self) -> Option<Vec<StructureMap_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Target {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.dependent() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.rule() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .source()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.target() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_RuleBuilder {
    pub(crate) value: Value,
}

impl StructureMap_RuleBuilder {
    pub fn build(&self) -> StructureMap_Rule {
        StructureMap_Rule {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Rule) -> StructureMap_RuleBuilder {
        StructureMap_RuleBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(source: Vec<StructureMap_Source>) -> StructureMap_RuleBuilder {
        let mut __value: Value = json!({});
        __value["source"] = json!(source.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return StructureMap_RuleBuilder { value: __value };
    }

    pub fn _documentation<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_RuleBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_RuleBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn dependent<'a>(
        &'a mut self,
        val: Vec<StructureMap_Dependent>,
    ) -> &'a mut StructureMap_RuleBuilder {
        self.value["dependent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn documentation<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_RuleBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureMap_RuleBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_RuleBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_RuleBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_RuleBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn rule<'a>(&'a mut self, val: Vec<StructureMap_Rule>) -> &'a mut StructureMap_RuleBuilder {
        self.value["rule"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target<'a>(
        &'a mut self,
        val: Vec<StructureMap_Target>,
    ) -> &'a mut StructureMap_RuleBuilder {
        self.value["target"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

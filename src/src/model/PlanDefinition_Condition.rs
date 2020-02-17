#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_Condition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PlanDefinition_Condition<'_> {
    pub fn new(value: &Value) -> PlanDefinition_Condition {
        PlanDefinition_Condition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An expression that returns true or false, indicating whether the condition is
    /// satisfied.
    pub fn expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("expression") {
            return Some(Expression {
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

    /// The kind of condition.
    pub fn kind(&self) -> Option<PlanDefinition_ConditionKind> {
        if let Some(Value::String(val)) = self.value.get("kind") {
            return Some(PlanDefinition_ConditionKind::from_string(&val).unwrap());
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
        if let Some(_val) = self._kind() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expression() {
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
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PlanDefinition_ConditionBuilder {
    pub(crate) value: Value,
}

impl PlanDefinition_ConditionBuilder {
    pub fn build(&self) -> PlanDefinition_Condition {
        PlanDefinition_Condition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PlanDefinition_Condition) -> PlanDefinition_ConditionBuilder {
        PlanDefinition_ConditionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PlanDefinition_ConditionBuilder {
        let mut __value: Value = json!({});
        return PlanDefinition_ConditionBuilder { value: __value };
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn expression<'a>(
        &'a mut self,
        val: Expression,
    ) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["expression"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn kind<'a>(
        &'a mut self,
        val: PlanDefinition_ConditionKind,
    ) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["kind"] = json!(val.to_string());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_ConditionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ConditionKind {
    Applicability,
    Start,
    Stop,
}

impl PlanDefinition_ConditionKind {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ConditionKind> {
        match string {
            "applicability" => Some(PlanDefinition_ConditionKind::Applicability),
            "start" => Some(PlanDefinition_ConditionKind::Start),
            "stop" => Some(PlanDefinition_ConditionKind::Stop),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PlanDefinition_ConditionKind::Applicability => "applicability".to_string(),
            PlanDefinition_ConditionKind::Start => "start".to_string(),
            PlanDefinition_ConditionKind::Stop => "stop".to_string(),
        }
    }
}

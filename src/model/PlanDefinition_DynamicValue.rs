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
pub struct PlanDefinition_DynamicValue<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PlanDefinition_DynamicValue<'_> {
    pub fn new(value: &Value) -> PlanDefinition_DynamicValue {
        PlanDefinition_DynamicValue {
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

    /// An expression specifying the value of the customized element.
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

    /// The path to the element to be customized. This is the path on the resource that
    /// will hold the result of the calculation defined by the expression. The specified
    /// path SHALL be a FHIRPath resolveable on the specified target type of the
    /// ActivityDefinition, and SHALL consist only of identifiers, constant indexers,
    /// and a restricted subset of functions. The path is allowed to contain qualifiers
    /// (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-
    /// cardinality sub-elements (see the [Simple FHIRPath
    /// Profile](fhirpath.html#simple) for full details).
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.path() {}
        return true;
    }
}

#[derive(Debug)]
pub struct PlanDefinition_DynamicValueBuilder {
    pub(crate) value: Value,
}

impl PlanDefinition_DynamicValueBuilder {
    pub fn build(&self) -> PlanDefinition_DynamicValue {
        PlanDefinition_DynamicValue {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PlanDefinition_DynamicValue) -> PlanDefinition_DynamicValueBuilder {
        PlanDefinition_DynamicValueBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PlanDefinition_DynamicValueBuilder {
        let mut __value: Value = json!({});
        return PlanDefinition_DynamicValueBuilder { value: __value };
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn expression<'a>(
        &'a mut self,
        val: Expression,
    ) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["expression"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut PlanDefinition_DynamicValueBuilder {
        self.value["path"] = json!(val);
        return self;
    }
}

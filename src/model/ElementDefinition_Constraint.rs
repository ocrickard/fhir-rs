#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Constraint<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Constraint<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Constraint {
        ElementDefinition_Constraint {
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

    /// Extensions for human
    pub fn _human(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_human") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for key
    pub fn _key(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_key") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requirements
    pub fn _requirements(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirements") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for severity
    pub fn _severity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_severity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for xpath
    pub fn _xpath(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_xpath") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see
    /// if this constraint is met.
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

    /// Text that can be used to describe the constraint in messages identifying that
    /// the constraint has been violated.
    pub fn human(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("human") {
            return Some(string);
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

    /// Allows identification of which elements have their cardinalities impacted by the
    /// constraint.  Will not be referenced for constraints that do not affect
    /// cardinality.
    pub fn key(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("key") {
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

    /// Description of why this constraint is necessary or appropriate.
    pub fn requirements(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirements") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the impact constraint violation has on the conformance of the
    /// instance.
    pub fn severity(&self) -> Option<ElementDefinition_ConstraintSeverity> {
        if let Some(Value::String(val)) = self.value.get("severity") {
            return Some(ElementDefinition_ConstraintSeverity::from_string(&val).unwrap());
        }
        return None;
    }

    /// A reference to the original source of the constraint, for traceability purposes.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// An XPath expression of constraint that can be executed to see if this constraint
    /// is met.
    pub fn xpath(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("xpath") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._human() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._key() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._requirements() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._xpath() {
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
        if let Some(_val) = self.human() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.key() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requirements() {}
        if let Some(_val) = self.severity() {}
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.xpath() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_ConstraintBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_ConstraintBuilder {
    pub fn build(&self) -> ElementDefinition_Constraint {
        ElementDefinition_Constraint {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ElementDefinition_Constraint) -> ElementDefinition_ConstraintBuilder {
        ElementDefinition_ConstraintBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_ConstraintBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_ConstraintBuilder { value: __value };
    }

    pub fn _expression<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn _human<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_human"] = json!(val.value);
        return self;
    }

    pub fn _key<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_key"] = json!(val.value);
        return self;
    }

    pub fn _requirements<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_requirements"] = json!(val.value);
        return self;
    }

    pub fn _severity<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_severity"] = json!(val.value);
        return self;
    }

    pub fn _xpath<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["_xpath"] = json!(val.value);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn human<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["human"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn key<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["key"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requirements<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["requirements"] = json!(val);
        return self;
    }

    pub fn severity<'a>(
        &'a mut self,
        val: ElementDefinition_ConstraintSeverity,
    ) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["severity"] = json!(val.to_string());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["source"] = json!(val);
        return self;
    }

    pub fn xpath<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_ConstraintBuilder {
        self.value["xpath"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_ConstraintSeverity {
    Error,
    Warning,
}

impl ElementDefinition_ConstraintSeverity {
    pub fn from_string(string: &str) -> Option<ElementDefinition_ConstraintSeverity> {
        match string {
            "error" => Some(ElementDefinition_ConstraintSeverity::Error),
            "warning" => Some(ElementDefinition_ConstraintSeverity::Warning),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ElementDefinition_ConstraintSeverity::Error => "error".to_string(),
            ElementDefinition_ConstraintSeverity::Warning => "warning".to_string(),
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.

#[derive(Debug)]
pub struct StructureDefinition_Context<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureDefinition_Context<'_> {
    pub fn new(value: &Value) -> StructureDefinition_Context {
        StructureDefinition_Context {
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An expression that defines where an extension can be used in resources.
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

    /// Defines how to interpret the expression that defines what the context of the
    /// extension is.
    pub fn fhir_type(&self) -> Option<StructureDefinition_ContextType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(StructureDefinition_ContextType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
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
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureDefinition_ContextBuilder {
    pub(crate) value: Value,
}

impl StructureDefinition_ContextBuilder {
    pub fn build(&self) -> StructureDefinition_Context {
        StructureDefinition_Context {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureDefinition_Context) -> StructureDefinition_ContextBuilder {
        StructureDefinition_ContextBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureDefinition_ContextBuilder {
        let mut __value: Value = json!({});
        return StructureDefinition_ContextBuilder { value: __value };
    }

    pub fn _expression<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: StructureDefinition_ContextType,
    ) -> &'a mut StructureDefinition_ContextBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum StructureDefinition_ContextType {
    Fhirpath,
    Element,
    Extension,
}

impl StructureDefinition_ContextType {
    pub fn from_string(string: &str) -> Option<StructureDefinition_ContextType> {
        match string {
            "fhirpath" => Some(StructureDefinition_ContextType::Fhirpath),
            "element" => Some(StructureDefinition_ContextType::Element),
            "extension" => Some(StructureDefinition_ContextType::Extension),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureDefinition_ContextType::Fhirpath => "fhirpath".to_string(),
            StructureDefinition_ContextType::Element => "element".to_string(),
            StructureDefinition_ContextType::Extension => "extension".to_string(),
        }
    }
}

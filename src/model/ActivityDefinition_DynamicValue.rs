#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.

#[derive(Debug)]
pub struct ActivityDefinition_DynamicValue<'a> {
    pub value: &'a Value,
}

impl ActivityDefinition_DynamicValue<'_> {
    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
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

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An expression specifying the value of the customized element.
    pub fn expression(&self) -> Expression {
        Expression {
            value: &self.value["expression"],
        }
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.path() {}
        if let Some(_val) = self._path() {
            _val.validate();
        }
        let _ = self.expression().validate();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

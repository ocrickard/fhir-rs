#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_Operation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement_Operation<'_> {
    pub fn new(value: &Value) -> CapabilityStatement_Operation {
        CapabilityStatement_Operation {
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

    /// Where the formal definition can be found. If a server references the base
    /// definition of an Operation (i.e. from the specification itself such as
    /// ```http://hl7.org/fhir/OperationDefinition/ValueSet-expand```), that means it
    /// supports the full capabilities of the operation - e.g. both GET and POST
    /// invocation.  If it only supports a subset, it must define its own custom
    /// [[[OperationDefinition]]] with a 'base' of the original OperationDefinition.
    /// The custom definition would describe the specific subset of functionality
    /// supported.
    pub fn definition(&self) -> &str {
        self.value.get("definition").unwrap().as_str().unwrap()
    }

    /// Documentation that describes anything special about the operation behavior,
    /// possibly detailing different behavior for system, type and instance-level
    /// invocation of the operation.
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

    /// The name of the operation or query. For an operation, this is the name  prefixed
    /// with $ and used in the URL. For a query, this is the name used in the _query
    /// parameter when the query is called.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
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
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement_OperationBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement_OperationBuilder {
    pub fn build(&self) -> CapabilityStatement_Operation {
        CapabilityStatement_Operation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement_Operation) -> CapabilityStatement_OperationBuilder {
        CapabilityStatement_OperationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(definition: &str) -> CapabilityStatement_OperationBuilder {
        let mut __value: Value = json!({});
        __value["definition"] = json!(definition);
        return CapabilityStatement_OperationBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_OperationBuilder {
        self.value["name"] = json!(val);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::ImplementationGuide_Grouping::ImplementationGuide_Grouping;
use crate::model::ImplementationGuide_Page::ImplementationGuide_Page;
use crate::model::ImplementationGuide_Parameter::ImplementationGuide_Parameter;
use crate::model::ImplementationGuide_Resource::ImplementationGuide_Resource;
use crate::model::ImplementationGuide_Template::ImplementationGuide_Template;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Definition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide_Definition<'_> {
    pub fn new(value: &Value) -> ImplementationGuide_Definition {
        ImplementationGuide_Definition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// A logical group of resources. Logical groups can be used when building pages.
    pub fn grouping(&self) -> Option<Vec<ImplementationGuide_Grouping>> {
        if let Some(Value::Array(val)) = self.value.get("grouping") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Grouping {
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

    /// A page / section in the implementation guide. The root page is the
    /// implementation guide home page.
    pub fn page(&self) -> Option<ImplementationGuide_Page> {
        if let Some(val) = self.value.get("page") {
            return Some(ImplementationGuide_Page {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Defines how IG is built by tools.
    pub fn parameter(&self) -> Option<Vec<ImplementationGuide_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Parameter {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A resource that is part of the implementation guide. Conformance resources
    /// (value set, structure definition, capability statements etc.) are obvious
    /// candidates for inclusion, but any kind of resource can be included as an example
    /// resource.
    pub fn resource(&self) -> Vec<ImplementationGuide_Resource> {
        self.value
            .get("resource")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ImplementationGuide_Resource {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// A template for building resources.
    pub fn template(&self) -> Option<Vec<ImplementationGuide_Template>> {
        if let Some(Value::Array(val)) = self.value.get("template") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Template {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.grouping() {
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
        if let Some(_val) = self.page() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.parameter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .resource()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.template() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuide_DefinitionBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuide_DefinitionBuilder {
    pub fn build(&self) -> ImplementationGuide_Definition {
        ImplementationGuide_Definition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide_Definition) -> ImplementationGuide_DefinitionBuilder {
        ImplementationGuide_DefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        resource: Vec<ImplementationGuide_Resource>,
    ) -> ImplementationGuide_DefinitionBuilder {
        let mut __value: Value = json!({});
        __value["resource"] = json!(resource.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return ImplementationGuide_DefinitionBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn grouping<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Grouping>,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["grouping"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn page<'a>(
        &'a mut self,
        val: ImplementationGuide_Page,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["page"] = json!(val.value);
        return self;
    }

    pub fn parameter<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Parameter>,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["parameter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn template<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Template>,
    ) -> &'a mut ImplementationGuide_DefinitionBuilder {
        self.value["template"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

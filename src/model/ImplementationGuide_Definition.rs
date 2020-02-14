#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::ImplementationGuide_Grouping::ImplementationGuide_Grouping;
use crate::model::ImplementationGuide_Page::ImplementationGuide_Page;
use crate::model::ImplementationGuide_Parameter::ImplementationGuide_Parameter;
use crate::model::ImplementationGuide_Resource::ImplementationGuide_Resource;
use crate::model::ImplementationGuide_Template::ImplementationGuide_Template;
use serde_json::value::Value;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Definition<'a> {
    pub value: &'a Value,
}

impl ImplementationGuide_Definition<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A page / section in the implementation guide. The root page is the
    /// implementation guide home page.
    pub fn page(&self) -> Option<ImplementationGuide_Page> {
        if let Some(val) = self.value.get("page") {
            return Some(ImplementationGuide_Page { value: val });
        }
        return None;
    }

    /// A template for building resources.
    pub fn template(&self) -> Option<Vec<ImplementationGuide_Template>> {
        if let Some(Value::Array(val)) = self.value.get("template") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Template { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
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
            .map(|e| ImplementationGuide_Resource { value: e })
            .collect::<Vec<_>>()
    }

    /// A logical group of resources. Logical groups can be used when building pages.
    pub fn grouping(&self) -> Option<Vec<ImplementationGuide_Grouping>> {
        if let Some(Value::Array(val)) = self.value.get("grouping") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Grouping { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines how IG is built by tools.
    pub fn parameter(&self) -> Option<Vec<ImplementationGuide_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.page() {
            _val.validate();
        }
        if let Some(_val) = self.template() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.resource().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.grouping() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

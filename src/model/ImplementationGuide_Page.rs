#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Page<'a> {
    pub value: &'a Value,
}

impl ImplementationGuide_Page<'_> {
    /// The source address for the page.
    pub fn name_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("nameUrl") {
            return Some(string);
        }
        return None;
    }

    /// The source address for the page.
    pub fn name_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("nameReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for nameUrl
    pub fn _name_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_nameUrl") {
            return Some(Element { value: val });
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

    /// A short title used to represent this page in navigational structures such as
    /// table of contents, bread crumbs, etc.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Nested Pages/Sections under this page.
    pub fn page(&self) -> Option<Vec<ImplementationGuide_Page>> {
        if let Some(Value::Array(val)) = self.value.get("page") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Page { value: e })
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

    /// Extensions for generation
    pub fn _generation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_generation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that indicates how the page is generated.
    pub fn generation(&self) -> Option<ImplementationGuide_PageGeneration> {
        if let Some(Value::String(val)) = self.value.get("generation") {
            return Some(ImplementationGuide_PageGeneration::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.name_url() {}
        if let Some(_val) = self.name_reference() {
            _val.validate();
        }
        if let Some(_val) = self._name_url() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.page() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._generation() {
            _val.validate();
        }
        if let Some(_val) = self.generation() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ImplementationGuide_PageGeneration {
    Html,
    Markdown,
    Xml,
    Generated,
}

impl ImplementationGuide_PageGeneration {
    pub fn from_string(string: &str) -> Option<ImplementationGuide_PageGeneration> {
        match string {
            "html" => Some(ImplementationGuide_PageGeneration::Html),
            "markdown" => Some(ImplementationGuide_PageGeneration::Markdown),
            "xml" => Some(ImplementationGuide_PageGeneration::Xml),
            "generated" => Some(ImplementationGuide_PageGeneration::Generated),
            _ => None,
        }
    }
}

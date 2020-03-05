#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Page<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide_Page<'_> {
    pub fn new(value: &Value) -> ImplementationGuide_Page {
        ImplementationGuide_Page {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for generation
    pub fn _generation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_generation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for nameUrl
    pub fn _name_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_nameUrl") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
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

    /// A code that indicates how the page is generated.
    pub fn generation(&self) -> Option<ImplementationGuide_PageGeneration> {
        if let Some(Value::String(val)) = self.value.get("generation") {
            return Some(ImplementationGuide_PageGeneration::from_string(&val).unwrap());
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

    /// The source address for the page.
    pub fn name_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("nameReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The source address for the page.
    pub fn name_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("nameUrl") {
            return Some(string);
        }
        return None;
    }

    /// Nested Pages/Sections under this page.
    pub fn page(&self) -> Option<Vec<ImplementationGuide_Page>> {
        if let Some(Value::Array(val)) = self.value.get("page") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Page {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._generation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name_url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.generation() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.name_url() {}
        if let Some(_val) = self.page() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuide_PageBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuide_PageBuilder {
    pub fn build(&self) -> ImplementationGuide_Page {
        ImplementationGuide_Page {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide_Page) -> ImplementationGuide_PageBuilder {
        ImplementationGuide_PageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ImplementationGuide_PageBuilder {
        let mut __value: Value = json!({});
        return ImplementationGuide_PageBuilder { value: __value };
    }

    pub fn _generation<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["_generation"] = json!(val.value);
        return self;
    }

    pub fn _name_url<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["_nameUrl"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn generation<'a>(
        &'a mut self,
        val: ImplementationGuide_PageGeneration,
    ) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["generation"] = json!(val.to_string());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["nameReference"] = json!(val.value);
        return self;
    }

    pub fn name_url<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["nameUrl"] = json!(val);
        return self;
    }

    pub fn page<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Page>,
    ) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["page"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_PageBuilder {
        self.value["title"] = json!(val);
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            ImplementationGuide_PageGeneration::Html => "html".to_string(),
            ImplementationGuide_PageGeneration::Markdown => "markdown".to_string(),
            ImplementationGuide_PageGeneration::Xml => "xml".to_string(),
            ImplementationGuide_PageGeneration::Generated => "generated".to_string(),
        }
    }
}

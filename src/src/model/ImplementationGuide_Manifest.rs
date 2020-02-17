#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ImplementationGuide_Page1::ImplementationGuide_Page1;
use crate::model::ImplementationGuide_Resource1::ImplementationGuide_Resource1;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Manifest<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide_Manifest<'_> {
    pub fn new(value: &Value) -> ImplementationGuide_Manifest {
        ImplementationGuide_Manifest {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for image
    pub fn _image(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_image") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for other
    pub fn _other(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_other") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for rendering
    pub fn _rendering(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rendering") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Indicates a relative path to an image that exists within the IG.
    pub fn image(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("image") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the relative path of an additional non-page, non-image file that is
    /// part of the IG - e.g. zip, jar and similar files that could be the target of a
    /// hyperlink in a derived IG.
    pub fn other(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("other") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information about a page within the IG.
    pub fn page(&self) -> Option<Vec<ImplementationGuide_Page1>> {
        if let Some(Value::Array(val)) = self.value.get("page") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Page1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A pointer to official web page, PDF or other rendering of the implementation
    /// guide.
    pub fn rendering(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rendering") {
            return Some(string);
        }
        return None;
    }

    /// A resource that is part of the implementation guide. Conformance resources
    /// (value set, structure definition, capability statements etc.) are obvious
    /// candidates for inclusion, but any kind of resource can be included as an example
    /// resource.
    pub fn resource(&self) -> Vec<ImplementationGuide_Resource1> {
        self.value
            .get("resource")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ImplementationGuide_Resource1 {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._image() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._other() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._rendering() {
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
        if let Some(_val) = self.image() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.other() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.page() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rendering() {}
        if !self
            .resource()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuide_ManifestBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuide_ManifestBuilder {
    pub fn build(&self) -> ImplementationGuide_Manifest {
        ImplementationGuide_Manifest {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide_Manifest) -> ImplementationGuide_ManifestBuilder {
        ImplementationGuide_ManifestBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        resource: Vec<ImplementationGuide_Resource1>,
    ) -> ImplementationGuide_ManifestBuilder {
        let mut __value: Value = json!({});
        __value["resource"] = json!(resource.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return ImplementationGuide_ManifestBuilder { value: __value };
    }

    pub fn _image<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["_image"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _other<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["_other"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _rendering<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["_rendering"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn image<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["image"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn other<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["other"] = json!(val);
        return self;
    }

    pub fn page<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Page1>,
    ) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["page"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rendering<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_ManifestBuilder {
        self.value["rendering"] = json!(val);
        return self;
    }
}

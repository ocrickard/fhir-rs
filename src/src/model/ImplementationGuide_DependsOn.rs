#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_DependsOn<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide_DependsOn<'_> {
    pub fn new(value: &Value) -> ImplementationGuide_DependsOn {
        ImplementationGuide_DependsOn {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for packageId
    pub fn _package_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_packageId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
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

    /// The NPM package name for the Implementation Guide that this IG depends on.
    pub fn package_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("packageId") {
            return Some(string);
        }
        return None;
    }

    /// A canonical reference to the Implementation guide for the dependency.
    pub fn uri(&self) -> &str {
        self.value.get("uri").unwrap().as_str().unwrap()
    }

    /// The version of the IG that is depended on, when the correct version is required
    /// to understand the IG correctly.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._package_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
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
        if let Some(_val) = self.package_id() {}
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuide_DependsOnBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuide_DependsOnBuilder {
    pub fn build(&self) -> ImplementationGuide_DependsOn {
        ImplementationGuide_DependsOn {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide_DependsOn) -> ImplementationGuide_DependsOnBuilder {
        ImplementationGuide_DependsOnBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(uri: &str) -> ImplementationGuide_DependsOnBuilder {
        let mut __value: Value = json!({});
        __value["uri"] = json!(uri);
        return ImplementationGuide_DependsOnBuilder { value: __value };
    }

    pub fn _package_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["_packageId"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn package_id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["packageId"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_DependsOnBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

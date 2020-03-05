#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The findings and interpretation of diagnostic  tests performed on patients,
/// groups of patients, devices, and locations, and/or specimens derived from these.
/// The report includes clinical context such as requesting and provider
/// information, and some mix of atomic results, images, textual and coded
/// interpretations, and formatted representation of diagnostic reports.

#[derive(Debug)]
pub struct DiagnosticReport_Media<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DiagnosticReport_Media<'_> {
    pub fn new(value: &Value) -> DiagnosticReport_Media {
        DiagnosticReport_Media {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A comment about the image. Typically, this is used to provide an explanation for
    /// why the image is included, or to draw the viewer's attention to important
    /// features.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// Reference to the image source.
    pub fn link(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["link"]),
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self.link().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DiagnosticReport_MediaBuilder {
    pub(crate) value: Value,
}

impl DiagnosticReport_MediaBuilder {
    pub fn build(&self) -> DiagnosticReport_Media {
        DiagnosticReport_Media {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DiagnosticReport_Media) -> DiagnosticReport_MediaBuilder {
        DiagnosticReport_MediaBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(link: Reference) -> DiagnosticReport_MediaBuilder {
        let mut __value: Value = json!({});
        __value["link"] = json!(link.value);
        return DiagnosticReport_MediaBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut DiagnosticReport_MediaBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut DiagnosticReport_MediaBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DiagnosticReport_MediaBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DiagnosticReport_MediaBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DiagnosticReport_MediaBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A  text note which also  contains information about who made the statement and
/// when.

#[derive(Debug)]
pub struct Annotation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Annotation<'_> {
    pub fn new(value: &Value) -> Annotation {
        Annotation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for authorString
    pub fn _author_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authorString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for time
    pub fn _time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_time") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The individual responsible for making the annotation.
    pub fn author_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("authorReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The individual responsible for making the annotation.
    pub fn author_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authorString") {
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

    /// The text of the annotation in markdown format.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Indicates when this particular annotation was made.
    pub fn time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("time") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._author_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author_string() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.time() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AnnotationBuilder {
    pub(crate) value: Value,
}

impl AnnotationBuilder {
    pub fn build(&self) -> Annotation {
        Annotation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Annotation) -> AnnotationBuilder {
        AnnotationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AnnotationBuilder {
        let mut __value: Value = json!({});
        return AnnotationBuilder { value: __value };
    }

    pub fn _author_string<'a>(&'a mut self, val: Element) -> &'a mut AnnotationBuilder {
        self.value["_authorString"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut AnnotationBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _time<'a>(&'a mut self, val: Element) -> &'a mut AnnotationBuilder {
        self.value["_time"] = json!(val.value);
        return self;
    }

    pub fn author_reference<'a>(&'a mut self, val: Reference) -> &'a mut AnnotationBuilder {
        self.value["authorReference"] = json!(val.value);
        return self;
    }

    pub fn author_string<'a>(&'a mut self, val: &str) -> &'a mut AnnotationBuilder {
        self.value["authorString"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AnnotationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AnnotationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut AnnotationBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn time<'a>(&'a mut self, val: &str) -> &'a mut AnnotationBuilder {
        self.value["time"] = json!(val);
        return self;
    }
}

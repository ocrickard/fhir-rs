#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeSystem_Designation::CodeSystem_Designation;
use crate::model::CodeSystem_Property1::CodeSystem_Property1;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem_Concept<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CodeSystem_Concept<'_> {
    pub fn new(value: &Value) -> CodeSystem_Concept {
        CodeSystem_Concept {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code - a text symbol - that uniquely identifies the concept within the code
    /// system.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Defines children of a concept to produce a hierarchy of concepts. The nature of
    /// the relationships is variable (is-a/contains/categorizes) - see
    /// hierarchyMeaning.
    pub fn concept(&self) -> Option<Vec<CodeSystem_Concept>> {
        if let Some(Value::Array(val)) = self.value.get("concept") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Concept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The formal definition of the concept. The code system resource does not make
    /// formal definitions required, because of the prevalence of legacy systems.
    /// However, they are highly recommended, as without them there is no formal meaning
    /// associated with the concept.
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// Additional representations for the concept - other languages, aliases,
    /// specialized purposes, used for particular purposes, etc.
    pub fn designation(&self) -> Option<Vec<CodeSystem_Designation>> {
        if let Some(Value::Array(val)) = self.value.get("designation") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Designation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human readable string that is the recommended default way to present this
    /// concept to a user.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// A property value for this concept.
    pub fn property(&self) -> Option<Vec<CodeSystem_Property1>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Property1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.concept() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.designation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.display() {}
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
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CodeSystem_ConceptBuilder {
    pub(crate) value: Value,
}

impl CodeSystem_ConceptBuilder {
    pub fn build(&self) -> CodeSystem_Concept {
        CodeSystem_Concept {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CodeSystem_Concept) -> CodeSystem_ConceptBuilder {
        CodeSystem_ConceptBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CodeSystem_ConceptBuilder {
        let mut __value: Value = json!({});
        return CodeSystem_ConceptBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _definition<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["_definition"] = json!(val.value);
        return self;
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn concept<'a>(
        &'a mut self,
        val: Vec<CodeSystem_Concept>,
    ) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["concept"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn definition<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["definition"] = json!(val);
        return self;
    }

    pub fn designation<'a>(
        &'a mut self,
        val: Vec<CodeSystem_Designation>,
    ) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["designation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<CodeSystem_Property1>,
    ) -> &'a mut CodeSystem_ConceptBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

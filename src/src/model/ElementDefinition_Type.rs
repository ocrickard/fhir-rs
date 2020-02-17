#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Type<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Type<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Type {
        ElementDefinition_Type {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for aggregation
    pub fn _aggregation(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_aggregation") {
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

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for versioning
    pub fn _versioning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versioning") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// URL of Data type or Resource that is a(or the) type used for this element.
    /// References are URLs that are relative to http://hl7.org/fhir/StructureDefinition
    /// e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string.
    /// Absolute URLs are only allowed in logical models.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// Identifies a profile structure or implementation Guide that applies to the
    /// datatype this element refers to. If any profiles are specified, then the content
    /// must conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition or
    /// Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the type SHALL conform to at least one profile defined in the
    /// implementation guide.
    pub fn profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("profile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Used when the type is "Reference" or "canonical", and identifies a profile
    /// structure or implementation Guide that applies to the target of the reference
    /// this element refers to. If any profiles are specified, then the content must
    /// conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition or
    /// Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the target resource SHALL conform to at least one profile defined in
    /// the implementation guide.
    pub fn target_profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("targetProfile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Whether this reference needs to be version specific or version independent, or
    /// whether either can be used.
    pub fn versioning(&self) -> Option<ElementDefinition_TypeVersioning> {
        if let Some(Value::String(val)) = self.value.get("versioning") {
            return Some(ElementDefinition_TypeVersioning::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._aggregation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._versioning() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
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
        if let Some(_val) = self.profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.target_profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.versioning() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_TypeBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_TypeBuilder {
    pub fn build(&self) -> ElementDefinition_Type {
        ElementDefinition_Type {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ElementDefinition_Type) -> ElementDefinition_TypeBuilder {
        ElementDefinition_TypeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_TypeBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_TypeBuilder { value: __value };
    }

    pub fn _aggregation<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["_aggregation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _versioning<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["_versioning"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["profile"] = json!(val);
        return self;
    }

    pub fn target_profile<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["targetProfile"] = json!(val);
        return self;
    }

    pub fn versioning<'a>(
        &'a mut self,
        val: ElementDefinition_TypeVersioning,
    ) -> &'a mut ElementDefinition_TypeBuilder {
        self.value["versioning"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_TypeVersioning {
    Either,
    Independent,
    Specific,
}

impl ElementDefinition_TypeVersioning {
    pub fn from_string(string: &str) -> Option<ElementDefinition_TypeVersioning> {
        match string {
            "either" => Some(ElementDefinition_TypeVersioning::Either),
            "independent" => Some(ElementDefinition_TypeVersioning::Independent),
            "specific" => Some(ElementDefinition_TypeVersioning::Specific),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ElementDefinition_TypeVersioning::Either => "either".to_string(),
            ElementDefinition_TypeVersioning::Independent => "independent".to_string(),
            ElementDefinition_TypeVersioning::Specific => "specific".to_string(),
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::ConceptMap_DependsOn::ConceptMap_DependsOn;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap_Target<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConceptMap_Target<'_> {
    pub fn new(value: &Value) -> ConceptMap_Target {
        ConceptMap_Target {
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

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
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

    /// Extensions for equivalence
    pub fn _equivalence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_equivalence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identity (code or path) or the element/item that the map refers to.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// A description of status/issues in mapping that conveys additional information
    /// not represented in  the structured data.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
        }
        return None;
    }

    /// A set of additional dependencies for this mapping to hold. This mapping is only
    /// applicable if the specified element can be resolved, and it has the specified
    /// value.
    pub fn depends_on(&self) -> Option<Vec<ConceptMap_DependsOn>> {
        if let Some(Value::Array(val)) = self.value.get("dependsOn") {
            return Some(
                val.into_iter()
                    .map(|e| ConceptMap_DependsOn {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The display for the code. The display is only provided to help editors when
    /// editing the concept map.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
            return Some(string);
        }
        return None;
    }

    /// The equivalence between the source and target concepts (counting for the
    /// dependencies and products). The equivalence is read from target to source (e.g.
    /// the target is 'wider' than the source).
    pub fn equivalence(&self) -> Option<ConceptMap_TargetEquivalence> {
        if let Some(Value::String(val)) = self.value.get("equivalence") {
            return Some(ConceptMap_TargetEquivalence::from_string(&val).unwrap());
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

    /// A set of additional outcomes from this mapping to other elements. To properly
    /// execute this mapping, the specified element must be mapped to some data element
    /// or source that is in context. The mapping may still be useful without a place
    /// for the additional data elements, but the equivalence cannot be relied on.
    pub fn product(&self) -> Option<Vec<ConceptMap_DependsOn>> {
        if let Some(Value::Array(val)) = self.value.get("product") {
            return Some(
                val.into_iter()
                    .map(|e| ConceptMap_DependsOn {
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
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._equivalence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.depends_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.equivalence() {}
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
        if let Some(_val) = self.product() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConceptMap_TargetBuilder {
    pub(crate) value: Value,
}

impl ConceptMap_TargetBuilder {
    pub fn build(&self) -> ConceptMap_Target {
        ConceptMap_Target {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConceptMap_Target) -> ConceptMap_TargetBuilder {
        ConceptMap_TargetBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ConceptMap_TargetBuilder {
        let mut __value: Value = json!({});
        return ConceptMap_TargetBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_TargetBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_TargetBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_TargetBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _equivalence<'a>(&'a mut self, val: Element) -> &'a mut ConceptMap_TargetBuilder {
        self.value["_equivalence"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_TargetBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_TargetBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn depends_on<'a>(
        &'a mut self,
        val: Vec<ConceptMap_DependsOn>,
    ) -> &'a mut ConceptMap_TargetBuilder {
        self.value["dependsOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_TargetBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn equivalence<'a>(
        &'a mut self,
        val: ConceptMap_TargetEquivalence,
    ) -> &'a mut ConceptMap_TargetBuilder {
        self.value["equivalence"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConceptMap_TargetBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap_TargetBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConceptMap_TargetBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product<'a>(
        &'a mut self,
        val: Vec<ConceptMap_DependsOn>,
    ) -> &'a mut ConceptMap_TargetBuilder {
        self.value["product"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum ConceptMap_TargetEquivalence {
    Relatedto,
    Equivalent,
    Equal,
    Wider,
    Subsumes,
    Narrower,
    Specializes,
    Inexact,
    Unmatched,
    Disjoint,
}

impl ConceptMap_TargetEquivalence {
    pub fn from_string(string: &str) -> Option<ConceptMap_TargetEquivalence> {
        match string {
            "relatedto" => Some(ConceptMap_TargetEquivalence::Relatedto),
            "equivalent" => Some(ConceptMap_TargetEquivalence::Equivalent),
            "equal" => Some(ConceptMap_TargetEquivalence::Equal),
            "wider" => Some(ConceptMap_TargetEquivalence::Wider),
            "subsumes" => Some(ConceptMap_TargetEquivalence::Subsumes),
            "narrower" => Some(ConceptMap_TargetEquivalence::Narrower),
            "specializes" => Some(ConceptMap_TargetEquivalence::Specializes),
            "inexact" => Some(ConceptMap_TargetEquivalence::Inexact),
            "unmatched" => Some(ConceptMap_TargetEquivalence::Unmatched),
            "disjoint" => Some(ConceptMap_TargetEquivalence::Disjoint),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ConceptMap_TargetEquivalence::Relatedto => "relatedto".to_string(),
            ConceptMap_TargetEquivalence::Equivalent => "equivalent".to_string(),
            ConceptMap_TargetEquivalence::Equal => "equal".to_string(),
            ConceptMap_TargetEquivalence::Wider => "wider".to_string(),
            ConceptMap_TargetEquivalence::Subsumes => "subsumes".to_string(),
            ConceptMap_TargetEquivalence::Narrower => "narrower".to_string(),
            ConceptMap_TargetEquivalence::Specializes => "specializes".to_string(),
            ConceptMap_TargetEquivalence::Inexact => "inexact".to_string(),
            ConceptMap_TargetEquivalence::Unmatched => "unmatched".to_string(),
            ConceptMap_TargetEquivalence::Disjoint => "disjoint".to_string(),
        }
    }
}

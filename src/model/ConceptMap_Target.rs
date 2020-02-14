#![allow(unused_imports, non_camel_case_types)]

use crate::model::ConceptMap_DependsOn::ConceptMap_DependsOn;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap_Target<'a> {
    pub value: &'a Value,
}

impl ConceptMap_Target<'_> {
    /// A description of status/issues in mapping that conveys additional information
    /// not represented in  the structured data.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
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
                    .map(|e| ConceptMap_DependsOn { value: e })
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element { value: val });
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

    /// A set of additional dependencies for this mapping to hold. This mapping is only
    /// applicable if the specified element can be resolved, and it has the specified
    /// value.
    pub fn depends_on(&self) -> Option<Vec<ConceptMap_DependsOn>> {
        if let Some(Value::Array(val)) = self.value.get("dependsOn") {
            return Some(
                val.into_iter()
                    .map(|e| ConceptMap_DependsOn { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
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

    /// Extensions for equivalence
    pub fn _equivalence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_equivalence") {
            return Some(Element { value: val });
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

    /// The equivalence between the source and target concepts (counting for the
    /// dependencies and products). The equivalence is read from target to source (e.g.
    /// the target is 'wider' than the source).
    pub fn equivalence(&self) -> Option<ConceptMap_TargetEquivalence> {
        if let Some(Value::String(val)) = self.value.get("equivalence") {
            return Some(ConceptMap_TargetEquivalence::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.product() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._display() {
            _val.validate();
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.depends_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._equivalence() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.equivalence() {}
        return true;
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
}

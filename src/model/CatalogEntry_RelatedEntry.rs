#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Catalog entries are wrappers that contextualize items included in a catalog.

#[derive(Debug)]
pub struct CatalogEntry_RelatedEntry<'a> {
    pub value: &'a Value,
}

impl CatalogEntry_RelatedEntry<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The reference to the related item.
    pub fn item(&self) -> Reference {
        Reference {
            value: &self.value["item"],
        }
    }

    /// The type of relation to the related item: child, parent, packageContent,
    /// containerPackage, usedIn, uses, requires, etc.
    pub fn relationtype(&self) -> Option<CatalogEntry_RelatedEntryRelationtype> {
        if let Some(Value::String(val)) = self.value.get("relationtype") {
            return Some(CatalogEntry_RelatedEntryRelationtype::from_string(&val).unwrap());
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

    /// Extensions for relationtype
    pub fn _relationtype(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relationtype") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        let _ = self.item().validate();
        if let Some(_val) = self.relationtype() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._relationtype() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum CatalogEntry_RelatedEntryRelationtype {
    Triggers,
    IsReplacedBy,
}

impl CatalogEntry_RelatedEntryRelationtype {
    pub fn from_string(string: &str) -> Option<CatalogEntry_RelatedEntryRelationtype> {
        match string {
            "triggers" => Some(CatalogEntry_RelatedEntryRelationtype::Triggers),
            "is-replaced-by" => Some(CatalogEntry_RelatedEntryRelationtype::IsReplacedBy),
            _ => None,
        }
    }
}

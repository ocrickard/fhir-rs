#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::value::Value;

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.

#[derive(Debug)]
pub struct SubstanceNucleicAcid_Linkage<'a> {
    pub value: &'a Value,
}

impl SubstanceNucleicAcid_Linkage<'_> {
    /// Extensions for connectivity
    pub fn _connectivity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_connectivity") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for residueSite
    pub fn _residue_site(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_residueSite") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The entity that links the sugar residues together should also be captured for
    /// nearly all naturally occurring nucleic acid the linkage is a phosphate group.
    /// For many synthetic oligonucleotides phosphorothioate linkages are often seen.
    /// Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or
    /// 5’-5’ this should be specified.
    pub fn connectivity(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("connectivity") {
            return Some(string);
        }
        return None;
    }

    /// Each linkage will be registered as a fragment and have at least one name. A
    /// single name shall be assigned to each linkage.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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
                    .map(|e| Extension { value: e })
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

    /// Each linkage will be registered as a fragment and have an ID.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Residues shall be captured as described in 5.3.6.8.3.
    pub fn residue_site(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("residueSite") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._connectivity() {
            _val.validate();
        }
        if let Some(_val) = self._residue_site() {
            _val.validate();
        }
        if let Some(_val) = self.connectivity() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.residue_site() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

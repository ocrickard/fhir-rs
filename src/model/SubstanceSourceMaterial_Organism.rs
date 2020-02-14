#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstanceSourceMaterial_Author::SubstanceSourceMaterial_Author;
use crate::model::SubstanceSourceMaterial_Hybrid::SubstanceSourceMaterial_Hybrid;
use crate::model::SubstanceSourceMaterial_OrganismGeneral::SubstanceSourceMaterial_OrganismGeneral;
use serde_json::value::Value;

/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to define
/// polymer substances isolated from biological matrices. Taxonomic and anatomical
/// origins shall be described using a controlled vocabulary as required. This
/// information is captured for naturally derived polymers ( . starch) and
/// structurally diverse substances. For Organisms belonging to the Kingdom Plantae
/// the Substance level defines the fresh material of a single species or
/// infraspecies, the Herbal Drug and the Herbal preparation. For Herbal
/// preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.

#[derive(Debug)]
pub struct SubstanceSourceMaterial_Organism<'a> {
    pub value: &'a Value,
}

impl SubstanceSourceMaterial_Organism<'_> {
    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional).
    pub fn hybrid(&self) -> Option<SubstanceSourceMaterial_Hybrid> {
        if let Some(val) = self.value.get("hybrid") {
            return Some(SubstanceSourceMaterial_Hybrid { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The intraspecific description of an organism shall be specified based on a
    /// controlled vocabulary. For Influenza Vaccine, the intraspecific description
    /// shall contain the syntax of the antigen in line with the WHO convention.
    pub fn intraspecific_description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intraspecificDescription") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for intraspecificDescription
    pub fn _intraspecific_description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intraspecificDescription") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// 4.9.13.7.1 Kingdom (Conditional).
    pub fn organism_general(&self) -> Option<SubstanceSourceMaterial_OrganismGeneral> {
        if let Some(val) = self.value.get("organismGeneral") {
            return Some(SubstanceSourceMaterial_OrganismGeneral { value: val });
        }
        return None;
    }

    /// The family of an organism shall be specified.
    pub fn family(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("family") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The species of an organism shall be specified; refers to the Latin epithet of
    /// the species of the plant/animal; it is present in names for species and
    /// infraspecies.
    pub fn species(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("species") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The Intraspecific type of an organism shall be specified.
    pub fn intraspecific_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("intraspecificType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The genus of an organism shall be specified; refers to the Latin epithet of the
    /// genus element of the plant/animal scientific name; it is present in names for
    /// genera, species and infraspecies.
    pub fn genus(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("genus") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// 4.9.13.6.1 Author type (Conditional).
    pub fn author(&self) -> Option<Vec<SubstanceSourceMaterial_Author>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSourceMaterial_Author { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.hybrid() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.intraspecific_description() {}
        if let Some(_val) = self._intraspecific_description() {
            _val.validate();
        }
        if let Some(_val) = self.organism_general() {
            _val.validate();
        }
        if let Some(_val) = self.family() {
            _val.validate();
        }
        if let Some(_val) = self.species() {
            _val.validate();
        }
        if let Some(_val) = self.intraspecific_type() {
            _val.validate();
        }
        if let Some(_val) = self.genus() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

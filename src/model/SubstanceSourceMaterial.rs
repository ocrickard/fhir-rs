#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceSourceMaterial_FractionDescription::SubstanceSourceMaterial_FractionDescription;
use crate::model::SubstanceSourceMaterial_Organism::SubstanceSourceMaterial_Organism;
use crate::model::SubstanceSourceMaterial_PartDescription::SubstanceSourceMaterial_PartDescription;
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
pub struct SubstanceSourceMaterial<'a> {
    pub value: &'a Value,
}

impl SubstanceSourceMaterial<'_> {
    /// General high level classification of the source material specific to the origin
    /// of the material.
    pub fn source_material_class(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialClass") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The parent substance of the Herbal Drug, or Herbal preparation.
    pub fn parent_substance_name(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("parentSubstanceName") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The unique identifier associated with the source material parent organism shall
    /// be specified.
    pub fn organism_id(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("organismId") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Extensions for parentSubstanceName
    pub fn _parent_substance_name(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_parentSubstanceName") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The country where the plant material is harvested or the countries where the
    /// plasma is sourced from as laid down in accordance with the Plasma Master File.
    /// For “Plasma-derived substances” the attribute country of origin provides
    /// information about the countries used for the manufacturing of the Cryopoor plama
    /// or Crioprecipitate.
    pub fn country_of_origin(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("countryOfOrigin") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for geographicalLocation
    pub fn _geographical_location(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_geographicalLocation") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organism accepted Scientific name shall be provided based on the organism
    /// taxonomy.
    pub fn organism_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("organismName") {
            return Some(string);
        }
        return None;
    }

    /// Many complex materials are fractions of parts of plants, animals, or minerals.
    /// Fraction elements are often necessary to define both Substances and Specified
    /// Group 1 Substances. For substances derived from Plants, fraction information
    /// will be captured at the Substance information level ( . Oils, Juices and
    /// Exudates). Additional information for Extracts, such as extraction solvent
    /// composition, will be captured at the Specified Substance Group 1 information
    /// level. For plasma-derived products fraction information will be captured at the
    /// Substance and the Specified Substance Group 1 levels.
    pub fn fraction_description(&self) -> Option<Vec<SubstanceSourceMaterial_FractionDescription>> {
        if let Some(Value::Array(val)) = self.value.get("fractionDescription") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSourceMaterial_FractionDescription { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// This subclause describes the organism which the substance is derived from. For
    /// vaccines, the parent organism shall be specified based on these subclause
    /// elements. As an example, full taxonomy will be described for the Substance Name:
    /// ., Leaf.
    pub fn organism(&self) -> Option<SubstanceSourceMaterial_Organism> {
        if let Some(val) = self.value.get("organism") {
            return Some(SubstanceSourceMaterial_Organism { value: val });
        }
        return None;
    }

    /// To do.
    pub fn part_description(&self) -> Option<Vec<SubstanceSourceMaterial_PartDescription>> {
        if let Some(Value::Array(val)) = self.value.get("partDescription") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSourceMaterial_PartDescription { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// Extensions for organismName
    pub fn _organism_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_organismName") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the
    /// substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant).
    pub fn parent_substance_id(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("parentSubstanceId") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// The state of the source material when extracted.
    pub fn source_material_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialState") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The place/region where the plant is harvested or the places/regions where the
    /// animal source material has its habitat.
    pub fn geographical_location(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("geographicalLocation") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Stage of life for animals, plants, insects and microorganisms. This information
    /// shall be provided only when the substance is significantly different in these
    /// stages (e.g. foetal bovine serum).
    pub fn development_stage(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("developmentStage") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The type of the source material shall be specified based on a controlled
    /// vocabulary. For vaccines, this subclause refers to the class of infectious
    /// agent.
    pub fn source_material_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.source_material_class() {
            _val.validate();
        }
        if let Some(_val) = self.parent_substance_name() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.organism_id() {
            _val.validate();
        }
        if let Some(_val) = self._parent_substance_name() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.country_of_origin() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._geographical_location() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.organism_name() {}
        if let Some(_val) = self.fraction_description() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.organism() {
            _val.validate();
        }
        if let Some(_val) = self.part_description() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._organism_name() {
            _val.validate();
        }
        if let Some(_val) = self.parent_substance_id() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.source_material_state() {
            _val.validate();
        }
        if let Some(_val) = self.geographical_location() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.development_stage() {
            _val.validate();
        }
        if let Some(_val) = self.source_material_type() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

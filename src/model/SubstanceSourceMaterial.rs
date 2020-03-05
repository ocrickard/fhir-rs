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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

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
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSourceMaterial<'_> {
    pub fn new(value: &Value) -> SubstanceSourceMaterial {
        SubstanceSourceMaterial {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for geographicalLocation
    pub fn _geographical_location(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_geographicalLocation") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for organismName
    pub fn _organism_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_organismName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for parentSubstanceName
    pub fn _parent_substance_name(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_parentSubstanceName") {
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

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| SubstanceSourceMaterial_FractionDescription {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(SubstanceSourceMaterial_Organism {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The unique identifier associated with the source material parent organism shall
    /// be specified.
    pub fn organism_id(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("organismId") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the
    /// substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant).
    pub fn parent_substance_id(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("parentSubstanceId") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// To do.
    pub fn part_description(&self) -> Option<Vec<SubstanceSourceMaterial_PartDescription>> {
        if let Some(Value::Array(val)) = self.value.get("partDescription") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSourceMaterial_PartDescription {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// General high level classification of the source material specific to the origin
    /// of the material.
    pub fn source_material_class(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialClass") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The state of the source material when extracted.
    pub fn source_material_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialState") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of the source material shall be specified based on a controlled
    /// vocabulary. For vaccines, this subclause refers to the class of infectious
    /// agent.
    pub fn source_material_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sourceMaterialType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._geographical_location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._organism_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._parent_substance_name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.country_of_origin() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.development_stage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fraction_description() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.geographical_location() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.organism() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.organism_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.organism_name() {}
        if let Some(_val) = self.parent_substance_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.parent_substance_name() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.part_description() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source_material_class() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source_material_state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source_material_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSourceMaterialBuilder {
    pub(crate) value: Value,
}

impl SubstanceSourceMaterialBuilder {
    pub fn build(&self) -> SubstanceSourceMaterial {
        SubstanceSourceMaterial {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSourceMaterial) -> SubstanceSourceMaterialBuilder {
        SubstanceSourceMaterialBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSourceMaterialBuilder {
        let mut __value: Value = json!({});
        return SubstanceSourceMaterialBuilder { value: __value };
    }

    pub fn _geographical_location<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["_geographicalLocation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _organism_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["_organismName"] = json!(val.value);
        return self;
    }

    pub fn _parent_substance_name<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["_parentSubstanceName"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn country_of_origin<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["countryOfOrigin"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn development_stage<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["developmentStage"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fraction_description<'a>(
        &'a mut self,
        val: Vec<SubstanceSourceMaterial_FractionDescription>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["fractionDescription"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn geographical_location<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["geographicalLocation"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn organism<'a>(
        &'a mut self,
        val: SubstanceSourceMaterial_Organism,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["organism"] = json!(val.value);
        return self;
    }

    pub fn organism_id<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["organismId"] = json!(val.value);
        return self;
    }

    pub fn organism_name<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["organismName"] = json!(val);
        return self;
    }

    pub fn parent_substance_id<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["parentSubstanceId"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parent_substance_name<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["parentSubstanceName"] = json!(val);
        return self;
    }

    pub fn part_description<'a>(
        &'a mut self,
        val: Vec<SubstanceSourceMaterial_PartDescription>,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["partDescription"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source_material_class<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["sourceMaterialClass"] = json!(val.value);
        return self;
    }

    pub fn source_material_state<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["sourceMaterialState"] = json!(val.value);
        return self;
    }

    pub fn source_material_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["sourceMaterialType"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubstanceSourceMaterialBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

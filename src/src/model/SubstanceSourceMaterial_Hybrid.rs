#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
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
pub struct SubstanceSourceMaterial_Hybrid<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSourceMaterial_Hybrid<'_> {
    pub fn new(value: &Value) -> SubstanceSourceMaterial_Hybrid {
        SubstanceSourceMaterial_Hybrid {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for maternalOrganismId
    pub fn _maternal_organism_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maternalOrganismId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for maternalOrganismName
    pub fn _maternal_organism_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maternalOrganismName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for paternalOrganismId
    pub fn _paternal_organism_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paternalOrganismId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for paternalOrganismName
    pub fn _paternal_organism_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paternalOrganismName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// The hybrid type of an organism shall be specified.
    pub fn hybrid_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("hybridType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The identifier of the maternal species constituting the hybrid organism shall be
    /// specified based on a controlled vocabulary. For plants, the parents aren’t
    /// always known, and it is unlikely that it will be known which is maternal and
    /// which is paternal.
    pub fn maternal_organism_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maternalOrganismId") {
            return Some(string);
        }
        return None;
    }

    /// The name of the maternal species constituting the hybrid organism shall be
    /// specified. For plants, the parents aren’t always known, and it is unlikely that
    /// it will be known which is maternal and which is paternal.
    pub fn maternal_organism_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maternalOrganismName") {
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

    /// The identifier of the paternal species constituting the hybrid organism shall be
    /// specified based on a controlled vocabulary.
    pub fn paternal_organism_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("paternalOrganismId") {
            return Some(string);
        }
        return None;
    }

    /// The name of the paternal species constituting the hybrid organism shall be
    /// specified.
    pub fn paternal_organism_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("paternalOrganismName") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._maternal_organism_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._maternal_organism_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._paternal_organism_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._paternal_organism_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.hybrid_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.maternal_organism_id() {}
        if let Some(_val) = self.maternal_organism_name() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.paternal_organism_id() {}
        if let Some(_val) = self.paternal_organism_name() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSourceMaterial_HybridBuilder {
    pub(crate) value: Value,
}

impl SubstanceSourceMaterial_HybridBuilder {
    pub fn build(&self) -> SubstanceSourceMaterial_Hybrid {
        SubstanceSourceMaterial_Hybrid {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSourceMaterial_Hybrid) -> SubstanceSourceMaterial_HybridBuilder {
        SubstanceSourceMaterial_HybridBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSourceMaterial_HybridBuilder {
        let mut __value: Value = json!({});
        return SubstanceSourceMaterial_HybridBuilder { value: __value };
    }

    pub fn _maternal_organism_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["_maternalOrganismId"] = json!(val.value);
        return self;
    }

    pub fn _maternal_organism_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["_maternalOrganismName"] = json!(val.value);
        return self;
    }

    pub fn _paternal_organism_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["_paternalOrganismId"] = json!(val.value);
        return self;
    }

    pub fn _paternal_organism_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["_paternalOrganismName"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn hybrid_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["hybridType"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn maternal_organism_id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["maternalOrganismId"] = json!(val);
        return self;
    }

    pub fn maternal_organism_name<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["maternalOrganismName"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn paternal_organism_id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["paternalOrganismId"] = json!(val);
        return self;
    }

    pub fn paternal_organism_name<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSourceMaterial_HybridBuilder {
        self.value["paternalOrganismName"] = json!(val);
        return self;
    }
}

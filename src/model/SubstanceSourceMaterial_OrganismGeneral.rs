#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
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
pub struct SubstanceSourceMaterial_OrganismGeneral<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSourceMaterial_OrganismGeneral<'_> {
    pub fn new(value: &Value) -> SubstanceSourceMaterial_OrganismGeneral {
        SubstanceSourceMaterial_OrganismGeneral {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The class of an organism shall be specified.
    pub fn class(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("class") {
            return Some(CodeableConcept {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The kingdom of an organism shall be specified.
    pub fn kingdom(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("kingdom") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The order of an organism shall be specified,.
    pub fn order(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("order") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The phylum of an organism shall be specified.
    pub fn phylum(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("phylum") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.class() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.kingdom() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.order() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.phylum() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSourceMaterial_OrganismGeneralBuilder {
    pub(crate) value: Value,
}

impl SubstanceSourceMaterial_OrganismGeneralBuilder {
    pub fn build(&self) -> SubstanceSourceMaterial_OrganismGeneral {
        SubstanceSourceMaterial_OrganismGeneral {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceSourceMaterial_OrganismGeneral,
    ) -> SubstanceSourceMaterial_OrganismGeneralBuilder {
        SubstanceSourceMaterial_OrganismGeneralBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSourceMaterial_OrganismGeneralBuilder {
        let mut __value: Value = json!({});
        return SubstanceSourceMaterial_OrganismGeneralBuilder { value: __value };
    }

    pub fn class<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["class"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn kingdom<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["kingdom"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn order<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["order"] = json!(val.value);
        return self;
    }

    pub fn phylum<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSourceMaterial_OrganismGeneralBuilder {
        self.value["phylum"] = json!(val.value);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_AdministrationGuidelines::MedicationKnowledge_AdministrationGuidelines;
use crate::model::MedicationKnowledge_Cost::MedicationKnowledge_Cost;
use crate::model::MedicationKnowledge_DrugCharacteristic::MedicationKnowledge_DrugCharacteristic;
use crate::model::MedicationKnowledge_Ingredient::MedicationKnowledge_Ingredient;
use crate::model::MedicationKnowledge_Kinetics::MedicationKnowledge_Kinetics;
use crate::model::MedicationKnowledge_MedicineClassification::MedicationKnowledge_MedicineClassification;
use crate::model::MedicationKnowledge_MonitoringProgram::MedicationKnowledge_MonitoringProgram;
use crate::model::MedicationKnowledge_Monograph::MedicationKnowledge_Monograph;
use crate::model::MedicationKnowledge_Packaging::MedicationKnowledge_Packaging;
use crate::model::MedicationKnowledge_Regulatory::MedicationKnowledge_Regulatory;
use crate::model::MedicationKnowledge_RelatedMedicationKnowledge::MedicationKnowledge_RelatedMedicationKnowledge;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge {
        MedicationKnowledge {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for preparationInstruction
    pub fn _preparation_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preparationInstruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for synonym
    pub fn _synonym(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_synonym") {
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

    /// Guidelines for the administration of the medication.
    pub fn administration_guidelines(
        &self,
    ) -> Option<Vec<MedicationKnowledge_AdministrationGuidelines>> {
        if let Some(Value::Array(val)) = self.value.get("administrationGuidelines") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_AdministrationGuidelines {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specific amount of the drug in the packaged product.  For example, when
    /// specifying a product that has the same strength (For example, Insulin glargine
    /// 100 unit per mL solution for injection), this attribute provides additional
    /// clarification of the package amount (For example, 3 mL, 10mL, etc.).
    pub fn amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amount") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Associated or related medications.  For example, if the medication is a branded
    /// product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if
    /// this is a generic medication (e.g. Rosuvastatin), this would link to a branded
    /// product (e.g. Crestor).
    pub fn associated_medication(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("associatedMedication") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code that specifies this medication, or a textual description if no code is
    /// available. Usage note: This could be a standard medication code such as a code
    /// from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary
    /// code, optionally with translations to other code systems.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Potential clinical issue with or between medication(s) (for example, drug-drug
    /// interaction, drug-disease contraindication, drug-allergy interaction, etc.).
    pub fn contraindication(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("contraindication") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The price of the medication.
    pub fn cost(&self) -> Option<Vec<MedicationKnowledge_Cost>> {
        if let Some(Value::Array(val)) = self.value.get("cost") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Cost {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes the form of the item.  Powder; tablets; capsule.
    pub fn dose_form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("doseForm") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies descriptive properties of the medicine, such as color, shape,
    /// imprints, etc.
    pub fn drug_characteristic(&self) -> Option<Vec<MedicationKnowledge_DrugCharacteristic>> {
        if let Some(Value::Array(val)) = self.value.get("drugCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_DrugCharacteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Identifies a particular constituent of interest in the product.
    pub fn ingredient(&self) -> Option<Vec<MedicationKnowledge_Ingredient>> {
        if let Some(Value::Array(val)) = self.value.get("ingredient") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Ingredient {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The intended or approved route of administration.
    pub fn intended_route(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("intendedRoute") {
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

    /// The time course of drug absorption, distribution, metabolism and excretion of a
    /// medication from the body.
    pub fn kinetics(&self) -> Option<Vec<MedicationKnowledge_Kinetics>> {
        if let Some(Value::Array(val)) = self.value.get("kinetics") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Kinetics {
                        value: Cow::Borrowed(e),
                    })
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

    /// Describes the details of the manufacturer of the medication product.  This is
    /// not intended to represent the distributor of a medication product.
    pub fn manufacturer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("manufacturer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Categorization of the medication within a formulary or classification system.
    pub fn medicine_classification(
        &self,
    ) -> Option<Vec<MedicationKnowledge_MedicineClassification>> {
        if let Some(Value::Array(val)) = self.value.get("medicineClassification") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_MedicineClassification {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The program under which the medication is reviewed.
    pub fn monitoring_program(&self) -> Option<Vec<MedicationKnowledge_MonitoringProgram>> {
        if let Some(Value::Array(val)) = self.value.get("monitoringProgram") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_MonitoringProgram {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Associated documentation about the medication.
    pub fn monograph(&self) -> Option<Vec<MedicationKnowledge_Monograph>> {
        if let Some(Value::Array(val)) = self.value.get("monograph") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Monograph {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information that only applies to packages (not products).
    pub fn packaging(&self) -> Option<MedicationKnowledge_Packaging> {
        if let Some(val) = self.value.get("packaging") {
            return Some(MedicationKnowledge_Packaging {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The instructions for preparing the medication.
    pub fn preparation_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preparationInstruction") {
            return Some(string);
        }
        return None;
    }

    /// Category of the medication or product (e.g. branded product, therapeutic moeity,
    /// generic product, innovator product, etc.).
    pub fn product_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("productType") {
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

    /// Regulatory information about a medication.
    pub fn regulatory(&self) -> Option<Vec<MedicationKnowledge_Regulatory>> {
        if let Some(Value::Array(val)) = self.value.get("regulatory") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Regulatory {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Associated or related knowledge about a medication.
    pub fn related_medication_knowledge(
        &self,
    ) -> Option<Vec<MedicationKnowledge_RelatedMedicationKnowledge>> {
        if let Some(Value::Array(val)) = self.value.get("relatedMedicationKnowledge") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_RelatedMedicationKnowledge {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code to indicate if the medication is in active use.  The status refers to the
    /// validity about the information of the medication and not to its medicinal
    /// properties.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Additional names for a medication, for example, the name(s) given to a
    /// medication in different countries.  For example, acetaminophen and paracetamol
    /// or salbutamol and albuterol.
    pub fn synonym(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("synonym") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self._preparation_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._synonym() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.administration_guidelines() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.associated_medication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contraindication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cost() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dose_form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.drug_characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.intended_route() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.kinetics() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.manufacturer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.medicine_classification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.monitoring_program() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.monograph() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.packaging() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.preparation_instruction() {}
        if let Some(_val) = self.product_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.regulatory() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.related_medication_knowledge() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.synonym() {
            _val.into_iter().for_each(|_e| {});
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
pub struct MedicationKnowledgeBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledgeBuilder {
    pub fn build(&self) -> MedicationKnowledge {
        MedicationKnowledge {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationKnowledge) -> MedicationKnowledgeBuilder {
        MedicationKnowledgeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledgeBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledgeBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut MedicationKnowledgeBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MedicationKnowledgeBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _preparation_instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["_preparationInstruction"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut MedicationKnowledgeBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _synonym<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MedicationKnowledgeBuilder {
        self.value["_synonym"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn administration_guidelines<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_AdministrationGuidelines>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["administrationGuidelines"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn amount<'a>(&'a mut self, val: Quantity) -> &'a mut MedicationKnowledgeBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn associated_medication<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["associatedMedication"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MedicationKnowledgeBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contraindication<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["contraindication"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cost<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Cost>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["cost"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dose_form<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MedicationKnowledgeBuilder {
        self.value["doseForm"] = json!(val.value);
        return self;
    }

    pub fn drug_characteristic<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_DrugCharacteristic>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["drugCharacteristic"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MedicationKnowledgeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledgeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledgeBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Ingredient>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn intended_route<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["intendedRoute"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn kinetics<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Kinetics>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["kinetics"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledgeBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(&'a mut self, val: Reference) -> &'a mut MedicationKnowledgeBuilder {
        self.value["manufacturer"] = json!(val.value);
        return self;
    }

    pub fn medicine_classification<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_MedicineClassification>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["medicineClassification"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicationKnowledgeBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn monitoring_program<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_MonitoringProgram>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["monitoringProgram"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn monograph<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Monograph>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["monograph"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn packaging<'a>(
        &'a mut self,
        val: MedicationKnowledge_Packaging,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["packaging"] = json!(val.value);
        return self;
    }

    pub fn preparation_instruction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["preparationInstruction"] = json!(val);
        return self;
    }

    pub fn product_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["productType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn regulatory<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Regulatory>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["regulatory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn related_medication_knowledge<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_RelatedMedicationKnowledge>,
    ) -> &'a mut MedicationKnowledgeBuilder {
        self.value["relatedMedicationKnowledge"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledgeBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn synonym<'a>(&'a mut self, val: Vec<&str>) -> &'a mut MedicationKnowledgeBuilder {
        self.value["synonym"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicationKnowledgeBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

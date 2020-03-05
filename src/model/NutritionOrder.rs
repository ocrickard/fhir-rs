#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::NutritionOrder_EnteralFormula::NutritionOrder_EnteralFormula;
use crate::model::NutritionOrder_OralDiet::NutritionOrder_OralDiet;
use crate::model::NutritionOrder_Supplement::NutritionOrder_Supplement;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder<'_> {
    pub fn new(value: &Value) -> NutritionOrder {
        NutritionOrder {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateTime
    pub fn _date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for instantiates
    pub fn _instantiates(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiates") {
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

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
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

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A link to a record of allergies or intolerances  which should be included in the
    /// nutrition order.
    pub fn allergy_intolerance(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("allergyIntolerance") {
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

    /// The date and time that this nutrition order was requested.
    pub fn date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateTime") {
            return Some(string);
        }
        return None;
    }

    /// An encounter that provides additional information about the healthcare context
    /// in which this request is made.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Feeding provided through the gastrointestinal tract via a tube, catheter, or
    /// stoma that delivers nutrition distal to the oral cavity.
    pub fn enteral_formula(&self) -> Option<NutritionOrder_EnteralFormula> {
        if let Some(val) = self.value.get("enteralFormula") {
            return Some(NutritionOrder_EnteralFormula {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This modifier is used to convey Order-specific modifier about the type of oral
    /// food or oral fluids that should not be given. These can be derived from patient
    /// allergies, intolerances, or preferences such as No Red Meat, No Soy or No Wheat
    /// or  Gluten-Free.  While it should not be necessary to repeat allergy or
    /// intolerance information captured in the referenced AllergyIntolerance resource
    /// in the excludeFoodModifier, this element may be used to convey additional
    /// specificity related to foods that should be eliminated from the patient’s diet
    /// for any reason.  This modifier applies to the entire nutrition order inclusive
    /// of the oral diet, nutritional supplements and enteral formula feedings.
    pub fn exclude_food_modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("excludeFoodModifier") {
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

    /// This modifier is used to convey order-specific modifiers about the type of food
    /// that should be given. These can be derived from patient allergies, intolerances,
    /// or preferences such as Halal, Vegan or Kosher. This modifier applies to the
    /// entire nutrition order inclusive of the oral diet, nutritional supplements and
    /// enteral formula feedings.
    pub fn food_preference_modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("foodPreferenceModifier") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Identifiers assigned to this order by the order sender or by the order receiver.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
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

    /// The URL pointing to a protocol, guideline, orderset or other definition that is
    /// adhered to in whole or in part by this NutritionOrder.
    pub fn instantiates(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiates") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this NutritionOrder.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this NutritionOrder.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the level of authority/intentionality associated with the NutrionOrder
    /// and where the request fits into the workflow chain.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
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

    /// Comments made about the {{title}} by the requester, performer, subject or other
    /// participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Diet given orally in contrast to enteral (tube) feeding.
    pub fn oral_diet(&self) -> Option<NutritionOrder_OralDiet> {
        if let Some(val) = self.value.get("oralDiet") {
            return Some(NutritionOrder_OralDiet {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The practitioner that holds legal responsibility for ordering the diet,
    /// nutritional supplement, or formula feedings.
    pub fn orderer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("orderer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The person (patient) who needs the nutrition order for an oral diet, nutritional
    /// supplement and/or enteral or formula feeding.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// The workflow status of the nutrition order/request.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Oral nutritional products given in order to add further nutritional value to the
    /// patient's diet.
    pub fn supplement(&self) -> Option<Vec<NutritionOrder_Supplement>> {
        if let Some(Value::Array(val)) = self.value.get("supplement") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionOrder_Supplement {
                        value: Cow::Borrowed(e),
                    })
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
        if let Some(_val) = self._date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._intent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.allergy_intolerance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_time() {}
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.enteral_formula() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exclude_food_modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.food_preference_modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.instantiates() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.intent() {}
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.oral_diet() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.orderer() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.supplement() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct NutritionOrderBuilder {
    pub(crate) value: Value,
}

impl NutritionOrderBuilder {
    pub fn build(&self) -> NutritionOrder {
        NutritionOrder {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder) -> NutritionOrderBuilder {
        NutritionOrderBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(patient: Reference) -> NutritionOrderBuilder {
        let mut __value: Value = json!({});
        __value["patient"] = json!(patient.value);
        return NutritionOrderBuilder { value: __value };
    }

    pub fn _date_time<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrderBuilder {
        self.value["_dateTime"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrderBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instantiates<'a>(&'a mut self, val: Vec<Element>) -> &'a mut NutritionOrderBuilder {
        self.value["_instantiates"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _instantiates_uri<'a>(&'a mut self, val: Vec<Element>) -> &'a mut NutritionOrderBuilder {
        self.value["_instantiatesUri"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _intent<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrderBuilder {
        self.value["_intent"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrderBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrderBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn allergy_intolerance<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["allergyIntolerance"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut NutritionOrderBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_time<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["dateTime"] = json!(val);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut NutritionOrderBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn enteral_formula<'a>(
        &'a mut self,
        val: NutritionOrder_EnteralFormula,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["enteralFormula"] = json!(val.value);
        return self;
    }

    pub fn exclude_food_modifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["excludeFoodModifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut NutritionOrderBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn food_preference_modifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["foodPreferenceModifier"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut NutritionOrderBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn instantiates<'a>(&'a mut self, val: Vec<&str>) -> &'a mut NutritionOrderBuilder {
        self.value["instantiates"] = json!(val);
        return self;
    }

    pub fn instantiates_canonical<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["instantiatesCanonical"] = json!(val);
        return self;
    }

    pub fn instantiates_uri<'a>(&'a mut self, val: Vec<&str>) -> &'a mut NutritionOrderBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn intent<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["intent"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut NutritionOrderBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut NutritionOrderBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn oral_diet<'a>(
        &'a mut self,
        val: NutritionOrder_OralDiet,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["oralDiet"] = json!(val.value);
        return self;
    }

    pub fn orderer<'a>(&'a mut self, val: Reference) -> &'a mut NutritionOrderBuilder {
        self.value["orderer"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrderBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn supplement<'a>(
        &'a mut self,
        val: Vec<NutritionOrder_Supplement>,
    ) -> &'a mut NutritionOrderBuilder {
        self.value["supplement"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut NutritionOrderBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

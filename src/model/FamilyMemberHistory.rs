#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::FamilyMemberHistory_Condition::FamilyMemberHistory_Condition;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.

#[derive(Debug)]
pub struct FamilyMemberHistory<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl FamilyMemberHistory<'_> {
    pub fn new(value: &Value) -> FamilyMemberHistory {
        FamilyMemberHistory {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for ageString
    pub fn _age_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ageString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for bornDate
    pub fn _born_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_bornDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for bornString
    pub fn _born_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_bornString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deceasedBoolean
    pub fn _deceased_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deceasedBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deceasedDate
    pub fn _deceased_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deceasedDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deceasedString
    pub fn _deceased_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deceasedString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for estimatedAge
    pub fn _estimated_age(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_estimatedAge") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    /// The age of the relative at the time the family member history is recorded.
    pub fn age_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("ageAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The age of the relative at the time the family member history is recorded.
    pub fn age_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("ageRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The age of the relative at the time the family member history is recorded.
    pub fn age_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ageString") {
            return Some(string);
        }
        return None;
    }

    /// The actual or approximate date of birth of the relative.
    pub fn born_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("bornDate") {
            return Some(string);
        }
        return None;
    }

    /// The actual or approximate date of birth of the relative.
    pub fn born_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("bornPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual or approximate date of birth of the relative.
    pub fn born_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("bornString") {
            return Some(string);
        }
        return None;
    }

    /// The significant Conditions (or condition) that the family member had. This is a
    /// repeating section to allow a system to represent more than one condition per
    /// resource, though there is nothing stopping multiple resources - one per
    /// condition.
    pub fn condition(&self) -> Option<Vec<FamilyMemberHistory_Condition>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| FamilyMemberHistory_Condition {
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

    /// Describes why the family member's history is not available.
    pub fn data_absent_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("dataAbsentReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date (and possibly time) when the family member history was recorded or last
    /// updated.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub fn deceased_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("deceasedAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub fn deceased_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("deceasedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub fn deceased_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deceasedDate") {
            return Some(string);
        }
        return None;
    }

    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub fn deceased_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("deceasedRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Deceased flag or the actual or approximate age of the relative at the time of
    /// death for the family member history record.
    pub fn deceased_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deceasedString") {
            return Some(string);
        }
        return None;
    }

    /// If true, indicates that the age value specified is an estimated value.
    pub fn estimated_age(&self) -> Option<bool> {
        if let Some(val) = self.value.get("estimatedAge") {
            return Some(val.as_bool().unwrap());
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

    /// Business identifiers assigned to this family member history by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
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

    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this FamilyMemberHistory.
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
    /// other definition that is adhered to in whole or in part by this
    /// FamilyMemberHistory.
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

    /// This will either be a name or a description; e.g. "Aunt Susan", "my cousin with
    /// the red hair".
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// This property allows a non condition-specific note to the made about the related
    /// person. Ideally, the note would be in the condition property, but this is not
    /// always possible.
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

    /// The person who this history concerns.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// Describes why the family member history occurred in coded or textual form.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
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

    /// Indicates a Condition, Observation, AllergyIntolerance, or QuestionnaireResponse
    /// that justifies this family member history event.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
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

    /// The type of relationship this person has to the patient (father, mother, brother
    /// etc.).
    pub fn relationship(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["relationship"]),
        }
    }

    /// The birth sex of the family member.
    pub fn sex(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sex") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code specifying the status of the record of the family history of a specific
    /// family member.
    pub fn status(&self) -> Option<FamilyMemberHistoryStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(FamilyMemberHistoryStatus::from_string(&val).unwrap());
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
        if let Some(_val) = self._age_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._born_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._born_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._deceased_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._deceased_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._deceased_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._estimated_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.age_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.age_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.age_string() {}
        if let Some(_val) = self.born_date() {}
        if let Some(_val) = self.born_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.born_string() {}
        if let Some(_val) = self.condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.data_absent_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.deceased_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.deceased_boolean() {}
        if let Some(_val) = self.deceased_date() {}
        if let Some(_val) = self.deceased_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.deceased_string() {}
        if let Some(_val) = self.estimated_age() {}
        if let Some(_val) = self.extension() {
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
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.reason_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.relationship().validate() {
            return false;
        }
        if let Some(_val) = self.sex() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct FamilyMemberHistoryBuilder {
    pub(crate) value: Value,
}

impl FamilyMemberHistoryBuilder {
    pub fn build(&self) -> FamilyMemberHistory {
        FamilyMemberHistory {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: FamilyMemberHistory) -> FamilyMemberHistoryBuilder {
        FamilyMemberHistoryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(patient: Reference, relationship: CodeableConcept) -> FamilyMemberHistoryBuilder {
        let mut __value: Value = json!({});
        __value["patient"] = json!(patient.value);
        __value["relationship"] = json!(relationship.value);
        return FamilyMemberHistoryBuilder { value: __value };
    }

    pub fn _age_string<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_ageString"] = json!(val.value);
        return self;
    }

    pub fn _born_date<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_bornDate"] = json!(val.value);
        return self;
    }

    pub fn _born_string<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_bornString"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _deceased_boolean<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_deceasedBoolean"] = json!(val.value);
        return self;
    }

    pub fn _deceased_date<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_deceasedDate"] = json!(val.value);
        return self;
    }

    pub fn _deceased_string<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_deceasedString"] = json!(val.value);
        return self;
    }

    pub fn _estimated_age<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_estimatedAge"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instantiates_uri<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_instantiatesUri"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn age_age<'a>(&'a mut self, val: Age) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["ageAge"] = json!(val.value);
        return self;
    }

    pub fn age_range<'a>(&'a mut self, val: Range) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["ageRange"] = json!(val.value);
        return self;
    }

    pub fn age_string<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["ageString"] = json!(val);
        return self;
    }

    pub fn born_date<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["bornDate"] = json!(val);
        return self;
    }

    pub fn born_period<'a>(&'a mut self, val: Period) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["bornPeriod"] = json!(val.value);
        return self;
    }

    pub fn born_string<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["bornString"] = json!(val);
        return self;
    }

    pub fn condition<'a>(
        &'a mut self,
        val: Vec<FamilyMemberHistory_Condition>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["condition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn data_absent_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["dataAbsentReason"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn deceased_age<'a>(&'a mut self, val: Age) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["deceasedAge"] = json!(val.value);
        return self;
    }

    pub fn deceased_boolean<'a>(&'a mut self, val: bool) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["deceasedBoolean"] = json!(val);
        return self;
    }

    pub fn deceased_date<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["deceasedDate"] = json!(val);
        return self;
    }

    pub fn deceased_range<'a>(&'a mut self, val: Range) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["deceasedRange"] = json!(val.value);
        return self;
    }

    pub fn deceased_string<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["deceasedString"] = json!(val);
        return self;
    }

    pub fn estimated_age<'a>(&'a mut self, val: bool) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["estimatedAge"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn instantiates_canonical<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["instantiatesCanonical"] = json!(val);
        return self;
    }

    pub fn instantiates_uri<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["reasonCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_reference<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["reasonReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sex<'a>(&'a mut self, val: CodeableConcept) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["sex"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: FamilyMemberHistoryStatus,
    ) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut FamilyMemberHistoryBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum FamilyMemberHistoryStatus {
    Partial,
    Completed,
    EnteredInError,
    HealthUnknown,
}

impl FamilyMemberHistoryStatus {
    pub fn from_string(string: &str) -> Option<FamilyMemberHistoryStatus> {
        match string {
            "partial" => Some(FamilyMemberHistoryStatus::Partial),
            "completed" => Some(FamilyMemberHistoryStatus::Completed),
            "entered-in-error" => Some(FamilyMemberHistoryStatus::EnteredInError),
            "health-unknown" => Some(FamilyMemberHistoryStatus::HealthUnknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FamilyMemberHistoryStatus::Partial => "partial".to_string(),
            FamilyMemberHistoryStatus::Completed => "completed".to_string(),
            FamilyMemberHistoryStatus::EnteredInError => "entered-in-error".to_string(),
            FamilyMemberHistoryStatus::HealthUnknown => "health-unknown".to_string(),
        }
    }
}

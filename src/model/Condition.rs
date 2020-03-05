#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Condition_Evidence::Condition_Evidence;
use crate::model::Condition_Stage::Condition_Stage;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
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

/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.

#[derive(Debug)]
pub struct Condition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Condition<'_> {
    pub fn new(value: &Value) -> Condition {
        Condition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for abatementDateTime
    pub fn _abatement_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_abatementDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for abatementString
    pub fn _abatement_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_abatementString") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for onsetDateTime
    pub fn _onset_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for onsetString
    pub fn _onset_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recordedDate
    pub fn _recorded_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recordedDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Conditions are never really
    /// resolved, but they can abate.
    pub fn abatement_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("abatementAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Conditions are never really
    /// resolved, but they can abate.
    pub fn abatement_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("abatementDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Conditions are never really
    /// resolved, but they can abate.
    pub fn abatement_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("abatementPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Conditions are never really
    /// resolved, but they can abate.
    pub fn abatement_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("abatementRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Conditions are never really
    /// resolved, but they can abate.
    pub fn abatement_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("abatementString") {
            return Some(string);
        }
        return None;
    }

    /// Individual who is making the condition statement.
    pub fn asserter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("asserter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The anatomical location where this condition manifests itself.
    pub fn body_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodySite") {
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

    /// A category assigned to the condition.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
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

    /// The clinical status of the condition.
    pub fn clinical_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("clinicalStatus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identification of the condition, problem or diagnosis.
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

    /// The Encounter during which this Condition was created or to which the creation
    /// of this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Supporting evidence / manifestations that are the basis of the Condition's
    /// verification status, such as evidence that confirmed or refuted the condition.
    pub fn evidence(&self) -> Option<Vec<Condition_Evidence>> {
        if let Some(Value::Array(val)) = self.value.get("evidence") {
            return Some(
                val.into_iter()
                    .map(|e| Condition_Evidence {
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

    /// Business identifiers assigned to this condition by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
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

    /// Additional information about the Condition. This is a general notes/comments
    /// entry  for description of the Condition, its diagnosis and prognosis.
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

    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub fn onset_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("onsetAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub fn onset_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub fn onset_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("onsetPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub fn onset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("onsetRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub fn onset_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetString") {
            return Some(string);
        }
        return None;
    }

    /// The recordedDate represents when this particular Condition record was created in
    /// the system, which is often a system-generated date.
    pub fn recorded_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recordedDate") {
            return Some(string);
        }
        return None;
    }

    /// Individual who recorded the record and takes responsibility for its content.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A subjective assessment of the severity of the condition as evaluated by the
    /// clinician.
    pub fn severity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("severity") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Clinical stage or grade of a condition. May include formal severity assessments.
    pub fn stage(&self) -> Option<Vec<Condition_Stage>> {
        if let Some(Value::Array(val)) = self.value.get("stage") {
            return Some(
                val.into_iter()
                    .map(|e| Condition_Stage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the patient or group who the condition record is associated with.
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
        }
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

    /// The verification status to support the clinical status of the condition.
    pub fn verification_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("verificationStatus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._abatement_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._abatement_string() {
            if !_val.validate() {
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
        if let Some(_val) = self._onset_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._onset_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recorded_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.abatement_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.abatement_date_time() {}
        if let Some(_val) = self.abatement_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.abatement_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.abatement_string() {}
        if let Some(_val) = self.asserter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.clinical_status() {
            if !_val.validate() {
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
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.evidence() {
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
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.onset_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_date_time() {}
        if let Some(_val) = self.onset_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_string() {}
        if let Some(_val) = self.recorded_date() {}
        if let Some(_val) = self.recorder() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.stage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.subject().validate() {
            return false;
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.verification_status() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConditionBuilder {
    pub(crate) value: Value,
}

impl ConditionBuilder {
    pub fn build(&self) -> Condition {
        Condition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Condition) -> ConditionBuilder {
        ConditionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(subject: Reference) -> ConditionBuilder {
        let mut __value: Value = json!({});
        __value["subject"] = json!(subject.value);
        return ConditionBuilder { value: __value };
    }

    pub fn _abatement_date_time<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_abatementDateTime"] = json!(val.value);
        return self;
    }

    pub fn _abatement_string<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_abatementString"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _onset_date_time<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_onsetDateTime"] = json!(val.value);
        return self;
    }

    pub fn _onset_string<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_onsetString"] = json!(val.value);
        return self;
    }

    pub fn _recorded_date<'a>(&'a mut self, val: Element) -> &'a mut ConditionBuilder {
        self.value["_recordedDate"] = json!(val.value);
        return self;
    }

    pub fn abatement_age<'a>(&'a mut self, val: Age) -> &'a mut ConditionBuilder {
        self.value["abatementAge"] = json!(val.value);
        return self;
    }

    pub fn abatement_date_time<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["abatementDateTime"] = json!(val);
        return self;
    }

    pub fn abatement_period<'a>(&'a mut self, val: Period) -> &'a mut ConditionBuilder {
        self.value["abatementPeriod"] = json!(val.value);
        return self;
    }

    pub fn abatement_range<'a>(&'a mut self, val: Range) -> &'a mut ConditionBuilder {
        self.value["abatementRange"] = json!(val.value);
        return self;
    }

    pub fn abatement_string<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["abatementString"] = json!(val);
        return self;
    }

    pub fn asserter<'a>(&'a mut self, val: Reference) -> &'a mut ConditionBuilder {
        self.value["asserter"] = json!(val.value);
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ConditionBuilder {
        self.value["bodySite"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ConditionBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn clinical_status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionBuilder {
        self.value["clinicalStatus"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ConditionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut ConditionBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn evidence<'a>(&'a mut self, val: Vec<Condition_Evidence>) -> &'a mut ConditionBuilder {
        self.value["evidence"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConditionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ConditionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ConditionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConditionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ConditionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn onset_age<'a>(&'a mut self, val: Age) -> &'a mut ConditionBuilder {
        self.value["onsetAge"] = json!(val.value);
        return self;
    }

    pub fn onset_date_time<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["onsetDateTime"] = json!(val);
        return self;
    }

    pub fn onset_period<'a>(&'a mut self, val: Period) -> &'a mut ConditionBuilder {
        self.value["onsetPeriod"] = json!(val.value);
        return self;
    }

    pub fn onset_range<'a>(&'a mut self, val: Range) -> &'a mut ConditionBuilder {
        self.value["onsetRange"] = json!(val.value);
        return self;
    }

    pub fn onset_string<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["onsetString"] = json!(val);
        return self;
    }

    pub fn recorded_date<'a>(&'a mut self, val: &str) -> &'a mut ConditionBuilder {
        self.value["recordedDate"] = json!(val);
        return self;
    }

    pub fn recorder<'a>(&'a mut self, val: Reference) -> &'a mut ConditionBuilder {
        self.value["recorder"] = json!(val.value);
        return self;
    }

    pub fn severity<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionBuilder {
        self.value["severity"] = json!(val.value);
        return self;
    }

    pub fn stage<'a>(&'a mut self, val: Vec<Condition_Stage>) -> &'a mut ConditionBuilder {
        self.value["stage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ConditionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn verification_status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConditionBuilder {
        self.value["verificationStatus"] = json!(val.value);
        return self;
    }
}

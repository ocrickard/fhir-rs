#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::EpisodeOfCare_Diagnosis::EpisodeOfCare_Diagnosis;
use crate::model::EpisodeOfCare_StatusHistory::EpisodeOfCare_StatusHistory;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.

#[derive(Debug)]
pub struct EpisodeOfCare<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EpisodeOfCare<'_> {
    pub fn new(value: &Value) -> EpisodeOfCare {
        EpisodeOfCare {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The set of accounts that may be used for billing for this EpisodeOfCare.
    pub fn account(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("account") {
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

    /// The practitioner that is the care manager/care coordinator for this patient.
    pub fn care_manager(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("careManager") {
            return Some(Reference {
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

    /// The list of diagnosis relevant to this episode of care.
    pub fn diagnosis(&self) -> Option<Vec<EpisodeOfCare_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| EpisodeOfCare_Diagnosis {
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

    /// The EpisodeOfCare may be known by different identifiers for different contexts
    /// of use, such as when an external agency is tracking the Episode for funding
    /// purposes.
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

    /// The organization that has assumed the specific responsibilities for the
    /// specified duration.
    pub fn managing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("managingOrganization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// The patient who is the focus of this episode of care.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// The interval during which the managing organization assumes the defined
    /// responsibility.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Referral Request(s) that are fulfilled by this EpisodeOfCare, incoming
    /// referrals.
    pub fn referral_request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("referralRequest") {
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

    /// planned | waitlist | active | onhold | finished | cancelled.
    pub fn status(&self) -> Option<EpisodeOfCareStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(EpisodeOfCareStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The history of statuses that the EpisodeOfCare has been through (without
    /// requiring processing the history of the resource).
    pub fn status_history(&self) -> Option<Vec<EpisodeOfCare_StatusHistory>> {
        if let Some(Value::Array(val)) = self.value.get("statusHistory") {
            return Some(
                val.into_iter()
                    .map(|e| EpisodeOfCare_StatusHistory {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The list of practitioners that may be facilitating this episode of care for
    /// specific purposes.
    pub fn team(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("team") {
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

    /// A classification of the type of episode of care; e.g. specialist referral,
    /// disease management, type of funded care.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.account() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.care_manager() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.diagnosis() {
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
        if let Some(_val) = self.managing_organization() {
            if !_val.validate() {
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
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.referral_request() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.team() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EpisodeOfCareBuilder {
    pub(crate) value: Value,
}

impl EpisodeOfCareBuilder {
    pub fn build(&self) -> EpisodeOfCare {
        EpisodeOfCare {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: EpisodeOfCare) -> EpisodeOfCareBuilder {
        EpisodeOfCareBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(patient: Reference) -> EpisodeOfCareBuilder {
        let mut __value: Value = json!({});
        __value["patient"] = json!(patient.value);
        return EpisodeOfCareBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut EpisodeOfCareBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut EpisodeOfCareBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut EpisodeOfCareBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn account<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EpisodeOfCareBuilder {
        self.value["account"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn care_manager<'a>(&'a mut self, val: Reference) -> &'a mut EpisodeOfCareBuilder {
        self.value["careManager"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut EpisodeOfCareBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn diagnosis<'a>(
        &'a mut self,
        val: Vec<EpisodeOfCare_Diagnosis>,
    ) -> &'a mut EpisodeOfCareBuilder {
        self.value["diagnosis"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EpisodeOfCareBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EpisodeOfCareBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut EpisodeOfCareBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut EpisodeOfCareBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut EpisodeOfCareBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn managing_organization<'a>(&'a mut self, val: Reference) -> &'a mut EpisodeOfCareBuilder {
        self.value["managingOrganization"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut EpisodeOfCareBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EpisodeOfCareBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut EpisodeOfCareBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn referral_request<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EpisodeOfCareBuilder {
        self.value["referralRequest"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: EpisodeOfCareStatus) -> &'a mut EpisodeOfCareBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn status_history<'a>(
        &'a mut self,
        val: Vec<EpisodeOfCare_StatusHistory>,
    ) -> &'a mut EpisodeOfCareBuilder {
        self.value["statusHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn team<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut EpisodeOfCareBuilder {
        self.value["team"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut EpisodeOfCareBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut EpisodeOfCareBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum EpisodeOfCareStatus {
    Planned,
    Waitlist,
    Active,
    Onhold,
    Finished,
    Cancelled,
    EnteredInError,
}

impl EpisodeOfCareStatus {
    pub fn from_string(string: &str) -> Option<EpisodeOfCareStatus> {
        match string {
            "planned" => Some(EpisodeOfCareStatus::Planned),
            "waitlist" => Some(EpisodeOfCareStatus::Waitlist),
            "active" => Some(EpisodeOfCareStatus::Active),
            "onhold" => Some(EpisodeOfCareStatus::Onhold),
            "finished" => Some(EpisodeOfCareStatus::Finished),
            "cancelled" => Some(EpisodeOfCareStatus::Cancelled),
            "entered-in-error" => Some(EpisodeOfCareStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EpisodeOfCareStatus::Planned => "planned".to_string(),
            EpisodeOfCareStatus::Waitlist => "waitlist".to_string(),
            EpisodeOfCareStatus::Active => "active".to_string(),
            EpisodeOfCareStatus::Onhold => "onhold".to_string(),
            EpisodeOfCareStatus::Finished => "finished".to_string(),
            EpisodeOfCareStatus::Cancelled => "cancelled".to_string(),
            EpisodeOfCareStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}

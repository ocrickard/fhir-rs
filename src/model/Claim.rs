#![allow(unused_imports, non_camel_case_types)]

use crate::model::Claim_Accident::Claim_Accident;
use crate::model::Claim_CareTeam::Claim_CareTeam;
use crate::model::Claim_Diagnosis::Claim_Diagnosis;
use crate::model::Claim_Insurance::Claim_Insurance;
use crate::model::Claim_Item::Claim_Item;
use crate::model::Claim_Payee::Claim_Payee;
use crate::model::Claim_Procedure::Claim_Procedure;
use crate::model::Claim_Related::Claim_Related;
use crate::model::Claim_SupportingInfo::Claim_SupportingInfo;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Claim<'_> {
    pub fn new(value: &Value) -> Claim {
        Claim {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Details of an accident which resulted in injuries which required the products
    /// and services listed in the claim.
    pub fn accident(&self) -> Option<Claim_Accident> {
        if let Some(val) = self.value.get("accident") {
            return Some(Claim_Accident {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period for which charges are being submitted.
    pub fn billable_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("billablePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The members of the team who provided the products and services.
    pub fn care_team(&self) -> Option<Vec<Claim_CareTeam>> {
        if let Some(Value::Array(val)) = self.value.get("careTeam") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_CareTeam {
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

    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// Information about diagnoses relevant to the claim items.
    pub fn diagnosis(&self) -> Option<Vec<Claim_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Diagnosis {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Individual who created the claim, predetermination or preauthorization.
    pub fn enterer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("enterer") {
            return Some(Reference {
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

    /// Facility where the services were provided.
    pub fn facility(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("facility") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code to indicate whether and for whom funds are to be reserved for future
    /// claims.
    pub fn funds_reserve(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserve") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A unique identifier assigned to this claim.
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

    /// Financial instruments for reimbursement for the health care products and
    /// services specified on the claim.
    pub fn insurance(&self) -> Vec<Claim_Insurance> {
        self.value
            .get("insurance")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Claim_Insurance {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// The Insurer who is target of the request.
    pub fn insurer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("insurer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A claim line. Either a simple  product or service or a 'group' of details which
    /// can each be a simple items or groups of sub-details.
    pub fn item(&self) -> Option<Vec<Claim_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Item {
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

    /// Original prescription which has been superseded by this prescription to support
    /// the dispensing of pharmacy services, medications or products.
    pub fn original_prescription(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("originalPrescription") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual or forecast reimbursement is sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// The party to be reimbursed for cost of the products and services according to
    /// the terms of the policy.
    pub fn payee(&self) -> Option<Claim_Payee> {
        if let Some(val) = self.value.get("payee") {
            return Some(Claim_Payee {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Prescription to support the dispensing of pharmacy, device or vision products.
    pub fn prescription(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("prescription") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The provider-required urgency of processing the request. Typical values include:
    /// stat, routine deferred.
    pub fn priority(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["priority"]),
        }
    }

    /// Procedures performed on the patient relevant to the billing items with the
    /// claim.
    pub fn procedure(&self) -> Option<Vec<Claim_Procedure>> {
        if let Some(Value::Array(val)) = self.value.get("procedure") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Procedure {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub fn provider(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["provider"]),
        }
    }

    /// A reference to a referral resource.
    pub fn referral(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("referral") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Other claims which are related to this claim such as prior submissions or claims
    /// for related services or for the same event.
    pub fn related(&self) -> Option<Vec<Claim_Related>> {
        if let Some(Value::Array(val)) = self.value.get("related") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Related {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of the resource instance.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information codes regarding exceptions, special considerations, the
    /// condition, situation, prior or concurrent issues.
    pub fn supporting_info(&self) -> Option<Vec<Claim_SupportingInfo>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_SupportingInfo {
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

    /// The total value of the all the items in the claim.
    pub fn total(&self) -> Option<Money> {
        if let Some(val) = self.value.get("total") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// A code to indicate whether the nature of the request is: to request adjudication
    /// of products and services previously rendered; or requesting authorization and
    /// adjudication for provision in the future; or requesting the non-binding
    /// adjudication of the listed products and services which could be provided in the
    /// future.
    pub fn fhir_use(&self) -> Option<ClaimUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(ClaimUse::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._created() {
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.accident() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.billable_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.care_team() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.diagnosis() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.enterer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.facility() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.funds_reserve() {
            if !_val.validate() {
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
        if !self
            .insurance()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.insurer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
        if let Some(_val) = self.original_prescription() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.payee() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.prescription() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.priority().validate() {
            return false;
        }
        if let Some(_val) = self.procedure() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.provider().validate() {
            return false;
        }
        if let Some(_val) = self.referral() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.related() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.sub_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.supporting_info() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.total() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimBuilder {
    pub(crate) value: Value,
}

impl ClaimBuilder {
    pub fn build(&self) -> Claim {
        Claim {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Claim) -> ClaimBuilder {
        ClaimBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        insurance: Vec<Claim_Insurance>,
        patient: Reference,
        priority: CodeableConcept,
        provider: Reference,
        fhir_type: CodeableConcept,
    ) -> ClaimBuilder {
        let mut __value: Value = json!({});
        __value["insurance"] = json!(insurance.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["patient"] = json!(patient.value);
        __value["priority"] = json!(priority.value);
        __value["provider"] = json!(provider.value);
        __value["type"] = json!(fhir_type.value);
        return ClaimBuilder { value: __value };
    }

    pub fn _created<'a>(&'a mut self, val: Element) -> &'a mut ClaimBuilder {
        self.value["_created"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ClaimBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ClaimBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ClaimBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut ClaimBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn accident<'a>(&'a mut self, val: Claim_Accident) -> &'a mut ClaimBuilder {
        self.value["accident"] = json!(val.value);
        return self;
    }

    pub fn billable_period<'a>(&'a mut self, val: Period) -> &'a mut ClaimBuilder {
        self.value["billablePeriod"] = json!(val.value);
        return self;
    }

    pub fn care_team<'a>(&'a mut self, val: Vec<Claim_CareTeam>) -> &'a mut ClaimBuilder {
        self.value["careTeam"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ClaimBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created<'a>(&'a mut self, val: &str) -> &'a mut ClaimBuilder {
        self.value["created"] = json!(val);
        return self;
    }

    pub fn diagnosis<'a>(&'a mut self, val: Vec<Claim_Diagnosis>) -> &'a mut ClaimBuilder {
        self.value["diagnosis"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn enterer<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["enterer"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ClaimBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn facility<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["facility"] = json!(val.value);
        return self;
    }

    pub fn funds_reserve<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimBuilder {
        self.value["fundsReserve"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ClaimBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ClaimBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn insurer<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["insurer"] = json!(val.value);
        return self;
    }

    pub fn item<'a>(&'a mut self, val: Vec<Claim_Item>) -> &'a mut ClaimBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ClaimBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ClaimBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ClaimBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn original_prescription<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["originalPrescription"] = json!(val.value);
        return self;
    }

    pub fn payee<'a>(&'a mut self, val: Claim_Payee) -> &'a mut ClaimBuilder {
        self.value["payee"] = json!(val.value);
        return self;
    }

    pub fn prescription<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["prescription"] = json!(val.value);
        return self;
    }

    pub fn procedure<'a>(&'a mut self, val: Vec<Claim_Procedure>) -> &'a mut ClaimBuilder {
        self.value["procedure"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn referral<'a>(&'a mut self, val: Reference) -> &'a mut ClaimBuilder {
        self.value["referral"] = json!(val.value);
        return self;
    }

    pub fn related<'a>(&'a mut self, val: Vec<Claim_Related>) -> &'a mut ClaimBuilder {
        self.value["related"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ClaimBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn sub_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimBuilder {
        self.value["subType"] = json!(val.value);
        return self;
    }

    pub fn supporting_info<'a>(
        &'a mut self,
        val: Vec<Claim_SupportingInfo>,
    ) -> &'a mut ClaimBuilder {
        self.value["supportingInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ClaimBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn total<'a>(&'a mut self, val: Money) -> &'a mut ClaimBuilder {
        self.value["total"] = json!(val.value);
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: ClaimUse) -> &'a mut ClaimBuilder {
        self.value["use"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ClaimUse {
    Claim,
    Preauthorization,
    Predetermination,
}

impl ClaimUse {
    pub fn from_string(string: &str) -> Option<ClaimUse> {
        match string {
            "claim" => Some(ClaimUse::Claim),
            "preauthorization" => Some(ClaimUse::Preauthorization),
            "predetermination" => Some(ClaimUse::Predetermination),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ClaimUse::Claim => "claim".to_string(),
            ClaimUse::Preauthorization => "preauthorization".to_string(),
            ClaimUse::Predetermination => "predetermination".to_string(),
        }
    }
}

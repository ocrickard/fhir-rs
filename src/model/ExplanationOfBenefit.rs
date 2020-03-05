#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::ExplanationOfBenefit_Accident::ExplanationOfBenefit_Accident;
use crate::model::ExplanationOfBenefit_AddItem::ExplanationOfBenefit_AddItem;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::ExplanationOfBenefit_BenefitBalance::ExplanationOfBenefit_BenefitBalance;
use crate::model::ExplanationOfBenefit_CareTeam::ExplanationOfBenefit_CareTeam;
use crate::model::ExplanationOfBenefit_Diagnosis::ExplanationOfBenefit_Diagnosis;
use crate::model::ExplanationOfBenefit_Insurance::ExplanationOfBenefit_Insurance;
use crate::model::ExplanationOfBenefit_Item::ExplanationOfBenefit_Item;
use crate::model::ExplanationOfBenefit_Payee::ExplanationOfBenefit_Payee;
use crate::model::ExplanationOfBenefit_Payment::ExplanationOfBenefit_Payment;
use crate::model::ExplanationOfBenefit_Procedure::ExplanationOfBenefit_Procedure;
use crate::model::ExplanationOfBenefit_ProcessNote::ExplanationOfBenefit_ProcessNote;
use crate::model::ExplanationOfBenefit_Related::ExplanationOfBenefit_Related;
use crate::model::ExplanationOfBenefit_SupportingInfo::ExplanationOfBenefit_SupportingInfo;
use crate::model::ExplanationOfBenefit_Total::ExplanationOfBenefit_Total;
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

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit<'_> {
    pub fn new(value: &Value) -> ExplanationOfBenefit {
        ExplanationOfBenefit {
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

    /// Extensions for disposition
    pub fn _disposition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disposition") {
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

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_preAuthRef") {
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

    /// Extensions for precedence
    pub fn _precedence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precedence") {
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

    /// Details of a accident which resulted in injuries which required the products and
    /// services listed in the claim.
    pub fn accident(&self) -> Option<ExplanationOfBenefit_Accident> {
        if let Some(val) = self.value.get("accident") {
            return Some(ExplanationOfBenefit_Accident {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The first-tier service adjudications for payor added product or service lines.
    pub fn add_item(&self) -> Option<Vec<ExplanationOfBenefit_AddItem>> {
        if let Some(Value::Array(val)) = self.value.get("addItem") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_AddItem {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The adjudication results which are presented at the header level rather than at
    /// the line-item or add-item levels.
    pub fn adjudication(&self) -> Option<Vec<ExplanationOfBenefit_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Adjudication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Balance by Benefit Category.
    pub fn benefit_balance(&self) -> Option<Vec<ExplanationOfBenefit_BenefitBalance>> {
        if let Some(Value::Array(val)) = self.value.get("benefitBalance") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_BenefitBalance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The term of the benefits documented in this response.
    pub fn benefit_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("benefitPeriod") {
            return Some(Period {
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
    pub fn care_team(&self) -> Option<Vec<ExplanationOfBenefit_CareTeam>> {
        if let Some(Value::Array(val)) = self.value.get("careTeam") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_CareTeam {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The business identifier for the instance of the adjudication request: claim
    /// predetermination or preauthorization.
    pub fn claim(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claim") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The business identifier for the instance of the adjudication response: claim,
    /// predetermination or preauthorization response.
    pub fn claim_response(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claimResponse") {
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

    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// Information about diagnoses relevant to the claim items.
    pub fn diagnosis(&self) -> Option<Vec<ExplanationOfBenefit_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Diagnosis {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human readable description of the status of the adjudication.
    pub fn disposition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disposition") {
            return Some(string);
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

    /// The actual form, by reference or inclusion, for printing the content or an EOB.
    pub fn form(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("form") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code for the form to be used for printing the content.
    pub fn form_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("formCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub fn funds_reserve(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserve") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code to indicate whether and for whom funds are to be reserved for future
    /// claims.
    pub fn funds_reserve_requested(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserveRequested") {
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

    /// A unique identifier assigned to this explanation of benefit.
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
    pub fn insurance(&self) -> Vec<ExplanationOfBenefit_Insurance> {
        self.value
            .get("insurance")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ExplanationOfBenefit_Insurance {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// The party responsible for authorization, adjudication and reimbursement.
    pub fn insurer(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["insurer"]),
        }
    }

    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub fn item(&self) -> Option<Vec<ExplanationOfBenefit_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Item {
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

    /// The outcome of the claim, predetermination, or preauthorization processing.
    pub fn outcome(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outcome") {
            return Some(string);
        }
        return None;
    }

    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for forecast reimbursement is
    /// sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// The party to be reimbursed for cost of the products and services according to
    /// the terms of the policy.
    pub fn payee(&self) -> Option<ExplanationOfBenefit_Payee> {
        if let Some(val) = self.value.get("payee") {
            return Some(ExplanationOfBenefit_Payee {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Payment details for the adjudication of the claim.
    pub fn payment(&self) -> Option<ExplanationOfBenefit_Payment> {
        if let Some(val) = self.value.get("payment") {
            return Some(ExplanationOfBenefit_Payment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub fn pre_auth_ref(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("preAuthRef") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The timeframe during which the supplied preauthorization reference may be quoted
    /// on claims to obtain the adjudication as provided.
    pub fn pre_auth_ref_period(&self) -> Option<Vec<Period>> {
        if let Some(Value::Array(val)) = self.value.get("preAuthRefPeriod") {
            return Some(
                val.into_iter()
                    .map(|e| Period {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This indicates the relative order of a series of EOBs related to different
    /// coverages for the same suite of services.
    pub fn precedence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("precedence") {
            return Some(val.as_i64().unwrap());
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
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Procedures performed on the patient relevant to the billing items with the
    /// claim.
    pub fn procedure(&self) -> Option<Vec<ExplanationOfBenefit_Procedure>> {
        if let Some(Value::Array(val)) = self.value.get("procedure") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Procedure {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A note that describes or explains adjudication results in a human readable form.
    pub fn process_note(&self) -> Option<Vec<ExplanationOfBenefit_ProcessNote>> {
        if let Some(Value::Array(val)) = self.value.get("processNote") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_ProcessNote {
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
    pub fn related(&self) -> Option<Vec<ExplanationOfBenefit_Related>> {
        if let Some(Value::Array(val)) = self.value.get("related") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Related {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of the resource instance.
    pub fn status(&self) -> Option<ExplanationOfBenefitStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ExplanationOfBenefitStatus::from_string(&val).unwrap());
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
    pub fn supporting_info(&self) -> Option<Vec<ExplanationOfBenefit_SupportingInfo>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_SupportingInfo {
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

    /// Categorized monetary totals for the adjudication.
    pub fn total(&self) -> Option<Vec<ExplanationOfBenefit_Total>> {
        if let Some(Value::Array(val)) = self.value.get("total") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Total {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
    pub fn fhir_use(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("use") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._created() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._disposition() {
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
        if let Some(_val) = self._outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._pre_auth_ref() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._precedence() {
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
        if let Some(_val) = self.add_item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.adjudication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.benefit_balance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.benefit_period() {
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
        if let Some(_val) = self.claim() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.claim_response() {
            if !_val.validate() {
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
        if let Some(_val) = self.disposition() {}
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
        if let Some(_val) = self.form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.form_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.funds_reserve() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.funds_reserve_requested() {
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
        if !self.insurer().validate() {
            return false;
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
        if let Some(_val) = self.outcome() {}
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.payee() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.payment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.pre_auth_ref() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.pre_auth_ref_period() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precedence() {}
        if let Some(_val) = self.prescription() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.procedure() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.process_note() {
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
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct ExplanationOfBenefitBuilder {
    pub(crate) value: Value,
}

impl ExplanationOfBenefitBuilder {
    pub fn build(&self) -> ExplanationOfBenefit {
        ExplanationOfBenefit {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExplanationOfBenefit) -> ExplanationOfBenefitBuilder {
        ExplanationOfBenefitBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        insurance: Vec<ExplanationOfBenefit_Insurance>,
        insurer: Reference,
        patient: Reference,
        provider: Reference,
        fhir_type: CodeableConcept,
    ) -> ExplanationOfBenefitBuilder {
        let mut __value: Value = json!({});
        __value["insurance"] = json!(insurance.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["insurer"] = json!(insurer.value);
        __value["patient"] = json!(patient.value);
        __value["provider"] = json!(provider.value);
        __value["type"] = json!(fhir_type.value);
        return ExplanationOfBenefitBuilder { value: __value };
    }

    pub fn _created<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_created"] = json!(val.value);
        return self;
    }

    pub fn _disposition<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_disposition"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _outcome<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_outcome"] = json!(val.value);
        return self;
    }

    pub fn _pre_auth_ref<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_preAuthRef"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _precedence<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_precedence"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn accident<'a>(
        &'a mut self,
        val: ExplanationOfBenefit_Accident,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["accident"] = json!(val.value);
        return self;
    }

    pub fn add_item<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_AddItem>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["addItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn adjudication<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Adjudication>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["adjudication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn benefit_balance<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_BenefitBalance>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["benefitBalance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn benefit_period<'a>(&'a mut self, val: Period) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["benefitPeriod"] = json!(val.value);
        return self;
    }

    pub fn billable_period<'a>(&'a mut self, val: Period) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["billablePeriod"] = json!(val.value);
        return self;
    }

    pub fn care_team<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_CareTeam>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["careTeam"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn claim<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["claim"] = json!(val.value);
        return self;
    }

    pub fn claim_response<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["claimResponse"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["created"] = json!(val);
        return self;
    }

    pub fn diagnosis<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Diagnosis>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["diagnosis"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn disposition<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["disposition"] = json!(val);
        return self;
    }

    pub fn enterer<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["enterer"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn facility<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["facility"] = json!(val.value);
        return self;
    }

    pub fn form<'a>(&'a mut self, val: Attachment) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["form"] = json!(val.value);
        return self;
    }

    pub fn form_code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["formCode"] = json!(val.value);
        return self;
    }

    pub fn funds_reserve<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["fundsReserve"] = json!(val.value);
        return self;
    }

    pub fn funds_reserve_requested<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["fundsReserveRequested"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn item<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Item>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn original_prescription<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["originalPrescription"] = json!(val.value);
        return self;
    }

    pub fn outcome<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["outcome"] = json!(val);
        return self;
    }

    pub fn payee<'a>(
        &'a mut self,
        val: ExplanationOfBenefit_Payee,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["payee"] = json!(val.value);
        return self;
    }

    pub fn payment<'a>(
        &'a mut self,
        val: ExplanationOfBenefit_Payment,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["payment"] = json!(val.value);
        return self;
    }

    pub fn pre_auth_ref<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["preAuthRef"] = json!(val);
        return self;
    }

    pub fn pre_auth_ref_period<'a>(
        &'a mut self,
        val: Vec<Period>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["preAuthRefPeriod"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precedence<'a>(&'a mut self, val: i64) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["precedence"] = json!(val);
        return self;
    }

    pub fn prescription<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["prescription"] = json!(val.value);
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["priority"] = json!(val.value);
        return self;
    }

    pub fn procedure<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Procedure>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["procedure"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn process_note<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_ProcessNote>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["processNote"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn referral<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["referral"] = json!(val.value);
        return self;
    }

    pub fn related<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Related>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["related"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: ExplanationOfBenefitStatus,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn sub_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["subType"] = json!(val.value);
        return self;
    }

    pub fn supporting_info<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_SupportingInfo>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["supportingInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn total<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Total>,
    ) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["total"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefitBuilder {
        self.value["use"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ExplanationOfBenefitStatus {
    Active,
    Cancelled,
    Draft,
    EnteredInError,
}

impl ExplanationOfBenefitStatus {
    pub fn from_string(string: &str) -> Option<ExplanationOfBenefitStatus> {
        match string {
            "active" => Some(ExplanationOfBenefitStatus::Active),
            "cancelled" => Some(ExplanationOfBenefitStatus::Cancelled),
            "draft" => Some(ExplanationOfBenefitStatus::Draft),
            "entered-in-error" => Some(ExplanationOfBenefitStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ExplanationOfBenefitStatus::Active => "active".to_string(),
            ExplanationOfBenefitStatus::Cancelled => "cancelled".to_string(),
            ExplanationOfBenefitStatus::Draft => "draft".to_string(),
            ExplanationOfBenefitStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}

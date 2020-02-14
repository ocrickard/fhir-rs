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
use serde_json::value::Value;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit<'a> {
    pub value: &'a Value,
}

impl ExplanationOfBenefit<'_> {
    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
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

    /// Prescription to support the dispensing of pharmacy, device or vision products.
    pub fn prescription(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("prescription") {
            return Some(Reference { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The timeframe during which the supplied preauthorization reference may be quoted
    /// on claims to obtain the adjudication as provided.
    pub fn pre_auth_ref_period(&self) -> Option<Vec<Period>> {
        if let Some(Value::Array(val)) = self.value.get("preAuthRefPeriod") {
            return Some(
                val.into_iter()
                    .map(|e| Period { value: e })
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
                    .map(|e| ExplanationOfBenefit_Adjudication { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Original prescription which has been superseded by this prescription to support
    /// the dispensing of pharmacy services, medications or products.
    pub fn original_prescription(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("originalPrescription") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Information about diagnoses relevant to the claim items.
    pub fn diagnosis(&self) -> Option<Vec<ExplanationOfBenefit_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Diagnosis { value: e })
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

    /// The actual form, by reference or inclusion, for printing the content or an EOB.
    pub fn form(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("form") {
            return Some(Attachment { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code to indicate whether and for whom funds are to be reserved for future
    /// claims.
    pub fn funds_reserve_requested(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserveRequested") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Other claims which are related to this claim such as prior submissions or claims
    /// for related services or for the same event.
    pub fn related(&self) -> Option<Vec<ExplanationOfBenefit_Related>> {
        if let Some(Value::Array(val)) = self.value.get("related") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Related { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The business identifier for the instance of the adjudication request: claim
    /// predetermination or preauthorization.
    pub fn claim(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claim") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A reference to a referral resource.
    pub fn referral(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("referral") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Procedures performed on the patient relevant to the billing items with the
    /// claim.
    pub fn procedure(&self) -> Option<Vec<ExplanationOfBenefit_Procedure>> {
        if let Some(Value::Array(val)) = self.value.get("procedure") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Procedure { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for precedence
    pub fn _precedence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precedence") {
            return Some(Element { value: val });
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

    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub fn item(&self) -> Option<Vec<ExplanationOfBenefit_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Item { value: e })
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
                    .map(|e| ExplanationOfBenefit_BenefitBalance { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The first-tier service adjudications for payor added product or service lines.
    pub fn add_item(&self) -> Option<Vec<ExplanationOfBenefit_AddItem>> {
        if let Some(Value::Array(val)) = self.value.get("addItem") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_AddItem { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_preAuthRef") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for disposition
    pub fn _disposition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disposition") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Facility where the services were provided.
    pub fn facility(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("facility") {
            return Some(Reference { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub fn provider(&self) -> Reference {
        Reference {
            value: &self.value["provider"],
        }
    }

    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for forecast reimbursement is
    /// sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: &self.value["patient"],
        }
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Additional information codes regarding exceptions, special considerations, the
    /// condition, situation, prior or concurrent issues.
    pub fn supporting_info(&self) -> Option<Vec<ExplanationOfBenefit_SupportingInfo>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_SupportingInfo { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
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

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// The provider-required urgency of processing the request. Typical values include:
    /// stat, routine deferred.
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A note that describes or explains adjudication results in a human readable form.
    pub fn process_note(&self) -> Option<Vec<ExplanationOfBenefit_ProcessNote>> {
        if let Some(Value::Array(val)) = self.value.get("processNote") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_ProcessNote { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The period for which charges are being submitted.
    pub fn billable_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("billablePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The term of the benefits documented in this response.
    pub fn benefit_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("benefitPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The party responsible for authorization, adjudication and reimbursement.
    pub fn insurer(&self) -> Reference {
        Reference {
            value: &self.value["insurer"],
        }
    }

    /// The party to be reimbursed for cost of the products and services according to
    /// the terms of the policy.
    pub fn payee(&self) -> Option<ExplanationOfBenefit_Payee> {
        if let Some(val) = self.value.get("payee") {
            return Some(ExplanationOfBenefit_Payee { value: val });
        }
        return None;
    }

    /// The business identifier for the instance of the adjudication response: claim,
    /// predetermination or preauthorization response.
    pub fn claim_response(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claimResponse") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The members of the team who provided the products and services.
    pub fn care_team(&self) -> Option<Vec<ExplanationOfBenefit_CareTeam>> {
        if let Some(Value::Array(val)) = self.value.get("careTeam") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_CareTeam { value: e })
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

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
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
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Individual who created the claim, predetermination or preauthorization.
    pub fn enterer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("enterer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub fn funds_reserve(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserve") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// A unique identifier assigned to this explanation of benefit.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    /// Categorized monetary totals for the adjudication.
    pub fn total(&self) -> Option<Vec<ExplanationOfBenefit_Total>> {
        if let Some(Value::Array(val)) = self.value.get("total") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Total { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Payment details for the adjudication of the claim.
    pub fn payment(&self) -> Option<ExplanationOfBenefit_Payment> {
        if let Some(val) = self.value.get("payment") {
            return Some(ExplanationOfBenefit_Payment { value: val });
        }
        return None;
    }

    /// Details of a accident which resulted in injuries which required the products and
    /// services listed in the claim.
    pub fn accident(&self) -> Option<ExplanationOfBenefit_Accident> {
        if let Some(val) = self.value.get("accident") {
            return Some(ExplanationOfBenefit_Accident { value: val });
        }
        return None;
    }

    /// A code for the form to be used for printing the content.
    pub fn form_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("formCode") {
            return Some(CodeableConcept { value: val });
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
            .map(|e| ExplanationOfBenefit_Insurance { value: e })
            .collect::<Vec<_>>()
    }

    /// The outcome of the claim, predetermination, or preauthorization processing.
    pub fn outcome(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outcome") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.prescription() {
            _val.validate();
        }
        if let Some(_val) = self.disposition() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.pre_auth_ref_period() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.adjudication() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.original_prescription() {
            _val.validate();
        }
        if let Some(_val) = self.diagnosis() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.precedence() {}
        if let Some(_val) = self.form() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.funds_reserve_requested() {
            _val.validate();
        }
        if let Some(_val) = self.related() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.claim() {
            _val.validate();
        }
        if let Some(_val) = self.referral() {
            _val.validate();
        }
        if let Some(_val) = self.procedure() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._precedence() {
            _val.validate();
        }
        if let Some(_val) = self.pre_auth_ref() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.benefit_balance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.add_item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._pre_auth_ref() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._disposition() {
            _val.validate();
        }
        if let Some(_val) = self.facility() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.provider().validate();
        let _ = self.patient().validate();
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._created() {
            _val.validate();
        }
        if let Some(_val) = self.supporting_info() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_use() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.priority() {
            _val.validate();
        }
        if let Some(_val) = self.process_note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._use() {
            _val.validate();
        }
        if let Some(_val) = self.billable_period() {
            _val.validate();
        }
        if let Some(_val) = self.benefit_period() {
            _val.validate();
        }
        let _ = self.insurer().validate();
        if let Some(_val) = self.payee() {
            _val.validate();
        }
        if let Some(_val) = self.claim_response() {
            _val.validate();
        }
        if let Some(_val) = self._outcome() {
            _val.validate();
        }
        if let Some(_val) = self.care_team() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.sub_type() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.enterer() {
            _val.validate();
        }
        if let Some(_val) = self.funds_reserve() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self.total() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.payment() {
            _val.validate();
        }
        if let Some(_val) = self.accident() {
            _val.validate();
        }
        if let Some(_val) = self.form_code() {
            _val.validate();
        }
        let _ = self.insurance().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.outcome() {}
        if let Some(_val) = self.status() {}
        return true;
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
}

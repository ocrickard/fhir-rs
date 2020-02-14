#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::ClaimResponse_AddItem::ClaimResponse_AddItem;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::ClaimResponse_Error::ClaimResponse_Error;
use crate::model::ClaimResponse_Insurance::ClaimResponse_Insurance;
use crate::model::ClaimResponse_Item::ClaimResponse_Item;
use crate::model::ClaimResponse_Payment::ClaimResponse_Payment;
use crate::model::ClaimResponse_ProcessNote::ClaimResponse_ProcessNote;
use crate::model::ClaimResponse_Total::ClaimResponse_Total;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse<'a> {
    pub value: &'a Value,
}

impl ClaimResponse<'_> {
    /// Financial instruments for reimbursement for the health care products and
    /// services specified on the claim.
    pub fn insurance(&self) -> Option<Vec<ClaimResponse_Insurance>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Insurance { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Errors encountered during the processing of the adjudication.
    pub fn error(&self) -> Option<Vec<ClaimResponse_Error>> {
        if let Some(Value::Array(val)) = self.value.get("error") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Error { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A unique identifier assigned to this claim response.
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

    /// A human readable description of the status of the adjudication.
    pub fn disposition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disposition") {
            return Some(string);
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

    /// Type of Party to be reimbursed: subscriber, provider, other.
    pub fn payee_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("payeeType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A note that describes or explains adjudication results in a human readable form.
    pub fn process_note(&self) -> Option<Vec<ClaimResponse_ProcessNote>> {
        if let Some(Value::Array(val)) = self.value.get("processNote") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_ProcessNote { value: e })
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

    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
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

    /// The party responsible for authorization, adjudication and reimbursement.
    pub fn insurer(&self) -> Reference {
        Reference {
            value: &self.value["insurer"],
        }
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

    /// Request for additional supporting or authorizing information.
    pub fn communication_request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("communicationRequest") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub fn funds_reserve(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserve") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for facast reimbursement is sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: &self.value["patient"],
        }
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

    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub fn pre_auth_ref(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preAuthRef") {
            return Some(string);
        }
        return None;
    }

    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub fn item(&self) -> Option<Vec<ClaimResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Item { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Categorized monetary totals for the adjudication.
    pub fn total(&self) -> Option<Vec<ClaimResponse_Total>> {
        if let Some(Value::Array(val)) = self.value.get("total") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Total { value: e })
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

    /// A code for the form to be used for printing the content.
    pub fn form_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("formCode") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The first-tier service adjudications for payor added product or service lines.
    pub fn add_item(&self) -> Option<Vec<ClaimResponse_AddItem>> {
        if let Some(Value::Array(val)) = self.value.get("addItem") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_AddItem { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preAuthRef") {
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

    /// Extensions for disposition
    pub fn _disposition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disposition") {
            return Some(Element { value: val });
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Original request resource reference.
    pub fn request(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("request") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The time frame during which this authorization is effective.
    pub fn pre_auth_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("preAuthPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub fn requestor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestor") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The adjudication results which are presented at the header level rather than at
    /// the line-item or add-item levels.
    pub fn adjudication(&self) -> Option<Vec<ClaimResponse_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Adjudication { value: e })
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

    /// Payment details for the adjudication of the claim.
    pub fn payment(&self) -> Option<ClaimResponse_Payment> {
        if let Some(val) = self.value.get("payment") {
            return Some(ClaimResponse_Payment { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.insurance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.error() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.disposition() {}
        if let Some(_val) = self._use() {
            _val.validate();
        }
        if let Some(_val) = self.payee_type() {
            _val.validate();
        }
        if let Some(_val) = self.process_note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.created() {}
        if let Some(_val) = self._created() {
            _val.validate();
        }
        let _ = self.insurer().validate();
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.communication_request() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.funds_reserve() {
            _val.validate();
        }
        let _ = self.patient().validate();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.pre_auth_ref() {}
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.total() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.form_code() {
            _val.validate();
        }
        if let Some(_val) = self.add_item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.form() {
            _val.validate();
        }
        if let Some(_val) = self.sub_type() {
            _val.validate();
        }
        if let Some(_val) = self._outcome() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.outcome() {}
        if let Some(_val) = self._pre_auth_ref() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._disposition() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.request() {
            _val.validate();
        }
        if let Some(_val) = self.pre_auth_period() {
            _val.validate();
        }
        if let Some(_val) = self.requestor() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.adjudication() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_use() {}
        if let Some(_val) = self.payment() {
            _val.validate();
        }
        return true;
    }
}

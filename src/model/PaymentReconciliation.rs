#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::Narrative::Narrative;
use crate::model::PaymentReconciliation_Detail::PaymentReconciliation_Detail;
use crate::model::PaymentReconciliation_ProcessNote::PaymentReconciliation_ProcessNote;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.

#[derive(Debug)]
pub struct PaymentReconciliation<'a> {
    pub value: &'a Value,
}

impl PaymentReconciliation<'_> {
    /// Issuer's unique identifier for the payment instrument.
    pub fn payment_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("paymentIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// The period of time for which payments have been gathered into this bulk payment
    /// for settlement.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for paymentDate
    pub fn _payment_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paymentDate") {
            return Some(Element { value: val });
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

    /// The date when the resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
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

    /// A code for the form to be used for printing the content.
    pub fn form_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("formCode") {
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

    /// The outcome of a request for a reconciliation.
    pub fn outcome(&self) -> Option<PaymentReconciliationOutcome> {
        if let Some(Value::String(val)) = self.value.get("outcome") {
            return Some(PaymentReconciliationOutcome::from_string(&val).unwrap());
        }
        return None;
    }

    /// A note that describes or explains the processing in a human readable form.
    pub fn process_note(&self) -> Option<Vec<PaymentReconciliation_ProcessNote>> {
        if let Some(Value::Array(val)) = self.value.get("processNote") {
            return Some(
                val.into_iter()
                    .map(|e| PaymentReconciliation_ProcessNote { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The party who generated the payment.
    pub fn payment_issuer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("paymentIssuer") {
            return Some(Reference { value: val });
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

    /// A unique identifier assigned to this payment reconciliation.
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

    /// The date of payment as indicated on the financial instrument.
    pub fn payment_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("paymentDate") {
            return Some(string);
        }
        return None;
    }

    /// Distribution of the payment amount for a previously acknowledged payable.
    pub fn detail(&self) -> Option<Vec<PaymentReconciliation_Detail>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| PaymentReconciliation_Detail { value: e })
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// A human readable description of the status of the request for the
    /// reconciliation.
    pub fn disposition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disposition") {
            return Some(string);
        }
        return None;
    }

    /// The practitioner who is responsible for the services rendered to the patient.
    pub fn requestor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestor") {
            return Some(Reference { value: val });
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

    /// The status of the resource instance.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
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

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element { value: val });
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

    /// Total payment amount as indicated on the financial instrument.
    pub fn payment_amount(&self) -> Money {
        Money {
            value: &self.value["paymentAmount"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.payment_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self._payment_date() {
            _val.validate();
        }
        if let Some(_val) = self.request() {
            _val.validate();
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.form_code() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.outcome() {}
        if let Some(_val) = self.process_note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.payment_issuer() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.payment_date() {}
        if let Some(_val) = self.detail() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.disposition() {}
        if let Some(_val) = self.requestor() {
            _val.validate();
        }
        if let Some(_val) = self._disposition() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._created() {
            _val.validate();
        }
        if let Some(_val) = self._outcome() {
            _val.validate();
        }
        let _ = self.payment_amount().validate();
        return true;
    }
}

#[derive(Debug)]
pub enum PaymentReconciliationOutcome {
    Queued,
    Complete,
    Error,
    Partial,
}

impl PaymentReconciliationOutcome {
    pub fn from_string(string: &str) -> Option<PaymentReconciliationOutcome> {
        match string {
            "queued" => Some(PaymentReconciliationOutcome::Queued),
            "complete" => Some(PaymentReconciliationOutcome::Complete),
            "error" => Some(PaymentReconciliationOutcome::Error),
            "partial" => Some(PaymentReconciliationOutcome::Partial),
            _ => None,
        }
    }
}

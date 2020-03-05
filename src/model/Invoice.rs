#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Invoice_LineItem::Invoice_LineItem;
use crate::model::Invoice_Participant::Invoice_Participant;
use crate::model::Invoice_PriceComponent::Invoice_PriceComponent;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.

#[derive(Debug)]
pub struct Invoice<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Invoice<'_> {
    pub fn new(value: &Value) -> Invoice {
        Invoice {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for cancelledReason
    pub fn _cancelled_reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cancelledReason") {
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

    /// Extensions for paymentTerms
    pub fn _payment_terms(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paymentTerms") {
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

    /// Account which is supposed to be balanced with this Invoice.
    pub fn account(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("account") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// In case of Invoice cancellation a reason must be given (entered in error,
    /// superseded by corrected invoice etc.).
    pub fn cancelled_reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("cancelledReason") {
            return Some(string);
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

    /// Date/time(s) of when this Invoice was posted.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
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

    /// Identifier of this Invoice, often used for reference in correspondence about
    /// this invoice or for tracking of payments.
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

    /// The organizationissuing the Invoice.
    pub fn issuer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("issuer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Each line item represents one charge for goods and services rendered. Details
    /// such as date, code and amount are found in the referenced ChargeItem resource.
    pub fn line_item(&self) -> Option<Vec<Invoice_LineItem>> {
        if let Some(Value::Array(val)) = self.value.get("lineItem") {
            return Some(
                val.into_iter()
                    .map(|e| Invoice_LineItem {
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

    /// Comments made about the invoice by the issuer, subject, or other participants.
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

    /// Indicates who or what performed or participated in the charged service.
    pub fn participant(&self) -> Option<Vec<Invoice_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| Invoice_Participant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Payment details such as banking details, period of payment, deductibles, methods
    /// of payment.
    pub fn payment_terms(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("paymentTerms") {
            return Some(string);
        }
        return None;
    }

    /// The individual or Organization responsible for balancing of this invoice.
    pub fn recipient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recipient") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The current state of the Invoice.
    pub fn status(&self) -> Option<InvoiceStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(InvoiceStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The individual or set of individuals receiving the goods and services billed in
    /// this invoice.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Invoice total, tax included.
    pub fn total_gross(&self) -> Option<Money> {
        if let Some(val) = self.value.get("totalGross") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Invoice total , taxes excluded.
    pub fn total_net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("totalNet") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The total amount for the Invoice may be calculated as the sum of the line items
    /// with surcharges/deductions that apply in certain conditions.  The priceComponent
    /// element can be used to offer transparency to the recipient of the Invoice of how
    /// the total price was calculated.
    pub fn total_price_component(&self) -> Option<Vec<Invoice_PriceComponent>> {
        if let Some(Value::Array(val)) = self.value.get("totalPriceComponent") {
            return Some(
                val.into_iter()
                    .map(|e| Invoice_PriceComponent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of Invoice depending on domain, realm an usage (e.g. internal/external,
    /// dental, preliminary).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._cancelled_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
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
        if let Some(_val) = self._payment_terms() {
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
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.cancelled_reason() {}
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
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
        if let Some(_val) = self.issuer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.line_item() {
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.payment_terms() {}
        if let Some(_val) = self.recipient() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.total_gross() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.total_net() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.total_price_component() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct InvoiceBuilder {
    pub(crate) value: Value,
}

impl InvoiceBuilder {
    pub fn build(&self) -> Invoice {
        Invoice {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Invoice) -> InvoiceBuilder {
        InvoiceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> InvoiceBuilder {
        let mut __value: Value = json!({});
        return InvoiceBuilder { value: __value };
    }

    pub fn _cancelled_reason<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_cancelledReason"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _payment_terms<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_paymentTerms"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut InvoiceBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn account<'a>(&'a mut self, val: Reference) -> &'a mut InvoiceBuilder {
        self.value["account"] = json!(val.value);
        return self;
    }

    pub fn cancelled_reason<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["cancelledReason"] = json!(val);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut InvoiceBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut InvoiceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut InvoiceBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn issuer<'a>(&'a mut self, val: Reference) -> &'a mut InvoiceBuilder {
        self.value["issuer"] = json!(val.value);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn line_item<'a>(&'a mut self, val: Vec<Invoice_LineItem>) -> &'a mut InvoiceBuilder {
        self.value["lineItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut InvoiceBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut InvoiceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut InvoiceBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant<'a>(&'a mut self, val: Vec<Invoice_Participant>) -> &'a mut InvoiceBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn payment_terms<'a>(&'a mut self, val: &str) -> &'a mut InvoiceBuilder {
        self.value["paymentTerms"] = json!(val);
        return self;
    }

    pub fn recipient<'a>(&'a mut self, val: Reference) -> &'a mut InvoiceBuilder {
        self.value["recipient"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: InvoiceStatus) -> &'a mut InvoiceBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut InvoiceBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut InvoiceBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn total_gross<'a>(&'a mut self, val: Money) -> &'a mut InvoiceBuilder {
        self.value["totalGross"] = json!(val.value);
        return self;
    }

    pub fn total_net<'a>(&'a mut self, val: Money) -> &'a mut InvoiceBuilder {
        self.value["totalNet"] = json!(val.value);
        return self;
    }

    pub fn total_price_component<'a>(
        &'a mut self,
        val: Vec<Invoice_PriceComponent>,
    ) -> &'a mut InvoiceBuilder {
        self.value["totalPriceComponent"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut InvoiceBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Balanced,
    Cancelled,
    EnteredInError,
}

impl InvoiceStatus {
    pub fn from_string(string: &str) -> Option<InvoiceStatus> {
        match string {
            "draft" => Some(InvoiceStatus::Draft),
            "issued" => Some(InvoiceStatus::Issued),
            "balanced" => Some(InvoiceStatus::Balanced),
            "cancelled" => Some(InvoiceStatus::Cancelled),
            "entered-in-error" => Some(InvoiceStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            InvoiceStatus::Draft => "draft".to_string(),
            InvoiceStatus::Issued => "issued".to_string(),
            InvoiceStatus::Balanced => "balanced".to_string(),
            InvoiceStatus::Cancelled => "cancelled".to_string(),
            InvoiceStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}

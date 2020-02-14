#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::ChargeItem_Performer::ChargeItem_Performer;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// The resource ChargeItem describes the provision of healthcare provider products
/// for a certain patient, therefore referring not only to the product, but
/// containing in addition details of the provision, like date, time, amounts and
/// participating organizations and persons. Main Usage of the ChargeItem is to
/// enable the billing process and internal cost allocation.

#[derive(Debug)]
pub struct ChargeItem<'a> {
    pub value: &'a Value,
}

impl ChargeItem<'_> {
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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

    /// References the source of pricing information, rules of application for the code
    /// this ChargeItem uses.
    pub fn definition_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("definitionCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// ChargeItems can be grouped to larger ChargeItems covering the whole set.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Quantity of which the charge item has been serviced.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Factor overriding the factor determined by the rules associated with the code.
    pub fn factor_override(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factorOverride") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The organization requesting the service.
    pub fn performing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performingOrganization") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The device, practitioner, etc. who entered the charge item.
    pub fn enterer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("enterer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Identifies the device, food, drug or other product being charged either by type
    /// code or reference to an instance.
    pub fn product_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// The anatomical location where the related service has been applied.
    pub fn bodysite(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodysite") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates who or what performed or participated in the charged service.
    pub fn performer(&self) -> Option<Vec<ChargeItem_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItem_Performer { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifiers assigned to this event performer or other systems.
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

    /// Further information supporting this charge.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization performing the service.
    pub fn requesting_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestingOrganization") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for definitionUri
    pub fn _definition_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_definitionUri") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The individual or set of individuals the action is being or was performed on.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// Extensions for factorOverride
    pub fn _factor_override(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factorOverride") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Total price of the charge overriding the list price associated with the code.
    pub fn price_override(&self) -> Option<Money> {
        if let Some(val) = self.value.get("priceOverride") {
            return Some(Money { value: val });
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

    /// Identifies the device, food, drug or other product being charged either by type
    /// code or reference to an instance.
    pub fn product_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("productReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The financial cost center permits the tracking of charge attribution.
    pub fn cost_center(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("costCenter") {
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

    /// A code that identifies the charge, like a billing code.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If the list price or the rule-based factor associated with the code is
    /// overridden, this attribute can capture a text to indicate the  reason for this
    /// action.
    pub fn override_reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("overrideReason") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for overrideReason
    pub fn _override_reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_overrideReason") {
            return Some(Element { value: val });
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

    /// Account into which this ChargeItems belongs.
    pub fn account(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("account") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Comments made about the event by the performer, subject or other participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// References the (external) source of pricing information, rules of application
    /// for the code this ChargeItem uses.
    pub fn definition_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("definitionUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Describes why the event occurred in coded or textual form.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for enteredDate
    pub fn _entered_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_enteredDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The encounter or episode of care that establishes the context for this event.
    pub fn context(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("context") {
            return Some(Reference { value: val });
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

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// Indicated the rendered service that caused this charge.
    pub fn service(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("service") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// The current state of the ChargeItem.
    pub fn status(&self) -> Option<ChargeItemStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ChargeItemStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Date the charge item was entered.
    pub fn entered_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("enteredDate") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.definition_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.factor_override() {}
        if let Some(_val) = self.performing_organization() {
            _val.validate();
        }
        if let Some(_val) = self.enterer() {
            _val.validate();
        }
        if let Some(_val) = self.product_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.bodysite() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.supporting_information() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.requesting_organization() {
            _val.validate();
        }
        if let Some(_val) = self._definition_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.subject().validate();
        if let Some(_val) = self._factor_override() {
            _val.validate();
        }
        if let Some(_val) = self.price_override() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.product_reference() {
            _val.validate();
        }
        if let Some(_val) = self.cost_center() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        let _ = self.code().validate();
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.override_reason() {}
        if let Some(_val) = self._override_reason() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.account() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.definition_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.reason() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._entered_date() {
            _val.validate();
        }
        if let Some(_val) = self.context() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_timing() {
            _val.validate();
        }
        if let Some(_val) = self.service() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.occurrence_period() {
            _val.validate();
        }
        if let Some(_val) = self.entered_date() {}
        if let Some(_val) = self.language() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ChargeItemStatus {
    Planned,
    Billable,
    NotBillable,
    Aborted,
    Billed,
    EnteredInError,
    Unknown,
}

impl ChargeItemStatus {
    pub fn from_string(string: &str) -> Option<ChargeItemStatus> {
        match string {
            "planned" => Some(ChargeItemStatus::Planned),
            "billable" => Some(ChargeItemStatus::Billable),
            "not-billable" => Some(ChargeItemStatus::NotBillable),
            "aborted" => Some(ChargeItemStatus::Aborted),
            "billed" => Some(ChargeItemStatus::Billed),
            "entered-in-error" => Some(ChargeItemStatus::EnteredInError),
            "unknown" => Some(ChargeItemStatus::Unknown),
            _ => None,
        }
    }
}

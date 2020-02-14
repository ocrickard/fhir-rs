#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::SupplyRequest_Parameter::SupplyRequest_Parameter;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A record of a request for a medication, substance or device used in the
/// healthcare setting.

#[derive(Debug)]
pub struct SupplyRequest<'a> {
    pub value: &'a Value,
}

impl SupplyRequest<'_> {
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// The amount that is being ordered of the indicated item.
    pub fn quantity(&self) -> Quantity {
        Quantity {
            value: &self.value["quantity"],
        }
    }

    /// Extensions for authoredOn
    pub fn _authored_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authoredOn") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The reason why the supply item was requested.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Business identifiers assigned to this SupplyRequest by the author and/or other
    /// systems. These identifiers remain constant as the resource is updated and
    /// propagates from server to server.
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

    /// The item that is requested to be supplied. This is either a link to a resource
    /// representing the details of the item or a code that identifies the item from a
    /// known list.
    pub fn item_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("itemCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Status of the supply request.
    pub fn status(&self) -> Option<SupplyRequestStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(SupplyRequestStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// When the request should be fulfilled.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Who is intended to fulfill the request.
    pub fn supplier(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supplier") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// When the request should be fulfilled.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
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

    /// The item that is requested to be supplied. This is either a link to a resource
    /// representing the details of the item or a code that identifies the item from a
    /// known list.
    pub fn item_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("itemReference") {
            return Some(Reference { value: val });
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The device, practitioner, etc. who initiated the request.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference { value: val });
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

    /// Where the supply is expected to come from.
    pub fn deliver_from(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("deliverFrom") {
            return Some(Reference { value: val });
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

    /// The reason why the supply item was requested.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When the request was made.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
            return Some(string);
        }
        return None;
    }

    /// When the request should be fulfilled.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// Category of supply, e.g.  central, non-stock, etc. This is used to support work
    /// flows associated with the supply process.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept { value: val });
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

    /// Specific parameters for the ordered item.  For example, the size of the
    /// indicated item.
    pub fn parameter(&self) -> Option<Vec<SupplyRequest_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| SupplyRequest_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the supply is destined to go.
    pub fn deliver_to(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("deliverTo") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element { value: val });
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

    /// Indicates how quickly this SupplyRequest should be addressed with respect to
    /// other requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.quantity().validate();
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.item_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.occurrence_period() {
            _val.validate();
        }
        if let Some(_val) = self.supplier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.item_reference() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.requester() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.deliver_from() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self.occurrence_timing() {
            _val.validate();
        }
        if let Some(_val) = self.category() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.deliver_to() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.priority() {}
        return true;
    }
}

#[derive(Debug)]
pub enum SupplyRequestStatus {
    Draft,
    Active,
    Suspended,
    Cancelled,
    Completed,
    EnteredInError,
    Unknown,
}

impl SupplyRequestStatus {
    pub fn from_string(string: &str) -> Option<SupplyRequestStatus> {
        match string {
            "draft" => Some(SupplyRequestStatus::Draft),
            "active" => Some(SupplyRequestStatus::Active),
            "suspended" => Some(SupplyRequestStatus::Suspended),
            "cancelled" => Some(SupplyRequestStatus::Cancelled),
            "completed" => Some(SupplyRequestStatus::Completed),
            "entered-in-error" => Some(SupplyRequestStatus::EnteredInError),
            "unknown" => Some(SupplyRequestStatus::Unknown),
            _ => None,
        }
    }
}

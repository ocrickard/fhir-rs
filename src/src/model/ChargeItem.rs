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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The resource ChargeItem describes the provision of healthcare provider products
/// for a certain patient, therefore referring not only to the product, but
/// containing in addition details of the provision, like date, time, amounts and
/// participating organizations and persons. Main Usage of the ChargeItem is to
/// enable the billing process and internal cost allocation.

#[derive(Debug)]
pub struct ChargeItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ChargeItem<'_> {
    pub fn new(value: &Value) -> ChargeItem {
        ChargeItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for definitionUri
    pub fn _definition_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_definitionUri") {
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

    /// Extensions for enteredDate
    pub fn _entered_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_enteredDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for factorOverride
    pub fn _factor_override(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factorOverride") {
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

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for overrideReason
    pub fn _override_reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_overrideReason") {
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

    /// Account into which this ChargeItems belongs.
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

    /// The anatomical location where the related service has been applied.
    pub fn bodysite(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodysite") {
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

    /// A code that identifies the charge, like a billing code.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
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

    /// The encounter or episode of care that establishes the context for this event.
    pub fn context(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("context") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The financial cost center permits the tracking of charge attribution.
    pub fn cost_center(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("costCenter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Date the charge item was entered.
    pub fn entered_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("enteredDate") {
            return Some(string);
        }
        return None;
    }

    /// The device, practitioner, etc. who entered the charge item.
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

    /// Factor overriding the factor determined by the rules associated with the code.
    pub fn factor_override(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factorOverride") {
            return Some(val.as_f64().unwrap());
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

    /// Identifiers assigned to this event performer or other systems.
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

    /// Comments made about the event by the performer, subject or other participants.
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

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Date/time(s) or duration when the charged service was applied.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
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

    /// ChargeItems can be grouped to larger ChargeItems covering the whole set.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
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

    /// Indicates who or what performed or participated in the charged service.
    pub fn performer(&self) -> Option<Vec<ChargeItem_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItem_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization requesting the service.
    pub fn performing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performingOrganization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Total price of the charge overriding the list price associated with the code.
    pub fn price_override(&self) -> Option<Money> {
        if let Some(val) = self.value.get("priceOverride") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the device, food, drug or other product being charged either by type
    /// code or reference to an instance.
    pub fn product_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the device, food, drug or other product being charged either by type
    /// code or reference to an instance.
    pub fn product_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("productReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Quantity of which the charge item has been serviced.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes why the event occurred in coded or textual form.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
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

    /// The organization performing the service.
    pub fn requesting_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestingOrganization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicated the rendered service that caused this charge.
    pub fn service(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("service") {
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

    /// The current state of the ChargeItem.
    pub fn status(&self) -> Option<ChargeItemStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ChargeItemStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The individual or set of individuals the action is being or was performed on.
    pub fn subject(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["subject"]),
        }
    }

    /// Further information supporting this charge.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition_uri() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._entered_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._factor_override() {
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
        if let Some(_val) = self._occurrence_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._override_reason() {
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
        if let Some(_val) = self.bodysite() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.cost_center() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.definition_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.entered_date() {}
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
        if let Some(_val) = self.factor_override() {}
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
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.occurrence_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.override_reason() {}
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performing_organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.price_override() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.product_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.product_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requesting_organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.service() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if !self.subject().validate() {
            return false;
        }
        if let Some(_val) = self.supporting_information() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ChargeItemBuilder {
    pub(crate) value: Value,
}

impl ChargeItemBuilder {
    pub fn build(&self) -> ChargeItem {
        ChargeItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ChargeItem) -> ChargeItemBuilder {
        ChargeItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept, subject: Reference) -> ChargeItemBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        __value["subject"] = json!(subject.value);
        return ChargeItemBuilder { value: __value };
    }

    pub fn _definition_uri<'a>(&'a mut self, val: Vec<Element>) -> &'a mut ChargeItemBuilder {
        self.value["_definitionUri"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _entered_date<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_enteredDate"] = json!(val.value);
        return self;
    }

    pub fn _factor_override<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_factorOverride"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_date_time<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_occurrenceDateTime"] = json!(val.value);
        return self;
    }

    pub fn _override_reason<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_overrideReason"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ChargeItemBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn account<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ChargeItemBuilder {
        self.value["account"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn bodysite<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ChargeItemBuilder {
        self.value["bodysite"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ChargeItemBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn context<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["context"] = json!(val.value);
        return self;
    }

    pub fn cost_center<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["costCenter"] = json!(val.value);
        return self;
    }

    pub fn definition_canonical<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ChargeItemBuilder {
        self.value["definitionCanonical"] = json!(val);
        return self;
    }

    pub fn definition_uri<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ChargeItemBuilder {
        self.value["definitionUri"] = json!(val);
        return self;
    }

    pub fn entered_date<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["enteredDate"] = json!(val);
        return self;
    }

    pub fn enterer<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["enterer"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ChargeItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor_override<'a>(&'a mut self, val: f64) -> &'a mut ChargeItemBuilder {
        self.value["factorOverride"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ChargeItemBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ChargeItemBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ChargeItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ChargeItemBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn occurrence_date_time<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["occurrenceDateTime"] = json!(val);
        return self;
    }

    pub fn occurrence_period<'a>(&'a mut self, val: Period) -> &'a mut ChargeItemBuilder {
        self.value["occurrencePeriod"] = json!(val.value);
        return self;
    }

    pub fn occurrence_timing<'a>(&'a mut self, val: Timing) -> &'a mut ChargeItemBuilder {
        self.value["occurrenceTiming"] = json!(val.value);
        return self;
    }

    pub fn override_reason<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemBuilder {
        self.value["overrideReason"] = json!(val);
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ChargeItemBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<ChargeItem_Performer>,
    ) -> &'a mut ChargeItemBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performing_organization<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["performingOrganization"] = json!(val.value);
        return self;
    }

    pub fn price_override<'a>(&'a mut self, val: Money) -> &'a mut ChargeItemBuilder {
        self.value["priceOverride"] = json!(val.value);
        return self;
    }

    pub fn product_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ChargeItemBuilder {
        self.value["productCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn product_reference<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["productReference"] = json!(val.value);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut ChargeItemBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ChargeItemBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requesting_organization<'a>(&'a mut self, val: Reference) -> &'a mut ChargeItemBuilder {
        self.value["requestingOrganization"] = json!(val.value);
        return self;
    }

    pub fn service<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ChargeItemBuilder {
        self.value["service"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: ChargeItemStatus) -> &'a mut ChargeItemBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn supporting_information<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ChargeItemBuilder {
        self.value["supportingInformation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ChargeItemBuilder {
        self.value["text"] = json!(val.value);
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            ChargeItemStatus::Planned => "planned".to_string(),
            ChargeItemStatus::Billable => "billable".to_string(),
            ChargeItemStatus::NotBillable => "not-billable".to_string(),
            ChargeItemStatus::Aborted => "aborted".to_string(),
            ChargeItemStatus::Billed => "billed".to_string(),
            ChargeItemStatus::EnteredInError => "entered-in-error".to_string(),
            ChargeItemStatus::Unknown => "unknown".to_string(),
        }
    }
}

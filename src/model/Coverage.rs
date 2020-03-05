#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coverage_Class::Coverage_Class;
use crate::model::Coverage_CostToBeneficiary::Coverage_CostToBeneficiary;
use crate::model::Element::Element;
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

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.

#[derive(Debug)]
pub struct Coverage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Coverage<'_> {
    pub fn new(value: &Value) -> Coverage {
        Coverage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dependent
    pub fn _dependent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dependent") {
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

    /// Extensions for network
    pub fn _network(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_network") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for order
    pub fn _order(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_order") {
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

    /// Extensions for subrogation
    pub fn _subrogation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subrogation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subscriberId
    pub fn _subscriber_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subscriberId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The party who benefits from the insurance coverage; the patient when products
    /// and/or services are provided.
    pub fn beneficiary(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["beneficiary"]),
        }
    }

    /// A suite of underwriter specific classifiers.
    pub fn class(&self) -> Option<Vec<Coverage_Class>> {
        if let Some(Value::Array(val)) = self.value.get("class") {
            return Some(
                val.into_iter()
                    .map(|e| Coverage_Class {
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

    /// The policy(s) which constitute this insurance coverage.
    pub fn contract(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("contract") {
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

    /// A suite of codes indicating the cost category and associated amount which have
    /// been detailed in the policy and may have been  included on the health card.
    pub fn cost_to_beneficiary(&self) -> Option<Vec<Coverage_CostToBeneficiary>> {
        if let Some(Value::Array(val)) = self.value.get("costToBeneficiary") {
            return Some(
                val.into_iter()
                    .map(|e| Coverage_CostToBeneficiary {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A unique identifier for a dependent under the coverage.
    pub fn dependent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dependent") {
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

    /// A unique identifier assigned to this coverage.
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

    /// The insurer-specific identifier for the insurer-defined network of providers to
    /// which the beneficiary may seek treatment which will be covered at the 'in-
    /// network' rate, otherwise 'out of network' terms and conditions apply.
    pub fn network(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("network") {
            return Some(string);
        }
        return None;
    }

    /// The order of applicability of this coverage relative to other coverages which
    /// are currently in force. Note, there may be gaps in the numbering and this does
    /// not imply primary, secondary etc. as the specific positioning of coverages
    /// depends upon the episode of care.
    pub fn order(&self) -> Option<i64> {
        if let Some(val) = self.value.get("order") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The program or plan underwriter or payor including both insurance and non-
    /// insurance agreements, such as patient-pay agreements.
    pub fn payor(&self) -> Vec<Reference> {
        self.value
            .get("payor")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Reference {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// Time period during which the coverage is in force. A missing start date
    /// indicates the start date isn't known, a missing end date means the coverage is
    /// continuing to be in force.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The party who 'owns' the insurance policy.
    pub fn policy_holder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("policyHolder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The relationship of beneficiary (patient) to the subscriber.
    pub fn relationship(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("relationship") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// When 'subrogation=true' this insurance instance has been included not for
    /// adjudication but to provide insurers with the details to recover costs.
    pub fn subrogation(&self) -> Option<bool> {
        if let Some(val) = self.value.get("subrogation") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The party who has signed-up for or 'owns' the contractual relationship to the
    /// policy or to whom the benefit of the policy for services rendered to them or
    /// their family is due.
    pub fn subscriber(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subscriber") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The insurer assigned ID for the Subscriber.
    pub fn subscriber_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subscriberId") {
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of coverage: social program, medical plan, accident coverage (workers
    /// compensation, auto), group health or payment by an individual or organization.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._dependent() {
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
        if let Some(_val) = self._network() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._order() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subrogation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subscriber_id() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.beneficiary().validate() {
            return false;
        }
        if let Some(_val) = self.class() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contract() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cost_to_beneficiary() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dependent() {}
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
        if let Some(_val) = self.network() {}
        if let Some(_val) = self.order() {}
        if !self
            .payor()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.policy_holder() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subrogation() {}
        if let Some(_val) = self.subscriber() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subscriber_id() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
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
pub struct CoverageBuilder {
    pub(crate) value: Value,
}

impl CoverageBuilder {
    pub fn build(&self) -> Coverage {
        Coverage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Coverage) -> CoverageBuilder {
        CoverageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(beneficiary: Reference, payor: Vec<Reference>) -> CoverageBuilder {
        let mut __value: Value = json!({});
        __value["beneficiary"] = json!(beneficiary.value);
        __value["payor"] = json!(payor.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return CoverageBuilder { value: __value };
    }

    pub fn _dependent<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_dependent"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _network<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_network"] = json!(val.value);
        return self;
    }

    pub fn _order<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_order"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _subrogation<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_subrogation"] = json!(val.value);
        return self;
    }

    pub fn _subscriber_id<'a>(&'a mut self, val: Element) -> &'a mut CoverageBuilder {
        self.value["_subscriberId"] = json!(val.value);
        return self;
    }

    pub fn class<'a>(&'a mut self, val: Vec<Coverage_Class>) -> &'a mut CoverageBuilder {
        self.value["class"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut CoverageBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contract<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut CoverageBuilder {
        self.value["contract"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cost_to_beneficiary<'a>(
        &'a mut self,
        val: Vec<Coverage_CostToBeneficiary>,
    ) -> &'a mut CoverageBuilder {
        self.value["costToBeneficiary"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dependent<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["dependent"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CoverageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut CoverageBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut CoverageBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CoverageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn network<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["network"] = json!(val);
        return self;
    }

    pub fn order<'a>(&'a mut self, val: i64) -> &'a mut CoverageBuilder {
        self.value["order"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut CoverageBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn policy_holder<'a>(&'a mut self, val: Reference) -> &'a mut CoverageBuilder {
        self.value["policyHolder"] = json!(val.value);
        return self;
    }

    pub fn relationship<'a>(&'a mut self, val: CodeableConcept) -> &'a mut CoverageBuilder {
        self.value["relationship"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn subrogation<'a>(&'a mut self, val: bool) -> &'a mut CoverageBuilder {
        self.value["subrogation"] = json!(val);
        return self;
    }

    pub fn subscriber<'a>(&'a mut self, val: Reference) -> &'a mut CoverageBuilder {
        self.value["subscriber"] = json!(val.value);
        return self;
    }

    pub fn subscriber_id<'a>(&'a mut self, val: &str) -> &'a mut CoverageBuilder {
        self.value["subscriberId"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut CoverageBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut CoverageBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Claim_Detail::Claim_Detail;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_Item<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Claim_Item<'_> {
    pub fn new(value: &Value) -> Claim_Item {
        Claim_Item {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for careTeamSequence
    pub fn _care_team_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_careTeamSequence") {
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

    /// Extensions for diagnosisSequence
    pub fn _diagnosis_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_diagnosisSequence") {
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

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for informationSequence
    pub fn _information_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_informationSequence") {
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

    /// Extensions for procedureSequence
    pub fn _procedure_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_procedureSequence") {
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

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for servicedDate
    pub fn _serviced_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_servicedDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Physical service site on the patient (limb, tooth, etc.).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// CareTeam members related to this service or product.
    pub fn care_team_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("careTeamSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub fn detail(&self) -> Option<Vec<Claim_Detail>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Detail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Diagnosis applicable for this service or product.
    pub fn diagnosis_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosisSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Encounters during which this Claim was created or to which the creation of
    /// this record is tightly associated.
    pub fn encounter(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("encounter") {
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub fn factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Exceptions, special conditions and supporting information applicable for this
    /// service or product.
    pub fn information_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("informationSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("locationAddress") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("locationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("locationReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// The quantity times the unit price for an additional service or product or
    /// charge.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Procedures applicable for this service or product.
    pub fn procedure_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("procedureSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When the value is a group code then this item collects a set of related claim
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item.
    pub fn product_or_service(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["productOrService"]),
        }
    }

    /// Identifies the program under which this may be recovered.
    pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programCode") {
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

    /// The number of repetitions of a service or product.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of revenue or cost center providing the product and/or service.
    pub fn revenue(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("revenue") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A number to uniquely identify item entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("servicedDate") {
            return Some(string);
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("servicedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
    pub fn sub_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subSite") {
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

    /// Unique Device Identifiers associated with this line item.
    pub fn udi(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("udi") {
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

    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._care_team_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._diagnosis_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._information_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._procedure_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._serviced_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.care_team_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.detail() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.diagnosis_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.encounter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.information_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.location_address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.net() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.procedure_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if !self.product_or_service().validate() {
            return false;
        }
        if let Some(_val) = self.program_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.revenue() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.serviced_date() {}
        if let Some(_val) = self.serviced_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sub_site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.udi() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.unit_price() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Claim_ItemBuilder {
    pub(crate) value: Value,
}

impl Claim_ItemBuilder {
    pub fn build(&self) -> Claim_Item {
        Claim_Item {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Claim_Item) -> Claim_ItemBuilder {
        Claim_ItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(product_or_service: CodeableConcept) -> Claim_ItemBuilder {
        let mut __value: Value = json!({});
        __value["productOrService"] = json!(product_or_service.value);
        return Claim_ItemBuilder { value: __value };
    }

    pub fn _care_team_sequence<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Claim_ItemBuilder {
        self.value["_careTeamSequence"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _diagnosis_sequence<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Claim_ItemBuilder {
        self.value["_diagnosisSequence"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _factor<'a>(&'a mut self, val: Element) -> &'a mut Claim_ItemBuilder {
        self.value["_factor"] = json!(val.value);
        return self;
    }

    pub fn _information_sequence<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Claim_ItemBuilder {
        self.value["_informationSequence"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _procedure_sequence<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Claim_ItemBuilder {
        self.value["_procedureSequence"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _sequence<'a>(&'a mut self, val: Element) -> &'a mut Claim_ItemBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn _serviced_date<'a>(&'a mut self, val: Element) -> &'a mut Claim_ItemBuilder {
        self.value["_servicedDate"] = json!(val.value);
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_ItemBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn care_team_sequence<'a>(&'a mut self, val: Vec<i64>) -> &'a mut Claim_ItemBuilder {
        self.value["careTeamSequence"] = json!(val);
        return self;
    }

    pub fn category<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_ItemBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn detail<'a>(&'a mut self, val: Vec<Claim_Detail>) -> &'a mut Claim_ItemBuilder {
        self.value["detail"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn diagnosis_sequence<'a>(&'a mut self, val: Vec<i64>) -> &'a mut Claim_ItemBuilder {
        self.value["diagnosisSequence"] = json!(val);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Claim_ItemBuilder {
        self.value["encounter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Claim_ItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor<'a>(&'a mut self, val: f64) -> &'a mut Claim_ItemBuilder {
        self.value["factor"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Claim_ItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn information_sequence<'a>(&'a mut self, val: Vec<i64>) -> &'a mut Claim_ItemBuilder {
        self.value["informationSequence"] = json!(val);
        return self;
    }

    pub fn location_address<'a>(&'a mut self, val: Address) -> &'a mut Claim_ItemBuilder {
        self.value["locationAddress"] = json!(val.value);
        return self;
    }

    pub fn location_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Claim_ItemBuilder {
        self.value["locationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn location_reference<'a>(&'a mut self, val: Reference) -> &'a mut Claim_ItemBuilder {
        self.value["locationReference"] = json!(val.value);
        return self;
    }

    pub fn modifier<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Claim_ItemBuilder {
        self.value["modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Claim_ItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn net<'a>(&'a mut self, val: Money) -> &'a mut Claim_ItemBuilder {
        self.value["net"] = json!(val.value);
        return self;
    }

    pub fn procedure_sequence<'a>(&'a mut self, val: Vec<i64>) -> &'a mut Claim_ItemBuilder {
        self.value["procedureSequence"] = json!(val);
        return self;
    }

    pub fn program_code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Claim_ItemBuilder {
        self.value["programCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Claim_ItemBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn revenue<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_ItemBuilder {
        self.value["revenue"] = json!(val.value);
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: i64) -> &'a mut Claim_ItemBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }

    pub fn serviced_date<'a>(&'a mut self, val: &str) -> &'a mut Claim_ItemBuilder {
        self.value["servicedDate"] = json!(val);
        return self;
    }

    pub fn serviced_period<'a>(&'a mut self, val: Period) -> &'a mut Claim_ItemBuilder {
        self.value["servicedPeriod"] = json!(val.value);
        return self;
    }

    pub fn sub_site<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Claim_ItemBuilder {
        self.value["subSite"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn udi<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Claim_ItemBuilder {
        self.value["udi"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn unit_price<'a>(&'a mut self, val: Money) -> &'a mut Claim_ItemBuilder {
        self.value["unitPrice"] = json!(val.value);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_ValuedItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_ValuedItem<'_> {
    pub fn new(value: &Value) -> Contract_ValuedItem {
        Contract_ValuedItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for effectiveTime
    pub fn _effective_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_linkId") {
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

    /// Extensions for payment
    pub fn _payment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_payment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for paymentDate
    pub fn _payment_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paymentDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for points
    pub fn _points(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_points") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for securityLabelNumber
    pub fn _security_label_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_securityLabelNumber") {
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

    /// Indicates the time during which this Contract ValuedItem information is
    /// effective.
    pub fn effective_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveTime") {
            return Some(string);
        }
        return None;
    }

    /// Specific type of Contract Valued Item that may be priced.
    pub fn entity_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("entityCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific type of Contract Valued Item that may be priced.
    pub fn entity_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("entityReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
    /// of the Contract Valued Item delivered. The concept of a Factor allows for a
    /// discount or surcharge multiplier to be applied to a monetary amount.
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

    /// Identifies a Contract Valued Item instance.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Id  of the clause or question text related to the context of this valuedItem in
    /// the referenced form or QuestionnaireResponse.
    pub fn link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("linkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Expresses the product of the Contract Valued Item unitQuantity and the
    /// unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per
    /// Point) * factor Number  * points = net Amount. Quantity, factor and points are
    /// assumed to be 1 if not supplied.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Terms of valuation.
    pub fn payment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("payment") {
            return Some(string);
        }
        return None;
    }

    /// When payment is due.
    pub fn payment_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("paymentDate") {
            return Some(string);
        }
        return None;
    }

    /// An amount that expresses the weighting (based on difficulty, cost and/or
    /// resource intensiveness) associated with the Contract Valued Item delivered. The
    /// concept of Points allows for assignment of point values for a Contract Valued
    /// Item, such that a monetary amount can be assigned to each point.
    pub fn points(&self) -> Option<f64> {
        if let Some(val) = self.value.get("points") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies the units by which the Contract Valued Item is measured or counted,
    /// and quantifies the countable or measurable Contract Valued Item instances.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Who will receive payment.
    pub fn recipient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recipient") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Who will make payment.
    pub fn responsible(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("responsible") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A set of security labels that define which terms are controlled by this
    /// condition.
    pub fn security_label_number(&self) -> Option<Vec<u64>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabelNumber") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_u64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A Contract Valued Item unit valuation measure.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._effective_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._payment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._payment_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._points() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._security_label_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_time() {}
        if let Some(_val) = self.entity_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.entity_reference() {
            if !_val.validate() {
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
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.link_id() {
            _val.into_iter().for_each(|_e| {});
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
        if let Some(_val) = self.payment() {}
        if let Some(_val) = self.payment_date() {}
        if let Some(_val) = self.points() {}
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.recipient() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.responsible() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.security_label_number() {
            _val.into_iter().for_each(|_e| {});
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
pub struct Contract_ValuedItemBuilder {
    pub(crate) value: Value,
}

impl Contract_ValuedItemBuilder {
    pub fn build(&self) -> Contract_ValuedItem {
        Contract_ValuedItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_ValuedItem) -> Contract_ValuedItemBuilder {
        Contract_ValuedItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Contract_ValuedItemBuilder {
        let mut __value: Value = json!({});
        return Contract_ValuedItemBuilder { value: __value };
    }

    pub fn _effective_time<'a>(&'a mut self, val: Element) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_effectiveTime"] = json!(val.value);
        return self;
    }

    pub fn _factor<'a>(&'a mut self, val: Element) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_factor"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_linkId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _payment<'a>(&'a mut self, val: Element) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_payment"] = json!(val.value);
        return self;
    }

    pub fn _payment_date<'a>(&'a mut self, val: Element) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_paymentDate"] = json!(val.value);
        return self;
    }

    pub fn _points<'a>(&'a mut self, val: Element) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_points"] = json!(val.value);
        return self;
    }

    pub fn _security_label_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Contract_ValuedItemBuilder {
        self.value["_securityLabelNumber"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_time<'a>(&'a mut self, val: &str) -> &'a mut Contract_ValuedItemBuilder {
        self.value["effectiveTime"] = json!(val);
        return self;
    }

    pub fn entity_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Contract_ValuedItemBuilder {
        self.value["entityCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn entity_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Contract_ValuedItemBuilder {
        self.value["entityReference"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_ValuedItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor<'a>(&'a mut self, val: f64) -> &'a mut Contract_ValuedItemBuilder {
        self.value["factor"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_ValuedItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut Contract_ValuedItemBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ValuedItemBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_ValuedItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn net<'a>(&'a mut self, val: Money) -> &'a mut Contract_ValuedItemBuilder {
        self.value["net"] = json!(val.value);
        return self;
    }

    pub fn payment<'a>(&'a mut self, val: &str) -> &'a mut Contract_ValuedItemBuilder {
        self.value["payment"] = json!(val);
        return self;
    }

    pub fn payment_date<'a>(&'a mut self, val: &str) -> &'a mut Contract_ValuedItemBuilder {
        self.value["paymentDate"] = json!(val);
        return self;
    }

    pub fn points<'a>(&'a mut self, val: f64) -> &'a mut Contract_ValuedItemBuilder {
        self.value["points"] = json!(val);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Contract_ValuedItemBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn recipient<'a>(&'a mut self, val: Reference) -> &'a mut Contract_ValuedItemBuilder {
        self.value["recipient"] = json!(val.value);
        return self;
    }

    pub fn responsible<'a>(&'a mut self, val: Reference) -> &'a mut Contract_ValuedItemBuilder {
        self.value["responsible"] = json!(val.value);
        return self;
    }

    pub fn security_label_number<'a>(
        &'a mut self,
        val: Vec<u64>,
    ) -> &'a mut Contract_ValuedItemBuilder {
        self.value["securityLabelNumber"] = json!(val);
        return self;
    }

    pub fn unit_price<'a>(&'a mut self, val: Money) -> &'a mut Contract_ValuedItemBuilder {
        self.value["unitPrice"] = json!(val.value);
        return self;
    }
}

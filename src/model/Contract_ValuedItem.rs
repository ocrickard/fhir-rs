#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_ValuedItem<'a> {
    pub value: &'a Value,
}

impl Contract_ValuedItem<'_> {
    /// Who will make payment.
    pub fn responsible(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("responsible") {
            return Some(Reference { value: val });
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

    /// Extensions for paymentDate
    pub fn _payment_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paymentDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_linkId") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies a Contract Valued Item instance.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for points
    pub fn _points(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_points") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for securityLabelNumber
    pub fn _security_label_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_securityLabelNumber") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Specific type of Contract Valued Item that may be priced.
    pub fn entity_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("entityReference") {
            return Some(Reference { value: val });
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

    /// Specific type of Contract Valued Item that may be priced.
    pub fn entity_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("entityCodeableConcept") {
            return Some(CodeableConcept { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A Contract Valued Item unit valuation measure.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// Specifies the units by which the Contract Valued Item is measured or counted,
    /// and quantifies the countable or measurable Contract Valued Item instances.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
            return Some(Element { value: val });
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

    /// Expresses the product of the Contract Valued Item unitQuantity and the
    /// unitPriceAmt. For example, the formula: unit Quantity * unit Price (Cost per
    /// Point) * factor Number  * points = net Amount. Quantity, factor and points are
    /// assumed to be 1 if not supplied.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// Who will receive payment.
    pub fn recipient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recipient") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for effectiveTime
    pub fn _effective_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveTime") {
            return Some(Element { value: val });
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

    /// Extensions for payment
    pub fn _payment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_payment") {
            return Some(Element { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.responsible() {
            _val.validate();
        }
        if let Some(_val) = self.link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._payment_date() {
            _val.validate();
        }
        if let Some(_val) = self._link_id() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.security_label_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._points() {
            _val.validate();
        }
        if let Some(_val) = self._security_label_number() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.payment() {}
        if let Some(_val) = self.entity_reference() {
            _val.validate();
        }
        if let Some(_val) = self.payment_date() {}
        if let Some(_val) = self.entity_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.unit_price() {
            _val.validate();
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self._factor() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.net() {
            _val.validate();
        }
        if let Some(_val) = self.recipient() {
            _val.validate();
        }
        if let Some(_val) = self._effective_time() {
            _val.validate();
        }
        if let Some(_val) = self.effective_time() {}
        if let Some(_val) = self._payment() {
            _val.validate();
        }
        if let Some(_val) = self.points() {}
        return true;
    }
}

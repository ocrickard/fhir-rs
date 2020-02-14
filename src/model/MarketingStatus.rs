#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// The marketing status describes the date when a medicinal product is actually put
/// on the market or the date as of which it is no longer available.

#[derive(Debug)]
pub struct MarketingStatus<'a> {
    pub value: &'a Value,
}

impl MarketingStatus<'_> {
    /// This attribute provides information on the status of the marketing of the
    /// medicinal product See ISO/TS 20443 for more information and examples.
    pub fn status(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["status"],
        }
    }

    /// Where a Medicines Regulatory Agency has granted a marketing authorisation for
    /// which specific provisions within a jurisdiction apply, the jurisdiction can be
    /// specified using an appropriate controlled terminology The controlled term and
    /// the controlled term identifier shall be specified.
    pub fn jurisdiction(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("jurisdiction") {
            return Some(CodeableConcept { value: val });
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

    /// The date when the Medicinal Product is placed on the market by the Marketing
    /// Authorisation Holder (or where applicable, the manufacturer/distributor) in a
    /// country and/or jurisdiction shall be provided A complete date consisting of day,
    /// month and year shall be specified using the ISO 8601 date format NOTE “Placed on
    /// the market” refers to the release of the Medicinal Product into the distribution
    /// chain.
    pub fn restore_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("restoreDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for restoreDate
    pub fn _restore_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_restoreDate") {
            return Some(Element { value: val });
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

    /// The date when the Medicinal Product is placed on the market by the Marketing
    /// Authorisation Holder (or where applicable, the manufacturer/distributor) in a
    /// country and/or jurisdiction shall be provided A complete date consisting of day,
    /// month and year shall be specified using the ISO 8601 date format NOTE “Placed on
    /// the market” refers to the release of the Medicinal Product into the distribution
    /// chain.
    pub fn date_range(&self) -> Period {
        Period {
            value: &self.value["dateRange"],
        }
    }

    /// The country in which the marketing authorisation has been granted shall be
    /// specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements.
    pub fn country(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["country"],
        }
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        let _ = self.status().validate();
        if let Some(_val) = self.jurisdiction() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.restore_date() {}
        if let Some(_val) = self._restore_date() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.date_range().validate();
        let _ = self.country().validate();
        if let Some(_val) = self.id() {}
        return true;
    }
}

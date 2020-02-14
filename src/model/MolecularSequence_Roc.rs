#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Roc<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_Roc<'_> {
    /// Calculated sensitivity if the GQ score threshold was set to "score" field value.
    pub fn sensitivity(&self) -> Option<Vec<f64>> {
        if let Some(Value::Array(val)) = self.value.get("sensitivity") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_f64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for score
    pub fn _score(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_score") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of false negatives if the GQ score threshold was set to "score" field
    /// value.
    pub fn num_f_n(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("numFN") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Invidual data point representing the GQ (genotype quality) score threshold.
    pub fn score(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("score") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of true positives if the GQ score threshold was set to "score" field
    /// value.
    pub fn num_t_p(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("numTP") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for fMeasure
    pub fn _f_measure(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_fMeasure") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of false positives if the GQ score threshold was set to "score" field
    /// value.
    pub fn num_f_p(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("numFP") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for numFP
    pub fn _num_f_p(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numFP") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for numTP
    pub fn _num_t_p(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numTP") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Calculated precision if the GQ score threshold was set to "score" field value.
    pub fn precision(&self) -> Option<Vec<f64>> {
        if let Some(Value::Array(val)) = self.value.get("precision") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_f64().unwrap())
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for sensitivity
    pub fn _sensitivity(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_sensitivity") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for precision
    pub fn _precision(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_precision") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for numFN
    pub fn _num_f_n(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numFN") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Calculated fScore if the GQ score threshold was set to "score" field value.
    pub fn f_measure(&self) -> Option<Vec<f64>> {
        if let Some(Value::Array(val)) = self.value.get("fMeasure") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_f64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.sensitivity() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._score() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.num_f_n() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.score() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.num_t_p() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._f_measure() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.num_f_p() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._num_f_p() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._num_t_p() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.precision() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._sensitivity() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._precision() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._num_f_n() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.f_measure() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

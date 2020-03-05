#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Roc<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MolecularSequence_Roc<'_> {
    pub fn new(value: &Value) -> MolecularSequence_Roc {
        MolecularSequence_Roc {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for fMeasure
    pub fn _f_measure(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_fMeasure") {
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

    /// Extensions for numFN
    pub fn _num_f_n(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numFN") {
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

    /// Extensions for numFP
    pub fn _num_f_p(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numFP") {
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

    /// Extensions for numTP
    pub fn _num_t_p(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_numTP") {
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

    /// Extensions for precision
    pub fn _precision(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_precision") {
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

    /// Extensions for score
    pub fn _score(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_score") {
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

    /// Extensions for sensitivity
    pub fn _sensitivity(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_sensitivity") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._f_measure() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._num_f_n() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._num_f_p() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._num_t_p() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._precision() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._score() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._sensitivity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.f_measure() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.num_f_n() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.num_f_p() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.num_t_p() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.precision() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.score() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.sensitivity() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MolecularSequence_RocBuilder {
    pub(crate) value: Value,
}

impl MolecularSequence_RocBuilder {
    pub fn build(&self) -> MolecularSequence_Roc {
        MolecularSequence_Roc {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MolecularSequence_Roc) -> MolecularSequence_RocBuilder {
        MolecularSequence_RocBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MolecularSequence_RocBuilder {
        let mut __value: Value = json!({});
        return MolecularSequence_RocBuilder { value: __value };
    }

    pub fn _f_measure<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_fMeasure"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _num_f_n<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_numFN"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _num_f_p<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_numFP"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _num_t_p<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_numTP"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _precision<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_precision"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _score<'a>(&'a mut self, val: Vec<Element>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_score"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _sensitivity<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut MolecularSequence_RocBuilder {
        self.value["_sensitivity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_RocBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn f_measure<'a>(&'a mut self, val: Vec<f64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["fMeasure"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequence_RocBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_RocBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn num_f_n<'a>(&'a mut self, val: Vec<i64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["numFN"] = json!(val);
        return self;
    }

    pub fn num_f_p<'a>(&'a mut self, val: Vec<i64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["numFP"] = json!(val);
        return self;
    }

    pub fn num_t_p<'a>(&'a mut self, val: Vec<i64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["numTP"] = json!(val);
        return self;
    }

    pub fn precision<'a>(&'a mut self, val: Vec<f64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["precision"] = json!(val);
        return self;
    }

    pub fn score<'a>(&'a mut self, val: Vec<i64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["score"] = json!(val);
        return self;
    }

    pub fn sensitivity<'a>(&'a mut self, val: Vec<f64>) -> &'a mut MolecularSequence_RocBuilder {
        self.value["sensitivity"] = json!(val);
        return self;
    }
}

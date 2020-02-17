#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement_CodeFilter::DataRequirement_CodeFilter;
use crate::model::DataRequirement_DateFilter::DataRequirement_DateFilter;
use crate::model::DataRequirement_Sort::DataRequirement_Sort;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.

#[derive(Debug)]
pub struct DataRequirement<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DataRequirement<'_> {
    pub fn new(value: &Value) -> DataRequirement {
        DataRequirement {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for limit
    pub fn _limit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_limit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for mustSupport
    pub fn _must_support(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_mustSupport") {
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Code filters specify additional constraints on the data, specifying the value
    /// set of interest for a particular element of the data. Each code filter defines
    /// an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.
    pub fn code_filter(&self) -> Option<Vec<DataRequirement_CodeFilter>> {
        if let Some(Value::Array(val)) = self.value.get("codeFilter") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement_CodeFilter {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Date filters specify additional constraints on the data in terms of the
    /// applicable date range for specific elements. Each date filter specifies an
    /// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
    pub fn date_filter(&self) -> Option<Vec<DataRequirement_DateFilter>> {
        if let Some(Value::Array(val)) = self.value.get("dateFilter") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement_DateFilter {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a maximum number of results that are required (uses the _count search
    /// parameter).
    pub fn limit(&self) -> Option<i64> {
        if let Some(val) = self.value.get("limit") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Indicates that specific elements of the type are referenced by the knowledge
    /// module and must be supported by the consumer in order to obtain an effective
    /// evaluation. This does not mean that a value is required for this element, only
    /// that the consuming system must understand the element and be able to provide
    /// values for it if they are available.     The value of mustSupport SHALL be a
    /// FHIRPath resolveable on the type of the DataRequirement. The path SHALL consist
    /// only of identifiers, constant indexers, and .resolve() (see the [Simple FHIRPath
    /// Profile](fhirpath.html#simple) for full details).
    pub fn must_support(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("mustSupport") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The profile of the required data, specified as the uri of the profile
    /// definition.
    pub fn profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("profile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies the order of the results to be returned.
    pub fn sort(&self) -> Option<Vec<DataRequirement_Sort>> {
        if let Some(Value::Array(val)) = self.value.get("sort") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement_Sort {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The intended subjects of the data requirement. If this element is not provided,
    /// a Patient subject is assumed.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The intended subjects of the data requirement. If this element is not provided,
    /// a Patient subject is assumed.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of the required data, specified as the type name of a resource. For
    /// profiles, this value is set to the type of the base resource of the profile.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._limit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._must_support() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code_filter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_filter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.limit() {}
        if let Some(_val) = self.must_support() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.sort() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.subject_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subject_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct DataRequirementBuilder {
    pub(crate) value: Value,
}

impl DataRequirementBuilder {
    pub fn build(&self) -> DataRequirement {
        DataRequirement {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DataRequirement) -> DataRequirementBuilder {
        DataRequirementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DataRequirementBuilder {
        let mut __value: Value = json!({});
        return DataRequirementBuilder { value: __value };
    }

    pub fn _limit<'a>(&'a mut self, val: Element) -> &'a mut DataRequirementBuilder {
        self.value["_limit"] = json!(val.value);
        return self;
    }

    pub fn _must_support<'a>(&'a mut self, val: Vec<Element>) -> &'a mut DataRequirementBuilder {
        self.value["_mustSupport"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut DataRequirementBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn code_filter<'a>(
        &'a mut self,
        val: Vec<DataRequirement_CodeFilter>,
    ) -> &'a mut DataRequirementBuilder {
        self.value["codeFilter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_filter<'a>(
        &'a mut self,
        val: Vec<DataRequirement_DateFilter>,
    ) -> &'a mut DataRequirementBuilder {
        self.value["dateFilter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DataRequirementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DataRequirementBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn limit<'a>(&'a mut self, val: i64) -> &'a mut DataRequirementBuilder {
        self.value["limit"] = json!(val);
        return self;
    }

    pub fn must_support<'a>(&'a mut self, val: Vec<&str>) -> &'a mut DataRequirementBuilder {
        self.value["mustSupport"] = json!(val);
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: Vec<&str>) -> &'a mut DataRequirementBuilder {
        self.value["profile"] = json!(val);
        return self;
    }

    pub fn sort<'a>(
        &'a mut self,
        val: Vec<DataRequirement_Sort>,
    ) -> &'a mut DataRequirementBuilder {
        self.value["sort"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subject_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut DataRequirementBuilder {
        self.value["subjectCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn subject_reference<'a>(&'a mut self, val: Reference) -> &'a mut DataRequirementBuilder {
        self.value["subjectReference"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut DataRequirementBuilder {
        self.value["type"] = json!(val);
        return self;
    }
}

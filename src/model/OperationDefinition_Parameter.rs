#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::OperationDefinition_Binding::OperationDefinition_Binding;
use crate::model::OperationDefinition_ReferencedFrom::OperationDefinition_ReferencedFrom;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).

#[derive(Debug)]
pub struct OperationDefinition_Parameter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl OperationDefinition_Parameter<'_> {
    pub fn new(value: &Value) -> OperationDefinition_Parameter {
        OperationDefinition_Parameter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for searchType
    pub fn _search_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_searchType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Binds to a value set if this parameter is coded (code, Coding, CodeableConcept).
    pub fn binding(&self) -> Option<OperationDefinition_Binding> {
        if let Some(val) = self.value.get("binding") {
            return Some(OperationDefinition_Binding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the meaning or use of this parameter.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
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

    /// The maximum number of times this element is permitted to appear in the request
    /// or response.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// The minimum number of times this parameter SHALL appear in the request or
    /// response.
    pub fn min(&self) -> Option<i64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_i64().unwrap());
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

    /// The name of used to identify the parameter.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The parts of a nested Parameter.
    pub fn part(&self) -> Option<Vec<OperationDefinition_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("part") {
            return Some(
                val.into_iter()
                    .map(|e| OperationDefinition_Parameter {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies other resource parameters within the operation invocation that are
    /// expected to resolve to this resource.
    pub fn referenced_from(&self) -> Option<Vec<OperationDefinition_ReferencedFrom>> {
        if let Some(Value::Array(val)) = self.value.get("referencedFrom") {
            return Some(
                val.into_iter()
                    .map(|e| OperationDefinition_ReferencedFrom {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// How the parameter is understood as a search parameter. This is only used if the
    /// parameter type is 'string'.
    pub fn search_type(&self) -> Option<OperationDefinition_ParameterSearchType> {
        if let Some(Value::String(val)) = self.value.get("searchType") {
            return Some(OperationDefinition_ParameterSearchType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Used when the type is "Reference" or "canonical", and identifies a profile
    /// structure or implementation Guide that applies to the target of the reference
    /// this parameter refers to. If any profiles are specified, then the content must
    /// conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition or
    /// Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the target resource SHALL conform to at least one profile defined in
    /// the implementation guide.
    pub fn target_profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("targetProfile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type for this parameter.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// Whether this is an input or an output parameter.
    pub fn fhir_use(&self) -> Option<OperationDefinition_ParameterUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(OperationDefinition_ParameterUse::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._min() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._search_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.binding() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.part() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.referenced_from() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.search_type() {}
        if let Some(_val) = self.target_profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct OperationDefinition_ParameterBuilder {
    pub(crate) value: Value,
}

impl OperationDefinition_ParameterBuilder {
    pub fn build(&self) -> OperationDefinition_Parameter {
        OperationDefinition_Parameter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: OperationDefinition_Parameter) -> OperationDefinition_ParameterBuilder {
        OperationDefinition_ParameterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> OperationDefinition_ParameterBuilder {
        let mut __value: Value = json!({});
        return OperationDefinition_ParameterBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _search_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_searchType"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn binding<'a>(
        &'a mut self,
        val: OperationDefinition_Binding,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["binding"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: i64) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn part<'a>(
        &'a mut self,
        val: Vec<OperationDefinition_Parameter>,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["part"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn referenced_from<'a>(
        &'a mut self,
        val: Vec<OperationDefinition_ReferencedFrom>,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["referencedFrom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn search_type<'a>(
        &'a mut self,
        val: OperationDefinition_ParameterSearchType,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["searchType"] = json!(val.to_string());
        return self;
    }

    pub fn target_profile<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["targetProfile"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn fhir_use<'a>(
        &'a mut self,
        val: OperationDefinition_ParameterUse,
    ) -> &'a mut OperationDefinition_ParameterBuilder {
        self.value["use"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum OperationDefinition_ParameterSearchType {
    Number,
    Date,
    String,
    Token,
    Reference,
    Composite,
    Quantity,
    Uri,
    Special,
}

impl OperationDefinition_ParameterSearchType {
    pub fn from_string(string: &str) -> Option<OperationDefinition_ParameterSearchType> {
        match string {
            "number" => Some(OperationDefinition_ParameterSearchType::Number),
            "date" => Some(OperationDefinition_ParameterSearchType::Date),
            "string" => Some(OperationDefinition_ParameterSearchType::String),
            "token" => Some(OperationDefinition_ParameterSearchType::Token),
            "reference" => Some(OperationDefinition_ParameterSearchType::Reference),
            "composite" => Some(OperationDefinition_ParameterSearchType::Composite),
            "quantity" => Some(OperationDefinition_ParameterSearchType::Quantity),
            "uri" => Some(OperationDefinition_ParameterSearchType::Uri),
            "special" => Some(OperationDefinition_ParameterSearchType::Special),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperationDefinition_ParameterSearchType::Number => "number".to_string(),
            OperationDefinition_ParameterSearchType::Date => "date".to_string(),
            OperationDefinition_ParameterSearchType::String => "string".to_string(),
            OperationDefinition_ParameterSearchType::Token => "token".to_string(),
            OperationDefinition_ParameterSearchType::Reference => "reference".to_string(),
            OperationDefinition_ParameterSearchType::Composite => "composite".to_string(),
            OperationDefinition_ParameterSearchType::Quantity => "quantity".to_string(),
            OperationDefinition_ParameterSearchType::Uri => "uri".to_string(),
            OperationDefinition_ParameterSearchType::Special => "special".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum OperationDefinition_ParameterUse {
    In,
    Out,
}

impl OperationDefinition_ParameterUse {
    pub fn from_string(string: &str) -> Option<OperationDefinition_ParameterUse> {
        match string {
            "in" => Some(OperationDefinition_ParameterUse::In),
            "out" => Some(OperationDefinition_ParameterUse::Out),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperationDefinition_ParameterUse::In => "in".to_string(),
            OperationDefinition_ParameterUse::Out => "out".to_string(),
        }
    }
}

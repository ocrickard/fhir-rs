#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::SearchParameter_Component::SearchParameter_Component;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A search parameter that defines a named search item that can be used to
/// search/filter on a resource.

#[derive(Debug)]
pub struct SearchParameter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SearchParameter<'_> {
    pub fn new(value: &Value) -> SearchParameter {
        SearchParameter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for base
    pub fn _base(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_base") {
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

    /// Extensions for chain
    pub fn _chain(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_chain") {
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

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for comparator
    pub fn _comparator(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_comparator") {
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

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
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

    /// Extensions for modifier
    pub fn _modifier(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_modifier") {
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

    /// Extensions for multipleAnd
    pub fn _multiple_and(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleAnd") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for multipleOr
    pub fn _multiple_or(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleOr") {
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
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

    /// Extensions for target
    pub fn _target(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_target") {
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

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for xpath
    pub fn _xpath(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_xpath") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for xpathUsage
    pub fn _xpath_usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_xpathUsage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The base resource type(s) that this search parameter can be used against.
    pub fn base(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("base") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contains the names of any search parameters which may be chained to the
    /// containing search parameter. Chained parameters may be added to search
    /// parameters of type reference and specify that resources will only be returned if
    /// they contain a reference to a resource which matches the chained parameter
    /// value. Values for this field should be drawn from SearchParameter.code for a
    /// parameter on the target resource type.
    pub fn chain(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("chain") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The code used in the URL or the parameter name in a parameters resource for this
    /// search parameter.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Used to define the parts of a composite search parameter.
    pub fn component(&self) -> Option<Vec<SearchParameter_Component>> {
        if let Some(Value::Array(val)) = self.value.get("component") {
            return Some(
                val.into_iter()
                    .map(|e| SearchParameter_Component {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
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

    /// The date  (and optionally time) when the search parameter was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the search parameter changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Where this search parameter is originally defined. If a derivedFrom is provided,
    /// then the details in the search parameter must be consistent with the definition
    /// from which it is defined. i.e. the parameter should have the same meaning, and
    /// (usually) the functionality should be a proper subset of the underlying search
    /// parameter.
    pub fn derived_from(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("derivedFrom") {
            return Some(string);
        }
        return None;
    }

    /// And how it used.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A Boolean value to indicate that this search parameter is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A FHIRPath expression that returns a set of elements for the search parameter.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
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

    /// A legal or geographic region in which the search parameter is intended to be
    /// used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
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

    /// Whether multiple parameters are allowed - e.g. more than one parameter with the
    /// same name. The search matches if all the parameters match.
    pub fn multiple_and(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleAnd") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether multiple values are allowed for each time the parameter exists. Values
    /// are separated by commas, and the parameter matches if any of the values match.
    pub fn multiple_or(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleOr") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A natural language name identifying the search parameter. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the search parameter.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this search parameter is needed and why it has been designed
    /// as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// The status of this search parameter. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<SearchParameterStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(SearchParameterStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Types of resource (if a resource is referenced).
    pub fn target(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// The type of value that a search parameter may contain, and how the content is
    /// interpreted.
    pub fn fhir_type(&self) -> Option<SearchParameterType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(SearchParameterType::from_string(&val).unwrap());
        }
        return None;
    }

    /// An absolute URI that is used to identify this search parameter when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this search parameter is
    /// (or will be) published. This URL can be the target of a canonical reference. It
    /// SHALL remain the same when the search parameter is stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate search parameter
    /// instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the search parameter
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the search parameter author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// An XPath expression that returns a set of elements for the search parameter.
    pub fn xpath(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("xpath") {
            return Some(string);
        }
        return None;
    }

    /// How the search parameter relates to the set of elements returned by evaluating
    /// the xpath query.
    pub fn xpath_usage(&self) -> Option<SearchParameterXpathUsage> {
        if let Some(Value::String(val)) = self.value.get("xpathUsage") {
            return Some(SearchParameterXpathUsage::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._base() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._chain() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._comparator() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._experimental() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expression() {
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
        if let Some(_val) = self._modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._multiple_and() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._multiple_or() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._purpose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._xpath() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._xpath_usage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.base() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.chain() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.component() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.derived_from() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.multiple_and() {}
        if let Some(_val) = self.multiple_or() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.xpath() {}
        if let Some(_val) = self.xpath_usage() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SearchParameterBuilder {
    pub(crate) value: Value,
}

impl SearchParameterBuilder {
    pub fn build(&self) -> SearchParameter {
        SearchParameter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SearchParameter) -> SearchParameterBuilder {
        SearchParameterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SearchParameterBuilder {
        let mut __value: Value = json!({});
        return SearchParameterBuilder { value: __value };
    }

    pub fn _base<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SearchParameterBuilder {
        self.value["_base"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _chain<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SearchParameterBuilder {
        self.value["_chain"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _comparator<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SearchParameterBuilder {
        self.value["_comparator"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _expression<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _modifier<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SearchParameterBuilder {
        self.value["_modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _multiple_and<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_multipleAnd"] = json!(val.value);
        return self;
    }

    pub fn _multiple_or<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_multipleOr"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _target<'a>(&'a mut self, val: Vec<Element>) -> &'a mut SearchParameterBuilder {
        self.value["_target"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn _xpath<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_xpath"] = json!(val.value);
        return self;
    }

    pub fn _xpath_usage<'a>(&'a mut self, val: Element) -> &'a mut SearchParameterBuilder {
        self.value["_xpathUsage"] = json!(val.value);
        return self;
    }

    pub fn base<'a>(&'a mut self, val: Vec<&str>) -> &'a mut SearchParameterBuilder {
        self.value["base"] = json!(val);
        return self;
    }

    pub fn chain<'a>(&'a mut self, val: Vec<&str>) -> &'a mut SearchParameterBuilder {
        self.value["chain"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn component<'a>(
        &'a mut self,
        val: Vec<SearchParameter_Component>,
    ) -> &'a mut SearchParameterBuilder {
        self.value["component"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut SearchParameterBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut SearchParameterBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn derived_from<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["derivedFrom"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut SearchParameterBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn expression<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SearchParameterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SearchParameterBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SearchParameterBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SearchParameterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn multiple_and<'a>(&'a mut self, val: bool) -> &'a mut SearchParameterBuilder {
        self.value["multipleAnd"] = json!(val);
        return self;
    }

    pub fn multiple_or<'a>(&'a mut self, val: bool) -> &'a mut SearchParameterBuilder {
        self.value["multipleOr"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: SearchParameterStatus) -> &'a mut SearchParameterBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn target<'a>(&'a mut self, val: Vec<&str>) -> &'a mut SearchParameterBuilder {
        self.value["target"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SearchParameterBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: SearchParameterType) -> &'a mut SearchParameterBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(&'a mut self, val: Vec<UsageContext>) -> &'a mut SearchParameterBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["version"] = json!(val);
        return self;
    }

    pub fn xpath<'a>(&'a mut self, val: &str) -> &'a mut SearchParameterBuilder {
        self.value["xpath"] = json!(val);
        return self;
    }

    pub fn xpath_usage<'a>(
        &'a mut self,
        val: SearchParameterXpathUsage,
    ) -> &'a mut SearchParameterBuilder {
        self.value["xpathUsage"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum SearchParameterStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl SearchParameterStatus {
    pub fn from_string(string: &str) -> Option<SearchParameterStatus> {
        match string {
            "draft" => Some(SearchParameterStatus::Draft),
            "active" => Some(SearchParameterStatus::Active),
            "retired" => Some(SearchParameterStatus::Retired),
            "unknown" => Some(SearchParameterStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SearchParameterStatus::Draft => "draft".to_string(),
            SearchParameterStatus::Active => "active".to_string(),
            SearchParameterStatus::Retired => "retired".to_string(),
            SearchParameterStatus::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum SearchParameterType {
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

impl SearchParameterType {
    pub fn from_string(string: &str) -> Option<SearchParameterType> {
        match string {
            "number" => Some(SearchParameterType::Number),
            "date" => Some(SearchParameterType::Date),
            "string" => Some(SearchParameterType::String),
            "token" => Some(SearchParameterType::Token),
            "reference" => Some(SearchParameterType::Reference),
            "composite" => Some(SearchParameterType::Composite),
            "quantity" => Some(SearchParameterType::Quantity),
            "uri" => Some(SearchParameterType::Uri),
            "special" => Some(SearchParameterType::Special),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SearchParameterType::Number => "number".to_string(),
            SearchParameterType::Date => "date".to_string(),
            SearchParameterType::String => "string".to_string(),
            SearchParameterType::Token => "token".to_string(),
            SearchParameterType::Reference => "reference".to_string(),
            SearchParameterType::Composite => "composite".to_string(),
            SearchParameterType::Quantity => "quantity".to_string(),
            SearchParameterType::Uri => "uri".to_string(),
            SearchParameterType::Special => "special".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum SearchParameterXpathUsage {
    Normal,
    Phonetic,
    Nearby,
    Distance,
    Other,
}

impl SearchParameterXpathUsage {
    pub fn from_string(string: &str) -> Option<SearchParameterXpathUsage> {
        match string {
            "normal" => Some(SearchParameterXpathUsage::Normal),
            "phonetic" => Some(SearchParameterXpathUsage::Phonetic),
            "nearby" => Some(SearchParameterXpathUsage::Nearby),
            "distance" => Some(SearchParameterXpathUsage::Distance),
            "other" => Some(SearchParameterXpathUsage::Other),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SearchParameterXpathUsage::Normal => "normal".to_string(),
            SearchParameterXpathUsage::Phonetic => "phonetic".to_string(),
            SearchParameterXpathUsage::Nearby => "nearby".to_string(),
            SearchParameterXpathUsage::Distance => "distance".to_string(),
            SearchParameterXpathUsage::Other => "other".to_string(),
        }
    }
}

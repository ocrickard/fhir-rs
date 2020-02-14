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
use serde_json::value::Value;

/// A search parameter that defines a named search item that can be used to
/// search/filter on a resource.

#[derive(Debug)]
pub struct SearchParameter<'a> {
    pub value: &'a Value,
}

impl SearchParameter<'_> {
    /// Extensions for xpathUsage
    pub fn _xpath_usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_xpathUsage") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for base
    pub fn _base(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_base") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element { value: val });
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
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

    /// Whether multiple parameters are allowed - e.g. more than one parameter with the
    /// same name. The search matches if all the parameters match.
    pub fn multiple_and(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleAnd") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for xpath
    pub fn _xpath(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_xpath") {
            return Some(Element { value: val });
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// The type of value that a search parameter may contain, and how the content is
    /// interpreted.
    pub fn fhir_type(&self) -> Option<SearchParameterType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(SearchParameterType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
            return Some(Element { value: val });
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

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for multipleOr
    pub fn _multiple_or(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleOr") {
            return Some(Element { value: val });
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

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for modifier
    pub fn _modifier(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_modifier") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for multipleAnd
    pub fn _multiple_and(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleAnd") {
            return Some(Element { value: val });
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
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A legal or geographic region in which the search parameter is intended to be
    /// used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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
                    .map(|e| SearchParameter_Component { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate search parameter
    /// instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for target
    pub fn _target(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_target") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for comparator
    pub fn _comparator(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_comparator") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
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

    /// The name of the organization or individual that published the search parameter.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._xpath_usage() {
            _val.validate();
        }
        if let Some(_val) = self._base() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.xpath_usage() {}
        if let Some(_val) = self.derived_from() {}
        if let Some(_val) = self.base() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.multiple_and() {}
        if let Some(_val) = self._xpath() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._expression() {
            _val.validate();
        }
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self._multiple_or() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.xpath() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self._modifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.chain() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self._multiple_and() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._chain() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.multiple_or() {}
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.component() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self._target() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._comparator() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.publisher() {}
        return true;
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
}

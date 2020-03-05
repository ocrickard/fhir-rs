#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::TerminologyCapabilities_Closure::TerminologyCapabilities_Closure;
use crate::model::TerminologyCapabilities_CodeSystem::TerminologyCapabilities_CodeSystem;
use crate::model::TerminologyCapabilities_Expansion::TerminologyCapabilities_Expansion;
use crate::model::TerminologyCapabilities_Implementation::TerminologyCapabilities_Implementation;
use crate::model::TerminologyCapabilities_Software::TerminologyCapabilities_Software;
use crate::model::TerminologyCapabilities_Translation::TerminologyCapabilities_Translation;
use crate::model::TerminologyCapabilities_ValidateCode::TerminologyCapabilities_ValidateCode;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TerminologyCapabilities<'_> {
    pub fn new(value: &Value) -> TerminologyCapabilities {
        TerminologyCapabilities {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for codeSearch
    pub fn _code_search(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_codeSearch") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
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

    /// Extensions for lockedDate
    pub fn _locked_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lockedDate") {
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
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

    /// Whether the $closure operation is supported.
    pub fn closure(&self) -> Option<TerminologyCapabilities_Closure> {
        if let Some(val) = self.value.get("closure") {
            return Some(TerminologyCapabilities_Closure {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The degree to which the server supports the code search parameter on ValueSet,
    /// if it is supported.
    pub fn code_search(&self) -> Option<TerminologyCapabilitiesCodeSearch> {
        if let Some(Value::String(val)) = self.value.get("codeSearch") {
            return Some(TerminologyCapabilitiesCodeSearch::from_string(&val).unwrap());
        }
        return None;
    }

    /// Identifies a code system that is supported by the server. If there is a no code
    /// system URL, then this declares the general assumptions a client can make about
    /// support for any CodeSystem resource.
    pub fn code_system(&self) -> Option<Vec<TerminologyCapabilities_CodeSystem>> {
        if let Some(Value::Array(val)) = self.value.get("codeSystem") {
            return Some(
                val.into_iter()
                    .map(|e| TerminologyCapabilities_CodeSystem {
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

    /// A copyright statement relating to the terminology capabilities and/or its
    /// contents. Copyright statements are generally legal restrictions on the use and
    /// publishing of the terminology capabilities.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the terminology capabilities was published.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the terminology capabilities changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the terminology capabilities from a
    /// consumer's perspective. Typically, this is used when the capability statement
    /// describes a desired rather than an actual solution, for example as a formal
    /// expression of requirements as part of an RFP.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Information about the [ValueSet/$expand](valueset-operation-expand.html)
    /// operation.
    pub fn expansion(&self) -> Option<TerminologyCapabilities_Expansion> {
        if let Some(val) = self.value.get("expansion") {
            return Some(TerminologyCapabilities_Expansion {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A Boolean value to indicate that this terminology capabilities is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
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

    /// Identifies a specific implementation instance that is described by the
    /// terminology capability statement - i.e. a particular installation, rather than
    /// the capabilities of a software program.
    pub fn implementation(&self) -> Option<TerminologyCapabilities_Implementation> {
        if let Some(val) = self.value.get("implementation") {
            return Some(TerminologyCapabilities_Implementation {
                value: Cow::Borrowed(val),
            });
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

    /// A legal or geographic region in which the terminology capabilities is intended
    /// to be used.
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

    /// The way that this statement is intended to be used, to describe an actual
    /// running instance of software, a particular product (kind, not instance of
    /// software) or a class of implementation (e.g. a desired purchase).
    pub fn kind(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("kind") {
            return Some(string);
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

    /// Whether the server supports lockedDate.
    pub fn locked_date(&self) -> Option<bool> {
        if let Some(val) = self.value.get("lockedDate") {
            return Some(val.as_bool().unwrap());
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

    /// A natural language name identifying the terminology capabilities. This name
    /// should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the terminology
    /// capabilities.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this terminology capabilities is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Software that is covered by this terminology capability statement.  It is used
    /// when the statement describes the capabilities of a particular software version,
    /// independent of an installation.
    pub fn software(&self) -> Option<TerminologyCapabilities_Software> {
        if let Some(val) = self.value.get("software") {
            return Some(TerminologyCapabilities_Software {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of this terminology capabilities. Enables tracking the life-cycle of
    /// the content.
    pub fn status(&self) -> Option<TerminologyCapabilitiesStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(TerminologyCapabilitiesStatus::from_string(&val).unwrap());
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

    /// A short, descriptive, user-friendly title for the terminology capabilities.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Information about the [ConceptMap/$translate](conceptmap-operation-
    /// translate.html) operation.
    pub fn translation(&self) -> Option<TerminologyCapabilities_Translation> {
        if let Some(val) = self.value.get("translation") {
            return Some(TerminologyCapabilities_Translation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An absolute URI that is used to identify this terminology capabilities when it
    /// is referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this terminology
    /// capabilities is (or will be) published. This URL can be the target of a
    /// canonical reference. It SHALL remain the same when the terminology capabilities
    /// is stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate terminology
    /// capabilities instances.
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

    /// Information about the [ValueSet/$validate-code](valueset-operation-validate-
    /// code.html) operation.
    pub fn validate_code(&self) -> Option<TerminologyCapabilities_ValidateCode> {
        if let Some(val) = self.value.get("validateCode") {
            return Some(TerminologyCapabilities_ValidateCode {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The identifier that is used to identify this version of the terminology
    /// capabilities when it is referenced in a specification, model, design or
    /// instance. This is an arbitrary value managed by the terminology capabilities
    /// author and is not expected to be globally unique. For example, it might be a
    /// timestamp (e.g. yyyymmdd) if a managed version is not available. There is also
    /// no expectation that versions can be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code_search() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
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
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._kind() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._locked_date() {
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
        if let Some(_val) = self._title() {
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
        if let Some(_val) = self.closure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code_search() {}
        if let Some(_val) = self.code_system() {
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
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.expansion() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implementation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.locked_date() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.software() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.translation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.validate_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TerminologyCapabilitiesBuilder {
    pub(crate) value: Value,
}

impl TerminologyCapabilitiesBuilder {
    pub fn build(&self) -> TerminologyCapabilities {
        TerminologyCapabilities {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TerminologyCapabilities) -> TerminologyCapabilitiesBuilder {
        TerminologyCapabilitiesBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TerminologyCapabilitiesBuilder {
        let mut __value: Value = json!({});
        return TerminologyCapabilitiesBuilder { value: __value };
    }

    pub fn _code_search<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_codeSearch"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _locked_date<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_lockedDate"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn closure<'a>(
        &'a mut self,
        val: TerminologyCapabilities_Closure,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["closure"] = json!(val.value);
        return self;
    }

    pub fn code_search<'a>(
        &'a mut self,
        val: TerminologyCapabilitiesCodeSearch,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["codeSearch"] = json!(val.to_string());
        return self;
    }

    pub fn code_system<'a>(
        &'a mut self,
        val: Vec<TerminologyCapabilities_CodeSystem>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["codeSystem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn expansion<'a>(
        &'a mut self,
        val: TerminologyCapabilities_Expansion,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["expansion"] = json!(val.value);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implementation<'a>(
        &'a mut self,
        val: TerminologyCapabilities_Implementation,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["implementation"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn kind<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["kind"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn locked_date<'a>(&'a mut self, val: bool) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["lockedDate"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn software<'a>(
        &'a mut self,
        val: TerminologyCapabilities_Software,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["software"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: TerminologyCapabilitiesStatus,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn translation<'a>(
        &'a mut self,
        val: TerminologyCapabilities_Translation,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["translation"] = json!(val.value);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn validate_code<'a>(
        &'a mut self,
        val: TerminologyCapabilities_ValidateCode,
    ) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["validateCode"] = json!(val.value);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilitiesBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum TerminologyCapabilitiesCodeSearch {
    Explicit,
    All,
}

impl TerminologyCapabilitiesCodeSearch {
    pub fn from_string(string: &str) -> Option<TerminologyCapabilitiesCodeSearch> {
        match string {
            "explicit" => Some(TerminologyCapabilitiesCodeSearch::Explicit),
            "all" => Some(TerminologyCapabilitiesCodeSearch::All),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TerminologyCapabilitiesCodeSearch::Explicit => "explicit".to_string(),
            TerminologyCapabilitiesCodeSearch::All => "all".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TerminologyCapabilitiesStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl TerminologyCapabilitiesStatus {
    pub fn from_string(string: &str) -> Option<TerminologyCapabilitiesStatus> {
        match string {
            "draft" => Some(TerminologyCapabilitiesStatus::Draft),
            "active" => Some(TerminologyCapabilitiesStatus::Active),
            "retired" => Some(TerminologyCapabilitiesStatus::Retired),
            "unknown" => Some(TerminologyCapabilitiesStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TerminologyCapabilitiesStatus::Draft => "draft".to_string(),
            TerminologyCapabilitiesStatus::Active => "active".to_string(),
            TerminologyCapabilitiesStatus::Retired => "retired".to_string(),
            TerminologyCapabilitiesStatus::Unknown => "unknown".to_string(),
        }
    }
}

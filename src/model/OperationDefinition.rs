#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::OperationDefinition_Overload::OperationDefinition_Overload;
use crate::model::OperationDefinition_Parameter::OperationDefinition_Parameter;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).

#[derive(Debug)]
pub struct OperationDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl OperationDefinition<'_> {
    pub fn new(value: &Value) -> OperationDefinition {
        OperationDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for affectsState
    pub fn _affects_state(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_affectsState") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
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

    /// Extensions for instance
    pub fn _instance(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instance") {
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

    /// Extensions for resource
    pub fn _resource(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_resource") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
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

    /// Whether the operation affects state. Side effects such as producing audit trail
    /// entries do not count as 'affecting  state'.
    pub fn affects_state(&self) -> Option<bool> {
        if let Some(val) = self.value.get("affectsState") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates that this operation definition is a constraining profile on the base.
    pub fn base(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("base") {
            return Some(string);
        }
        return None;
    }

    /// The name used to invoke the operation.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Additional information about how to use this operation or named query.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
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

    /// The date  (and optionally time) when the operation definition was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the operation definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the operation definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A Boolean value to indicate that this operation definition is authored for
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

    /// Additional validation information for the in parameters - a single profile that
    /// covers all the parameters. The profile is a constraint on the parameters
    /// resource as a whole.
    pub fn input_profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("inputProfile") {
            return Some(string);
        }
        return None;
    }

    /// Indicates whether this operation can be invoked on a particular instance of one
    /// of the given types.
    pub fn instance(&self) -> Option<bool> {
        if let Some(val) = self.value.get("instance") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A legal or geographic region in which the operation definition is intended to be
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

    /// Whether this is an operation or a named query.
    pub fn kind(&self) -> Option<OperationDefinitionKind> {
        if let Some(Value::String(val)) = self.value.get("kind") {
            return Some(OperationDefinitionKind::from_string(&val).unwrap());
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

    /// A natural language name identifying the operation definition. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Additional validation information for the out parameters - a single profile that
    /// covers all the parameters. The profile is a constraint on the parameters
    /// resource.
    pub fn output_profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outputProfile") {
            return Some(string);
        }
        return None;
    }

    /// Defines an appropriate combination of parameters to use when invoking this
    /// operation, to help code generators when generating overloaded parameter sets for
    /// this operation.
    pub fn overload(&self) -> Option<Vec<OperationDefinition_Overload>> {
        if let Some(Value::Array(val)) = self.value.get("overload") {
            return Some(
                val.into_iter()
                    .map(|e| OperationDefinition_Overload {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The parameters for the operation/query.
    pub fn parameter(&self) -> Option<Vec<OperationDefinition_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
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

    /// The name of the organization or individual that published the operation
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this operation definition is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// The types on which this operation can be executed.
    pub fn resource(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("resource") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of this operation definition. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<OperationDefinitionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(OperationDefinitionStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Indicates whether this operation or named query can be invoked at the system
    /// level (e.g. without needing to choose a resource type for the context).
    pub fn system(&self) -> Option<bool> {
        if let Some(val) = self.value.get("system") {
            return Some(val.as_bool().unwrap());
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

    /// A short, descriptive, user-friendly title for the operation definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Indicates whether this operation or named query can be invoked at the resource
    /// type level for any given resource type level (e.g. without needing to choose a
    /// specific resource id for the context).
    pub fn fhir_type(&self) -> Option<bool> {
        if let Some(val) = self.value.get("type") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An absolute URI that is used to identify this operation definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this operation definition
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the operation definition is stored on different
    /// servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate operation
    /// definition instances.
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

    /// The identifier that is used to identify this version of the operation definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the operation definition author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._affects_state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._comment() {
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
        if let Some(_val) = self._instance() {
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
        if let Some(_val) = self._resource() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
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
        if let Some(_val) = self.affects_state() {}
        if let Some(_val) = self.base() {}
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.comment() {}
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
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.input_profile() {}
        if let Some(_val) = self.instance() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.kind() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.output_profile() {}
        if let Some(_val) = self.overload() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.parameter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.resource() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct OperationDefinitionBuilder {
    pub(crate) value: Value,
}

impl OperationDefinitionBuilder {
    pub fn build(&self) -> OperationDefinition {
        OperationDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: OperationDefinition) -> OperationDefinitionBuilder {
        OperationDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> OperationDefinitionBuilder {
        let mut __value: Value = json!({});
        return OperationDefinitionBuilder { value: __value };
    }

    pub fn _affects_state<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_affectsState"] = json!(val.value);
        return self;
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instance<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_instance"] = json!(val.value);
        return self;
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _resource<'a>(&'a mut self, val: Vec<Element>) -> &'a mut OperationDefinitionBuilder {
        self.value["_resource"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _system<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_system"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut OperationDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn affects_state<'a>(&'a mut self, val: bool) -> &'a mut OperationDefinitionBuilder {
        self.value["affectsState"] = json!(val);
        return self;
    }

    pub fn base<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["base"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut OperationDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut OperationDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn input_profile<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["inputProfile"] = json!(val);
        return self;
    }

    pub fn instance<'a>(&'a mut self, val: bool) -> &'a mut OperationDefinitionBuilder {
        self.value["instance"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn kind<'a>(
        &'a mut self,
        val: OperationDefinitionKind,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["kind"] = json!(val.to_string());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut OperationDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn output_profile<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["outputProfile"] = json!(val);
        return self;
    }

    pub fn overload<'a>(
        &'a mut self,
        val: Vec<OperationDefinition_Overload>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["overload"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parameter<'a>(
        &'a mut self,
        val: Vec<OperationDefinition_Parameter>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["parameter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: Vec<&str>) -> &'a mut OperationDefinitionBuilder {
        self.value["resource"] = json!(val);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: OperationDefinitionStatus,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn system<'a>(&'a mut self, val: bool) -> &'a mut OperationDefinitionBuilder {
        self.value["system"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut OperationDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: bool) -> &'a mut OperationDefinitionBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut OperationDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut OperationDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum OperationDefinitionKind {
    Operation,
    Query,
}

impl OperationDefinitionKind {
    pub fn from_string(string: &str) -> Option<OperationDefinitionKind> {
        match string {
            "operation" => Some(OperationDefinitionKind::Operation),
            "query" => Some(OperationDefinitionKind::Query),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperationDefinitionKind::Operation => "operation".to_string(),
            OperationDefinitionKind::Query => "query".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum OperationDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl OperationDefinitionStatus {
    pub fn from_string(string: &str) -> Option<OperationDefinitionStatus> {
        match string {
            "draft" => Some(OperationDefinitionStatus::Draft),
            "active" => Some(OperationDefinitionStatus::Active),
            "retired" => Some(OperationDefinitionStatus::Retired),
            "unknown" => Some(OperationDefinitionStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperationDefinitionStatus::Draft => "draft".to_string(),
            OperationDefinitionStatus::Active => "active".to_string(),
            OperationDefinitionStatus::Retired => "retired".to_string(),
            OperationDefinitionStatus::Unknown => "unknown".to_string(),
        }
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::CapabilityStatement_Document::CapabilityStatement_Document;
use crate::model::CapabilityStatement_Implementation::CapabilityStatement_Implementation;
use crate::model::CapabilityStatement_Messaging::CapabilityStatement_Messaging;
use crate::model::CapabilityStatement_Rest::CapabilityStatement_Rest;
use crate::model::CapabilityStatement_Software::CapabilityStatement_Software;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement<'_> {
    pub fn new(value: &Value) -> CapabilityStatement {
        CapabilityStatement {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for fhirVersion
    pub fn _fhir_version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fhirVersion") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for format
    pub fn _format(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_format") {
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for patchFormat
    pub fn _patch_format(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_patchFormat") {
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

    /// A copyright statement relating to the capability statement and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the capability statement.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the capability statement was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the capability statement changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the capability statement from a
    /// consumer's perspective. Typically, this is used when the capability statement
    /// describes a desired rather than an actual solution, for example as a formal
    /// expression of requirements as part of an RFP.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A document definition.
    pub fn document(&self) -> Option<Vec<CapabilityStatement_Document>> {
        if let Some(Value::Array(val)) = self.value.get("document") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Document {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A Boolean value to indicate that this capability statement is authored for
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

    /// The version of the FHIR specification that this CapabilityStatement describes
    /// (which SHALL be the same as the FHIR version of the CapabilityStatement itself).
    /// There is no default value.
    pub fn fhir_version(&self) -> Option<CapabilityStatementFhirVersion> {
        if let Some(Value::String(val)) = self.value.get("fhirVersion") {
            return Some(CapabilityStatementFhirVersion::from_string(&val).unwrap());
        }
        return None;
    }

    /// A list of the formats supported by this implementation using their content
    /// types.
    pub fn format(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("format") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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
    /// capability statement - i.e. a particular installation, rather than the
    /// capabilities of a software program.
    pub fn implementation(&self) -> Option<CapabilityStatement_Implementation> {
        if let Some(val) = self.value.get("implementation") {
            return Some(CapabilityStatement_Implementation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A list of implementation guides that the server does (or should) support in
    /// their entirety.
    pub fn implementation_guide(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("implementationGuide") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// Reference to a canonical URL of another CapabilityStatement that this software
    /// adds to. The capability statement automatically includes everything in the other
    /// statement, and it is not duplicated, though the server may repeat the same
    /// resources, interactions and operations to add additional details to them.
    pub fn imports(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("imports") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Reference to a canonical URL of another CapabilityStatement that this software
    /// implements. This capability statement is a published API description that
    /// corresponds to a business service. The server may actually implement a subset of
    /// the capability statement it claims to implement, so the capability statement
    /// must specify the full capability details.
    pub fn instantiates(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiates") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A legal or geographic region in which the capability statement is intended to be
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

    /// The way that this statement is intended to be used, to describe an actual
    /// running instance of software, a particular product (kind, not instance of
    /// software) or a class of implementation (e.g. a desired purchase).
    pub fn kind(&self) -> Option<CapabilityStatementKind> {
        if let Some(Value::String(val)) = self.value.get("kind") {
            return Some(CapabilityStatementKind::from_string(&val).unwrap());
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

    /// A description of the messaging capabilities of the solution.
    pub fn messaging(&self) -> Option<Vec<CapabilityStatement_Messaging>> {
        if let Some(Value::Array(val)) = self.value.get("messaging") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Messaging {
                        value: Cow::Borrowed(e),
                    })
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

    /// A natural language name identifying the capability statement. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// A list of the patch formats supported by this implementation using their content
    /// types.
    pub fn patch_format(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("patchFormat") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the organization or individual that published the capability
    /// statement.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this capability statement is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// A definition of the restful capabilities of the solution, if any.
    pub fn rest(&self) -> Option<Vec<CapabilityStatement_Rest>> {
        if let Some(Value::Array(val)) = self.value.get("rest") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Rest {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Software that is covered by this capability statement.  It is used when the
    /// capability statement describes the capabilities of a particular software
    /// version, independent of an installation.
    pub fn software(&self) -> Option<CapabilityStatement_Software> {
        if let Some(val) = self.value.get("software") {
            return Some(CapabilityStatement_Software {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of this capability statement. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<CapabilityStatementStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(CapabilityStatementStatus::from_string(&val).unwrap());
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

    /// A short, descriptive, user-friendly title for the capability statement.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that is used to identify this capability statement when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this capability statement
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the capability statement is stored on different
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
    /// be used to assist with indexing and searching for appropriate capability
    /// statement instances.
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

    /// The identifier that is used to identify this version of the capability statement
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the capability statement author and is not expected
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
        if let Some(_val) = self._fhir_version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._format() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._patch_format() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.document() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_version() {}
        if let Some(_val) = self.format() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implementation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implementation_guide() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.imports() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.messaging() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.patch_format() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.rest() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
pub struct CapabilityStatementBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatementBuilder {
    pub fn build(&self) -> CapabilityStatement {
        CapabilityStatement {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement) -> CapabilityStatementBuilder {
        CapabilityStatementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CapabilityStatementBuilder {
        let mut __value: Value = json!({});
        return CapabilityStatementBuilder { value: __value };
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _fhir_version<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_fhirVersion"] = json!(val.value);
        return self;
    }

    pub fn _format<'a>(&'a mut self, val: Vec<Element>) -> &'a mut CapabilityStatementBuilder {
        self.value["_format"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _patch_format<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["_patchFormat"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatementBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn document<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement_Document>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["document"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut CapabilityStatementBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CapabilityStatementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_version<'a>(
        &'a mut self,
        val: CapabilityStatementFhirVersion,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["fhirVersion"] = json!(val.to_string());
        return self;
    }

    pub fn format<'a>(&'a mut self, val: Vec<&str>) -> &'a mut CapabilityStatementBuilder {
        self.value["format"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implementation<'a>(
        &'a mut self,
        val: CapabilityStatement_Implementation,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["implementation"] = json!(val.value);
        return self;
    }

    pub fn implementation_guide<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["implementationGuide"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn imports<'a>(&'a mut self, val: Vec<&str>) -> &'a mut CapabilityStatementBuilder {
        self.value["imports"] = json!(val);
        return self;
    }

    pub fn instantiates<'a>(&'a mut self, val: Vec<&str>) -> &'a mut CapabilityStatementBuilder {
        self.value["instantiates"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn kind<'a>(
        &'a mut self,
        val: CapabilityStatementKind,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["kind"] = json!(val.to_string());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn messaging<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement_Messaging>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["messaging"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut CapabilityStatementBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn patch_format<'a>(&'a mut self, val: Vec<&str>) -> &'a mut CapabilityStatementBuilder {
        self.value["patchFormat"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn rest<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement_Rest>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["rest"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn software<'a>(
        &'a mut self,
        val: CapabilityStatement_Software,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["software"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CapabilityStatementStatus,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut CapabilityStatementBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut CapabilityStatementBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatementBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum CapabilityStatementFhirVersion {
    Fhir001,
    Fhir005,
    Fhir006,
    Fhir011,
    Fhir0080,
    Fhir0081,
    Fhir0082,
    Fhir040,
    Fhir050,
    Fhir100,
    Fhir101,
    Fhir102,
    Fhir110,
    Fhir140,
    Fhir160,
    Fhir180,
    Fhir300,
    Fhir301,
    Fhir330,
    Fhir350,
    Fhir400,
    Fhir401,
}

impl CapabilityStatementFhirVersion {
    pub fn from_string(string: &str) -> Option<CapabilityStatementFhirVersion> {
        match string {
            "0.01" => Some(CapabilityStatementFhirVersion::Fhir001),
            "0.05" => Some(CapabilityStatementFhirVersion::Fhir005),
            "0.06" => Some(CapabilityStatementFhirVersion::Fhir006),
            "0.11" => Some(CapabilityStatementFhirVersion::Fhir011),
            "0.0.80" => Some(CapabilityStatementFhirVersion::Fhir0080),
            "0.0.81" => Some(CapabilityStatementFhirVersion::Fhir0081),
            "0.0.82" => Some(CapabilityStatementFhirVersion::Fhir0082),
            "0.4.0" => Some(CapabilityStatementFhirVersion::Fhir040),
            "0.5.0" => Some(CapabilityStatementFhirVersion::Fhir050),
            "1.0.0" => Some(CapabilityStatementFhirVersion::Fhir100),
            "1.0.1" => Some(CapabilityStatementFhirVersion::Fhir101),
            "1.0.2" => Some(CapabilityStatementFhirVersion::Fhir102),
            "1.1.0" => Some(CapabilityStatementFhirVersion::Fhir110),
            "1.4.0" => Some(CapabilityStatementFhirVersion::Fhir140),
            "1.6.0" => Some(CapabilityStatementFhirVersion::Fhir160),
            "1.8.0" => Some(CapabilityStatementFhirVersion::Fhir180),
            "3.0.0" => Some(CapabilityStatementFhirVersion::Fhir300),
            "3.0.1" => Some(CapabilityStatementFhirVersion::Fhir301),
            "3.3.0" => Some(CapabilityStatementFhirVersion::Fhir330),
            "3.5.0" => Some(CapabilityStatementFhirVersion::Fhir350),
            "4.0.0" => Some(CapabilityStatementFhirVersion::Fhir400),
            "4.0.1" => Some(CapabilityStatementFhirVersion::Fhir401),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CapabilityStatementFhirVersion::Fhir001 => "0.01".to_string(),
            CapabilityStatementFhirVersion::Fhir005 => "0.05".to_string(),
            CapabilityStatementFhirVersion::Fhir006 => "0.06".to_string(),
            CapabilityStatementFhirVersion::Fhir011 => "0.11".to_string(),
            CapabilityStatementFhirVersion::Fhir0080 => "0.0.80".to_string(),
            CapabilityStatementFhirVersion::Fhir0081 => "0.0.81".to_string(),
            CapabilityStatementFhirVersion::Fhir0082 => "0.0.82".to_string(),
            CapabilityStatementFhirVersion::Fhir040 => "0.4.0".to_string(),
            CapabilityStatementFhirVersion::Fhir050 => "0.5.0".to_string(),
            CapabilityStatementFhirVersion::Fhir100 => "1.0.0".to_string(),
            CapabilityStatementFhirVersion::Fhir101 => "1.0.1".to_string(),
            CapabilityStatementFhirVersion::Fhir102 => "1.0.2".to_string(),
            CapabilityStatementFhirVersion::Fhir110 => "1.1.0".to_string(),
            CapabilityStatementFhirVersion::Fhir140 => "1.4.0".to_string(),
            CapabilityStatementFhirVersion::Fhir160 => "1.6.0".to_string(),
            CapabilityStatementFhirVersion::Fhir180 => "1.8.0".to_string(),
            CapabilityStatementFhirVersion::Fhir300 => "3.0.0".to_string(),
            CapabilityStatementFhirVersion::Fhir301 => "3.0.1".to_string(),
            CapabilityStatementFhirVersion::Fhir330 => "3.3.0".to_string(),
            CapabilityStatementFhirVersion::Fhir350 => "3.5.0".to_string(),
            CapabilityStatementFhirVersion::Fhir400 => "4.0.0".to_string(),
            CapabilityStatementFhirVersion::Fhir401 => "4.0.1".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum CapabilityStatementKind {
    Instance,
    Capability,
    Requirements,
}

impl CapabilityStatementKind {
    pub fn from_string(string: &str) -> Option<CapabilityStatementKind> {
        match string {
            "instance" => Some(CapabilityStatementKind::Instance),
            "capability" => Some(CapabilityStatementKind::Capability),
            "requirements" => Some(CapabilityStatementKind::Requirements),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CapabilityStatementKind::Instance => "instance".to_string(),
            CapabilityStatementKind::Capability => "capability".to_string(),
            CapabilityStatementKind::Requirements => "requirements".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum CapabilityStatementStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl CapabilityStatementStatus {
    pub fn from_string(string: &str) -> Option<CapabilityStatementStatus> {
        match string {
            "draft" => Some(CapabilityStatementStatus::Draft),
            "active" => Some(CapabilityStatementStatus::Active),
            "retired" => Some(CapabilityStatementStatus::Retired),
            "unknown" => Some(CapabilityStatementStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CapabilityStatementStatus::Draft => "draft".to_string(),
            CapabilityStatementStatus::Active => "active".to_string(),
            CapabilityStatementStatus::Retired => "retired".to_string(),
            CapabilityStatementStatus::Unknown => "unknown".to_string(),
        }
    }
}

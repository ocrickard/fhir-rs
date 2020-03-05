#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::StructureDefinition_Context::StructureDefinition_Context;
use crate::model::StructureDefinition_Differential::StructureDefinition_Differential;
use crate::model::StructureDefinition_Mapping::StructureDefinition_Mapping;
use crate::model::StructureDefinition_Snapshot::StructureDefinition_Snapshot;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.

#[derive(Debug)]
pub struct StructureDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureDefinition<'_> {
    pub fn new(value: &Value) -> StructureDefinition {
        StructureDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for abstract
    pub fn _abstract(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_abstract") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for contextInvariant
    pub fn _context_invariant(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_contextInvariant") {
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

    /// Extensions for derivation
    pub fn _derivation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_derivation") {
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

    /// Whether structure this definition describes is abstract or not  - that is,
    /// whether the structure is not intended to be instantiated. For Resources and Data
    /// types, abstract types will never be exchanged  between systems.
    pub fn fhir_abstract(&self) -> Option<bool> {
        if let Some(val) = self.value.get("abstract") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An absolute URI that is the base structure from which this type is derived,
    /// either by specialization or constraint.
    pub fn base_definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("baseDefinition") {
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

    /// Identifies the types of resource or data type elements to which the extension
    /// can be applied.
    pub fn context(&self) -> Option<Vec<StructureDefinition_Context>> {
        if let Some(Value::Array(val)) = self.value.get("context") {
            return Some(
                val.into_iter()
                    .map(|e| StructureDefinition_Context {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A set of rules as FHIRPath Invariants about when the extension can be used (e.g.
    /// co-occurrence variants for the extension). All the rules must be true.
    pub fn context_invariant(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("contextInvariant") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A copyright statement relating to the structure definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the structure definition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the structure definition was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the structure definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// How the type relates to the baseDefinition.
    pub fn derivation(&self) -> Option<StructureDefinitionDerivation> {
        if let Some(Value::String(val)) = self.value.get("derivation") {
            return Some(StructureDefinitionDerivation::from_string(&val).unwrap());
        }
        return None;
    }

    /// A free text natural language description of the structure definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A differential view is expressed relative to the base StructureDefinition - a
    /// statement of differences that it applies.
    pub fn differential(&self) -> Option<StructureDefinition_Differential> {
        if let Some(val) = self.value.get("differential") {
            return Some(StructureDefinition_Differential {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A Boolean value to indicate that this structure definition is authored for
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

    /// The version of the FHIR specification on which this StructureDefinition is based
    /// - this is the formal version of the specification, without the revision number,
    /// e.g. [publication].[major].[minor], which is 4.0.1. for this version.
    pub fn fhir_version(&self) -> Option<StructureDefinitionFhirVersion> {
        if let Some(Value::String(val)) = self.value.get("fhirVersion") {
            return Some(StructureDefinitionFhirVersion::from_string(&val).unwrap());
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

    /// A formal identifier that is used to identify this structure definition when it
    /// is represented in other formats, or referenced in a specification, model, design
    /// or an instance.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// A legal or geographic region in which the structure definition is intended to be
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

    /// A set of key words or terms from external terminologies that may be used to
    /// assist with indexing and searching of templates nby describing the use of this
    /// structure definition, or the content it describes.
    pub fn keyword(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("keyword") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines the kind of structure that this definition is describing.
    pub fn kind(&self) -> Option<StructureDefinitionKind> {
        if let Some(Value::String(val)) = self.value.get("kind") {
            return Some(StructureDefinitionKind::from_string(&val).unwrap());
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

    /// An external specification that the content is mapped to.
    pub fn mapping(&self) -> Option<Vec<StructureDefinition_Mapping>> {
        if let Some(Value::Array(val)) = self.value.get("mapping") {
            return Some(
                val.into_iter()
                    .map(|e| StructureDefinition_Mapping {
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

    /// A natural language name identifying the structure definition. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the structure
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this structure definition is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// A snapshot view is expressed in a standalone form that can be used and
    /// interpreted without considering the base StructureDefinition.
    pub fn snapshot(&self) -> Option<StructureDefinition_Snapshot> {
        if let Some(val) = self.value.get("snapshot") {
            return Some(StructureDefinition_Snapshot {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of this structure definition. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<StructureDefinitionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(StructureDefinitionStatus::from_string(&val).unwrap());
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

    /// A short, descriptive, user-friendly title for the structure definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// The type this structure describes. If the derivation kind is 'specialization'
    /// then this is the master definition for a type, and there is always one of these
    /// (a data type, an extension, a resource, including abstract ones). Otherwise the
    /// structure definition is a constraint on the stated type (and in this case, the
    /// type cannot be an abstract type).  References are URLs that are relative to
    /// http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to
    /// http://hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed
    /// in logical models.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that is used to identify this structure definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this structure definition
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the structure definition is stored on different
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
    /// be used to assist with indexing and searching for appropriate structure
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

    /// The identifier that is used to identify this version of the structure definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the structure definition author and is not expected
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
        if let Some(_val) = self._abstract() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._context_invariant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._derivation() {
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
        if let Some(_val) = self.fhir_abstract() {}
        if let Some(_val) = self.base_definition() {}
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
        if let Some(_val) = self.context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.context_invariant() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.derivation() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.differential() {
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
        if let Some(_val) = self.fhir_version() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.keyword() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.mapping() {
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
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.snapshot() {
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
pub struct StructureDefinitionBuilder {
    pub(crate) value: Value,
}

impl StructureDefinitionBuilder {
    pub fn build(&self) -> StructureDefinition {
        StructureDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureDefinition) -> StructureDefinitionBuilder {
        StructureDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureDefinitionBuilder {
        let mut __value: Value = json!({});
        return StructureDefinitionBuilder { value: __value };
    }

    pub fn _abstract<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_abstract"] = json!(val.value);
        return self;
    }

    pub fn _context_invariant<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["_contextInvariant"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _derivation<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_derivation"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _fhir_version<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_fhirVersion"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_purpose"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut StructureDefinitionBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn fhir_abstract<'a>(&'a mut self, val: bool) -> &'a mut StructureDefinitionBuilder {
        self.value["abstract"] = json!(val);
        return self;
    }

    pub fn base_definition<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["baseDefinition"] = json!(val);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn context<'a>(
        &'a mut self,
        val: Vec<StructureDefinition_Context>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["context"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn context_invariant<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["contextInvariant"] = json!(val);
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn derivation<'a>(
        &'a mut self,
        val: StructureDefinitionDerivation,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["derivation"] = json!(val.to_string());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn differential<'a>(
        &'a mut self,
        val: StructureDefinition_Differential,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["differential"] = json!(val.value);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut StructureDefinitionBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_version<'a>(
        &'a mut self,
        val: StructureDefinitionFhirVersion,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["fhirVersion"] = json!(val.to_string());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn keyword<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut StructureDefinitionBuilder {
        self.value["keyword"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn kind<'a>(
        &'a mut self,
        val: StructureDefinitionKind,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["kind"] = json!(val.to_string());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn mapping<'a>(
        &'a mut self,
        val: Vec<StructureDefinition_Mapping>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["mapping"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut StructureDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn purpose<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["purpose"] = json!(val);
        return self;
    }

    pub fn snapshot<'a>(
        &'a mut self,
        val: StructureDefinition_Snapshot,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["snapshot"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: StructureDefinitionStatus,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut StructureDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut StructureDefinitionBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut StructureDefinitionBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum StructureDefinitionDerivation {
    Specialization,
    Constraint,
}

impl StructureDefinitionDerivation {
    pub fn from_string(string: &str) -> Option<StructureDefinitionDerivation> {
        match string {
            "specialization" => Some(StructureDefinitionDerivation::Specialization),
            "constraint" => Some(StructureDefinitionDerivation::Constraint),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureDefinitionDerivation::Specialization => "specialization".to_string(),
            StructureDefinitionDerivation::Constraint => "constraint".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum StructureDefinitionFhirVersion {
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

impl StructureDefinitionFhirVersion {
    pub fn from_string(string: &str) -> Option<StructureDefinitionFhirVersion> {
        match string {
            "0.01" => Some(StructureDefinitionFhirVersion::Fhir001),
            "0.05" => Some(StructureDefinitionFhirVersion::Fhir005),
            "0.06" => Some(StructureDefinitionFhirVersion::Fhir006),
            "0.11" => Some(StructureDefinitionFhirVersion::Fhir011),
            "0.0.80" => Some(StructureDefinitionFhirVersion::Fhir0080),
            "0.0.81" => Some(StructureDefinitionFhirVersion::Fhir0081),
            "0.0.82" => Some(StructureDefinitionFhirVersion::Fhir0082),
            "0.4.0" => Some(StructureDefinitionFhirVersion::Fhir040),
            "0.5.0" => Some(StructureDefinitionFhirVersion::Fhir050),
            "1.0.0" => Some(StructureDefinitionFhirVersion::Fhir100),
            "1.0.1" => Some(StructureDefinitionFhirVersion::Fhir101),
            "1.0.2" => Some(StructureDefinitionFhirVersion::Fhir102),
            "1.1.0" => Some(StructureDefinitionFhirVersion::Fhir110),
            "1.4.0" => Some(StructureDefinitionFhirVersion::Fhir140),
            "1.6.0" => Some(StructureDefinitionFhirVersion::Fhir160),
            "1.8.0" => Some(StructureDefinitionFhirVersion::Fhir180),
            "3.0.0" => Some(StructureDefinitionFhirVersion::Fhir300),
            "3.0.1" => Some(StructureDefinitionFhirVersion::Fhir301),
            "3.3.0" => Some(StructureDefinitionFhirVersion::Fhir330),
            "3.5.0" => Some(StructureDefinitionFhirVersion::Fhir350),
            "4.0.0" => Some(StructureDefinitionFhirVersion::Fhir400),
            "4.0.1" => Some(StructureDefinitionFhirVersion::Fhir401),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureDefinitionFhirVersion::Fhir001 => "0.01".to_string(),
            StructureDefinitionFhirVersion::Fhir005 => "0.05".to_string(),
            StructureDefinitionFhirVersion::Fhir006 => "0.06".to_string(),
            StructureDefinitionFhirVersion::Fhir011 => "0.11".to_string(),
            StructureDefinitionFhirVersion::Fhir0080 => "0.0.80".to_string(),
            StructureDefinitionFhirVersion::Fhir0081 => "0.0.81".to_string(),
            StructureDefinitionFhirVersion::Fhir0082 => "0.0.82".to_string(),
            StructureDefinitionFhirVersion::Fhir040 => "0.4.0".to_string(),
            StructureDefinitionFhirVersion::Fhir050 => "0.5.0".to_string(),
            StructureDefinitionFhirVersion::Fhir100 => "1.0.0".to_string(),
            StructureDefinitionFhirVersion::Fhir101 => "1.0.1".to_string(),
            StructureDefinitionFhirVersion::Fhir102 => "1.0.2".to_string(),
            StructureDefinitionFhirVersion::Fhir110 => "1.1.0".to_string(),
            StructureDefinitionFhirVersion::Fhir140 => "1.4.0".to_string(),
            StructureDefinitionFhirVersion::Fhir160 => "1.6.0".to_string(),
            StructureDefinitionFhirVersion::Fhir180 => "1.8.0".to_string(),
            StructureDefinitionFhirVersion::Fhir300 => "3.0.0".to_string(),
            StructureDefinitionFhirVersion::Fhir301 => "3.0.1".to_string(),
            StructureDefinitionFhirVersion::Fhir330 => "3.3.0".to_string(),
            StructureDefinitionFhirVersion::Fhir350 => "3.5.0".to_string(),
            StructureDefinitionFhirVersion::Fhir400 => "4.0.0".to_string(),
            StructureDefinitionFhirVersion::Fhir401 => "4.0.1".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum StructureDefinitionKind {
    PrimitiveType,
    ComplexType,
    Resource,
    Logical,
}

impl StructureDefinitionKind {
    pub fn from_string(string: &str) -> Option<StructureDefinitionKind> {
        match string {
            "primitive-type" => Some(StructureDefinitionKind::PrimitiveType),
            "complex-type" => Some(StructureDefinitionKind::ComplexType),
            "resource" => Some(StructureDefinitionKind::Resource),
            "logical" => Some(StructureDefinitionKind::Logical),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureDefinitionKind::PrimitiveType => "primitive-type".to_string(),
            StructureDefinitionKind::ComplexType => "complex-type".to_string(),
            StructureDefinitionKind::Resource => "resource".to_string(),
            StructureDefinitionKind::Logical => "logical".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum StructureDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl StructureDefinitionStatus {
    pub fn from_string(string: &str) -> Option<StructureDefinitionStatus> {
        match string {
            "draft" => Some(StructureDefinitionStatus::Draft),
            "active" => Some(StructureDefinitionStatus::Active),
            "retired" => Some(StructureDefinitionStatus::Retired),
            "unknown" => Some(StructureDefinitionStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureDefinitionStatus::Draft => "draft".to_string(),
            StructureDefinitionStatus::Active => "active".to_string(),
            StructureDefinitionStatus::Retired => "retired".to_string(),
            StructureDefinitionStatus::Unknown => "unknown".to_string(),
        }
    }
}

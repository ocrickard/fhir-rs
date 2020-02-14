#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ConceptMap_Group::ConceptMap_Group;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap<'a> {
    pub value: &'a Value,
}

impl ConceptMap<'_> {
    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date  (and optionally time) when the concept map was published. The date
    /// must change when the business version changes and it must change if the status
    /// code changes. In addition, it should change when the substantive content of the
    /// concept map changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
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

    /// Explanation of why this concept map is needed and why it has been designed as it
    /// has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the concept map.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
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

    /// Identifier for the source value set that contains the concepts that are being
    /// mapped and provides context for the mappings.
    pub fn source_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceUri") {
            return Some(string);
        }
        return None;
    }

    /// Identifier for the source value set that contains the concepts that are being
    /// mapped and provides context for the mappings.
    pub fn source_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceCanonical") {
            return Some(string);
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the concept map.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
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
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A copyright statement relating to the concept map and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// concept map.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for sourceUri
    pub fn _source_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceUri") {
            return Some(Element { value: val });
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

    /// Extensions for sourceCanonical
    pub fn _source_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The target value set provides context for the mappings. Note that the mapping is
    /// made between concepts, not between value sets, but the value set provides
    /// important context about how the concept mapping choices are made.
    pub fn target_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetCanonical") {
            return Some(string);
        }
        return None;
    }

    /// A group of mappings that all have the same source and target system.
    pub fn group(&self) -> Option<Vec<ConceptMap_Group>> {
        if let Some(Value::Array(val)) = self.value.get("group") {
            return Some(
                val.into_iter()
                    .map(|e| ConceptMap_Group { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate concept map
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

    /// The target value set provides context for the mappings. Note that the mapping is
    /// made between concepts, not between value sets, but the value set provides
    /// important context about how the concept mapping choices are made.
    pub fn target_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetUri") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that is used to identify this concept map when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which at which an authoritative instance of this concept map is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the concept map is stored on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
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

    /// A legal or geographic region in which the concept map is intended to be used.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A Boolean value to indicate that this concept map is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for targetUri
    pub fn _target_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetUri") {
            return Some(Element { value: val });
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

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// A formal identifier that is used to identify this concept map when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// The identifier that is used to identify this version of the concept map when it
    /// is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the concept map author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
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

    /// A natural language name identifying the concept map. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
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

    /// The status of this concept map. Enables tracking the life-cycle of the content.
    pub fn status(&self) -> Option<ConceptMapStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ConceptMapStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// A free text natural language description of the concept map from a consumer's
    /// perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for targetCanonical
    pub fn _target_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetCanonical") {
            return Some(Element { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.source_uri() {}
        if let Some(_val) = self.source_canonical() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self._source_uri() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._source_canonical() {
            _val.validate();
        }
        if let Some(_val) = self.target_canonical() {}
        if let Some(_val) = self.group() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.target_uri() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self._target_uri() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self._target_canonical() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ConceptMapStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl ConceptMapStatus {
    pub fn from_string(string: &str) -> Option<ConceptMapStatus> {
        match string {
            "draft" => Some(ConceptMapStatus::Draft),
            "active" => Some(ConceptMapStatus::Active),
            "retired" => Some(ConceptMapStatus::Retired),
            "unknown" => Some(ConceptMapStatus::Unknown),
            _ => None,
        }
    }
}

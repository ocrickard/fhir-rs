#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of healthcare-related information that is assembled together into a single
/// logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to who
/// is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).

#[derive(Debug)]
pub struct Composition_Section<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Composition_Section<'_> {
    pub fn new(value: &Value) -> Composition_Section {
        Composition_Section {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
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

    /// Identifies who is responsible for the information in this section, not
    /// necessarily who typed it in.
    pub fn author(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code identifying the kind of content contained within the section. This must
    /// be consistent with the section title.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// If the section is empty, why the list is empty. An empty section typically has
    /// some text explaining the empty reason.
    pub fn empty_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("emptyReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to the actual resource from which the narrative in the section is
    /// derived.
    pub fn entry(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("entry") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
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

    /// The actual focus of the section when it is not the subject of the composition,
    /// but instead represents something or someone associated with the subject such as
    /// (for a patient subject) a spouse, parent, fetus, or donor. If not focus is
    /// specified, the focus is assumed to be focus of the parent section, or, for a
    /// section in the Composition itself, the subject of the composition. Sections with
    /// a focus SHALL only include resources where the logical subject (patient,
    /// subject, focus, etc.) matches the section focus, or the resources have no
    /// logical subject (few resources).
    pub fn focus(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("focus") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// How the entry list was prepared - whether it is a working list that is suitable
    /// for being maintained on an ongoing basis, or if it represents a snapshot of a
    /// list of items from another source, or whether it is a prepared list where items
    /// may be marked as added, modified or deleted.
    pub fn mode(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("mode") {
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

    /// Specifies the order applied to the items in the section entries.
    pub fn ordered_by(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("orderedBy") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A nested sub-section within this section.
    pub fn section(&self) -> Option<Vec<Composition_Section>> {
        if let Some(Value::Array(val)) = self.value.get("section") {
            return Some(
                val.into_iter()
                    .map(|e| Composition_Section {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human-readable narrative that contains the attested content of the section,
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The label for this particular section.  This will be part of the rendered
    /// content for the document, and is often used to build a table of contents.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.empty_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.entry() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.ordered_by() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.section() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Composition_SectionBuilder {
    pub(crate) value: Value,
}

impl Composition_SectionBuilder {
    pub fn build(&self) -> Composition_Section {
        Composition_Section {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Composition_Section) -> Composition_SectionBuilder {
        Composition_SectionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Composition_SectionBuilder {
        let mut __value: Value = json!({});
        return Composition_SectionBuilder { value: __value };
    }

    pub fn _mode<'a>(&'a mut self, val: Element) -> &'a mut Composition_SectionBuilder {
        self.value["_mode"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut Composition_SectionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Composition_SectionBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Composition_SectionBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn empty_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Composition_SectionBuilder {
        self.value["emptyReason"] = json!(val.value);
        return self;
    }

    pub fn entry<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Composition_SectionBuilder {
        self.value["entry"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Composition_SectionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: Reference) -> &'a mut Composition_SectionBuilder {
        self.value["focus"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Composition_SectionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn mode<'a>(&'a mut self, val: &str) -> &'a mut Composition_SectionBuilder {
        self.value["mode"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Composition_SectionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn ordered_by<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Composition_SectionBuilder {
        self.value["orderedBy"] = json!(val.value);
        return self;
    }

    pub fn section<'a>(
        &'a mut self,
        val: Vec<Composition_Section>,
    ) -> &'a mut Composition_SectionBuilder {
        self.value["section"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut Composition_SectionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut Composition_SectionBuilder {
        self.value["title"] = json!(val);
        return self;
    }
}

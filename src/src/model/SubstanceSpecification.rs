#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceSpecification_Code::SubstanceSpecification_Code;
use crate::model::SubstanceSpecification_Moiety::SubstanceSpecification_Moiety;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;
use crate::model::SubstanceSpecification_Name::SubstanceSpecification_Name;
use crate::model::SubstanceSpecification_Property::SubstanceSpecification_Property;
use crate::model::SubstanceSpecification_Relationship::SubstanceSpecification_Relationship;
use crate::model::SubstanceSpecification_Structure::SubstanceSpecification_Structure;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification {
        SubstanceSpecification {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
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

    /// Codes associated with the substance.
    pub fn code(&self) -> Option<Vec<SubstanceSpecification_Code>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Code {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Textual comment about this record of a substance.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Textual description of the substance.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// If the substance applies to only human or veterinary use.
    pub fn domain(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("domain") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Identifier by which this substance is known.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
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

    /// Moiety, for structural modifications.
    pub fn moiety(&self) -> Option<Vec<SubstanceSpecification_Moiety>> {
        if let Some(Value::Array(val)) = self.value.get("moiety") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Moiety {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<Vec<SubstanceSpecification_MolecularWeight>> {
        if let Some(Value::Array(val)) = self.value.get("molecularWeight") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_MolecularWeight {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Names applicable to this substance.
    pub fn name(&self) -> Option<Vec<SubstanceSpecification_Name>> {
        if let Some(Value::Array(val)) = self.value.get("name") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Name {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Data items specific to nucleic acids.
    pub fn nucleic_acid(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("nucleicAcid") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Data items specific to polymers.
    pub fn polymer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("polymer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// General specifications for this substance, including how it is related to other
    /// substances.
    pub fn property(&self) -> Option<Vec<SubstanceSpecification_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Data items specific to proteins.
    pub fn protein(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("protein") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// General information detailing this substance.
    pub fn reference_information(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("referenceInformation") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A link between this substance and another, with details of the relationship.
    pub fn relationship(&self) -> Option<Vec<SubstanceSpecification_Relationship>> {
        if let Some(Value::Array(val)) = self.value.get("relationship") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Relationship {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Supporting literature.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
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

    /// Material or taxonomic/anatomical source for the substance.
    pub fn source_material(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sourceMaterial") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Status of substance within the catalogue e.g. approved.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Structural information.
    pub fn structure(&self) -> Option<SubstanceSpecification_Structure> {
        if let Some(val) = self.value.get("structure") {
            return Some(SubstanceSpecification_Structure {
                value: Cow::Borrowed(val),
            });
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

    /// High level categorization, e.g. polymer or nucleic acid.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
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
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.domain() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
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
        if let Some(_val) = self.moiety() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.molecular_weight() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.nucleic_acid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.polymer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.protein() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reference_information() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source_material() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.structure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecificationBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecificationBuilder {
    pub fn build(&self) -> SubstanceSpecification {
        SubstanceSpecification {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSpecification) -> SubstanceSpecificationBuilder {
        SubstanceSpecificationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecificationBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecificationBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecificationBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecificationBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecificationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Code>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecificationBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecificationBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn domain<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SubstanceSpecificationBuilder {
        self.value["domain"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecificationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut SubstanceSpecificationBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecificationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecificationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubstanceSpecificationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn moiety<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Moiety>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["moiety"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn molecular_weight<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_MolecularWeight>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["molecularWeight"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Name>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["name"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn nucleic_acid<'a>(&'a mut self, val: Reference) -> &'a mut SubstanceSpecificationBuilder {
        self.value["nucleicAcid"] = json!(val.value);
        return self;
    }

    pub fn polymer<'a>(&'a mut self, val: Reference) -> &'a mut SubstanceSpecificationBuilder {
        self.value["polymer"] = json!(val.value);
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Property>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn protein<'a>(&'a mut self, val: Reference) -> &'a mut SubstanceSpecificationBuilder {
        self.value["protein"] = json!(val.value);
        return self;
    }

    pub fn reference_information<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["referenceInformation"] = json!(val.value);
        return self;
    }

    pub fn relationship<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Relationship>,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["relationship"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut SubstanceSpecificationBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source_material<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["sourceMaterial"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SubstanceSpecificationBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn structure<'a>(
        &'a mut self,
        val: SubstanceSpecification_Structure,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["structure"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubstanceSpecificationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecificationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}

#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.

#[derive(Debug)]
pub struct SubstanceNucleicAcid_Sugar<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceNucleicAcid_Sugar<'_> {
    pub fn new(value: &Value) -> SubstanceNucleicAcid_Sugar {
        SubstanceNucleicAcid_Sugar {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for residueSite
    pub fn _residue_site(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_residueSite") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// The Substance ID of the sugar or sugar-like component that make up the
    /// nucleotide.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// The name of the sugar or sugar-like component that make up the nucleotide.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The residues that contain a given sugar will be captured. The order of given
    /// residues will be captured in the 5‘-3‘direction consistent with the base
    /// sequences listed above.
    pub fn residue_site(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("residueSite") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._residue_site() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.residue_site() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceNucleicAcid_SugarBuilder {
    pub(crate) value: Value,
}

impl SubstanceNucleicAcid_SugarBuilder {
    pub fn build(&self) -> SubstanceNucleicAcid_Sugar {
        SubstanceNucleicAcid_Sugar {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceNucleicAcid_Sugar) -> SubstanceNucleicAcid_SugarBuilder {
        SubstanceNucleicAcid_SugarBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceNucleicAcid_SugarBuilder {
        let mut __value: Value = json!({});
        return SubstanceNucleicAcid_SugarBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _residue_site<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["_residueSite"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn residue_site<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcid_SugarBuilder {
        self.value["residueSite"] = json!(val);
        return self;
    }
}

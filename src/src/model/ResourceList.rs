#![allow(unused_imports, non_camel_case_types)]

use crate::model::Account::Account;
use crate::model::ActivityDefinition::ActivityDefinition;
use crate::model::AdverseEvent::AdverseEvent;
use crate::model::AllergyIntolerance::AllergyIntolerance;
use crate::model::Appointment::Appointment;
use crate::model::AppointmentResponse::AppointmentResponse;
use crate::model::AuditEvent::AuditEvent;
use crate::model::Basic::Basic;
use crate::model::Binary::Binary;
use crate::model::BiologicallyDerivedProduct::BiologicallyDerivedProduct;
use crate::model::BodyStructure::BodyStructure;
use crate::model::Bundle::Bundle;
use crate::model::CapabilityStatement::CapabilityStatement;
use crate::model::CarePlan::CarePlan;
use crate::model::CareTeam::CareTeam;
use crate::model::CatalogEntry::CatalogEntry;
use crate::model::ChargeItem::ChargeItem;
use crate::model::ChargeItemDefinition::ChargeItemDefinition;
use crate::model::Claim::Claim;
use crate::model::ClaimResponse::ClaimResponse;
use crate::model::ClinicalImpression::ClinicalImpression;
use crate::model::CodeSystem::CodeSystem;
use crate::model::Communication::Communication;
use crate::model::CommunicationRequest::CommunicationRequest;
use crate::model::CompartmentDefinition::CompartmentDefinition;
use crate::model::Composition::Composition;
use crate::model::ConceptMap::ConceptMap;
use crate::model::Condition::Condition;
use crate::model::Consent::Consent;
use crate::model::Contract::Contract;
use crate::model::Coverage::Coverage;
use crate::model::CoverageEligibilityRequest::CoverageEligibilityRequest;
use crate::model::CoverageEligibilityResponse::CoverageEligibilityResponse;
use crate::model::DetectedIssue::DetectedIssue;
use crate::model::Device::Device;
use crate::model::DeviceDefinition::DeviceDefinition;
use crate::model::DeviceMetric::DeviceMetric;
use crate::model::DeviceRequest::DeviceRequest;
use crate::model::DeviceUseStatement::DeviceUseStatement;
use crate::model::DiagnosticReport::DiagnosticReport;
use crate::model::DocumentManifest::DocumentManifest;
use crate::model::DocumentReference::DocumentReference;
use crate::model::EffectEvidenceSynthesis::EffectEvidenceSynthesis;
use crate::model::Encounter::Encounter;
use crate::model::Endpoint::Endpoint;
use crate::model::EnrollmentRequest::EnrollmentRequest;
use crate::model::EnrollmentResponse::EnrollmentResponse;
use crate::model::EpisodeOfCare::EpisodeOfCare;
use crate::model::EventDefinition::EventDefinition;
use crate::model::Evidence::Evidence;
use crate::model::EvidenceVariable::EvidenceVariable;
use crate::model::ExampleScenario::ExampleScenario;
use crate::model::ExplanationOfBenefit::ExplanationOfBenefit;
use crate::model::FamilyMemberHistory::FamilyMemberHistory;
use crate::model::Flag::Flag;
use crate::model::Goal::Goal;
use crate::model::GraphDefinition::GraphDefinition;
use crate::model::Group::Group;
use crate::model::GuidanceResponse::GuidanceResponse;
use crate::model::HealthcareService::HealthcareService;
use crate::model::ImagingStudy::ImagingStudy;
use crate::model::Immunization::Immunization;
use crate::model::ImmunizationEvaluation::ImmunizationEvaluation;
use crate::model::ImmunizationRecommendation::ImmunizationRecommendation;
use crate::model::ImplementationGuide::ImplementationGuide;
use crate::model::InsurancePlan::InsurancePlan;
use crate::model::Invoice::Invoice;
use crate::model::Library::Library;
use crate::model::Linkage::Linkage;
use crate::model::List::List;
use crate::model::Location::Location;
use crate::model::Measure::Measure;
use crate::model::MeasureReport::MeasureReport;
use crate::model::Media::Media;
use crate::model::Medication::Medication;
use crate::model::MedicationAdministration::MedicationAdministration;
use crate::model::MedicationDispense::MedicationDispense;
use crate::model::MedicationKnowledge::MedicationKnowledge;
use crate::model::MedicationRequest::MedicationRequest;
use crate::model::MedicationStatement::MedicationStatement;
use crate::model::MedicinalProduct::MedicinalProduct;
use crate::model::MedicinalProductAuthorization::MedicinalProductAuthorization;
use crate::model::MedicinalProductContraindication::MedicinalProductContraindication;
use crate::model::MedicinalProductIndication::MedicinalProductIndication;
use crate::model::MedicinalProductIngredient::MedicinalProductIngredient;
use crate::model::MedicinalProductInteraction::MedicinalProductInteraction;
use crate::model::MedicinalProductManufactured::MedicinalProductManufactured;
use crate::model::MedicinalProductPackaged::MedicinalProductPackaged;
use crate::model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical;
use crate::model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect;
use crate::model::MessageDefinition::MessageDefinition;
use crate::model::MessageHeader::MessageHeader;
use crate::model::MolecularSequence::MolecularSequence;
use crate::model::NamingSystem::NamingSystem;
use crate::model::NutritionOrder::NutritionOrder;
use crate::model::Observation::Observation;
use crate::model::ObservationDefinition::ObservationDefinition;
use crate::model::OperationDefinition::OperationDefinition;
use crate::model::OperationOutcome::OperationOutcome;
use crate::model::Organization::Organization;
use crate::model::OrganizationAffiliation::OrganizationAffiliation;
use crate::model::Parameters::Parameters;
use crate::model::Patient::Patient;
use crate::model::PaymentNotice::PaymentNotice;
use crate::model::PaymentReconciliation::PaymentReconciliation;
use crate::model::Person::Person;
use crate::model::PlanDefinition::PlanDefinition;
use crate::model::Practitioner::Practitioner;
use crate::model::PractitionerRole::PractitionerRole;
use crate::model::Procedure::Procedure;
use crate::model::Provenance::Provenance;
use crate::model::Questionnaire::Questionnaire;
use crate::model::QuestionnaireResponse::QuestionnaireResponse;
use crate::model::RelatedPerson::RelatedPerson;
use crate::model::RequestGroup::RequestGroup;
use crate::model::ResearchDefinition::ResearchDefinition;
use crate::model::ResearchElementDefinition::ResearchElementDefinition;
use crate::model::ResearchStudy::ResearchStudy;
use crate::model::ResearchSubject::ResearchSubject;
use crate::model::RiskAssessment::RiskAssessment;
use crate::model::RiskEvidenceSynthesis::RiskEvidenceSynthesis;
use crate::model::Schedule::Schedule;
use crate::model::SearchParameter::SearchParameter;
use crate::model::ServiceRequest::ServiceRequest;
use crate::model::Slot::Slot;
use crate::model::Specimen::Specimen;
use crate::model::SpecimenDefinition::SpecimenDefinition;
use crate::model::StructureDefinition::StructureDefinition;
use crate::model::StructureMap::StructureMap;
use crate::model::Subscription::Subscription;
use crate::model::Substance::Substance;
use crate::model::SubstanceNucleicAcid::SubstanceNucleicAcid;
use crate::model::SubstancePolymer::SubstancePolymer;
use crate::model::SubstanceProtein::SubstanceProtein;
use crate::model::SubstanceReferenceInformation::SubstanceReferenceInformation;
use crate::model::SubstanceSourceMaterial::SubstanceSourceMaterial;
use crate::model::SubstanceSpecification::SubstanceSpecification;
use crate::model::SupplyDelivery::SupplyDelivery;
use crate::model::SupplyRequest::SupplyRequest;
use crate::model::Task::Task;
use crate::model::TerminologyCapabilities::TerminologyCapabilities;
use crate::model::TestReport::TestReport;
use crate::model::TestScript::TestScript;
use crate::model::ValueSet::ValueSet;
use crate::model::VerificationResult::VerificationResult;
use crate::model::VisionPrescription::VisionPrescription;
use serde_json::value::Value;
use std::borrow::Cow;

#[derive(Debug)]
pub struct ResourceList<'a> {
    pub(crate) value: Cow<'a, Value>,
}

#[derive(Debug)]
pub enum ResourceListEnum<'a> {
    ResourceAccount(Account<'a>),
    ResourceActivityDefinition(ActivityDefinition<'a>),
    ResourceAdverseEvent(AdverseEvent<'a>),
    ResourceAllergyIntolerance(AllergyIntolerance<'a>),
    ResourceAppointment(Appointment<'a>),
    ResourceAppointmentResponse(AppointmentResponse<'a>),
    ResourceAuditEvent(AuditEvent<'a>),
    ResourceBasic(Basic<'a>),
    ResourceBinary(Binary<'a>),
    ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct<'a>),
    ResourceBodyStructure(BodyStructure<'a>),
    ResourceBundle(Bundle<'a>),
    ResourceCapabilityStatement(CapabilityStatement<'a>),
    ResourceCarePlan(CarePlan<'a>),
    ResourceCareTeam(CareTeam<'a>),
    ResourceCatalogEntry(CatalogEntry<'a>),
    ResourceChargeItem(ChargeItem<'a>),
    ResourceChargeItemDefinition(ChargeItemDefinition<'a>),
    ResourceClaim(Claim<'a>),
    ResourceClaimResponse(ClaimResponse<'a>),
    ResourceClinicalImpression(ClinicalImpression<'a>),
    ResourceCodeSystem(CodeSystem<'a>),
    ResourceCommunication(Communication<'a>),
    ResourceCommunicationRequest(CommunicationRequest<'a>),
    ResourceCompartmentDefinition(CompartmentDefinition<'a>),
    ResourceComposition(Composition<'a>),
    ResourceConceptMap(ConceptMap<'a>),
    ResourceCondition(Condition<'a>),
    ResourceConsent(Consent<'a>),
    ResourceContract(Contract<'a>),
    ResourceCoverage(Coverage<'a>),
    ResourceCoverageEligibilityRequest(CoverageEligibilityRequest<'a>),
    ResourceCoverageEligibilityResponse(CoverageEligibilityResponse<'a>),
    ResourceDetectedIssue(DetectedIssue<'a>),
    ResourceDevice(Device<'a>),
    ResourceDeviceDefinition(DeviceDefinition<'a>),
    ResourceDeviceMetric(DeviceMetric<'a>),
    ResourceDeviceRequest(DeviceRequest<'a>),
    ResourceDeviceUseStatement(DeviceUseStatement<'a>),
    ResourceDiagnosticReport(DiagnosticReport<'a>),
    ResourceDocumentManifest(DocumentManifest<'a>),
    ResourceDocumentReference(DocumentReference<'a>),
    ResourceEffectEvidenceSynthesis(EffectEvidenceSynthesis<'a>),
    ResourceEncounter(Encounter<'a>),
    ResourceEndpoint(Endpoint<'a>),
    ResourceEnrollmentRequest(EnrollmentRequest<'a>),
    ResourceEnrollmentResponse(EnrollmentResponse<'a>),
    ResourceEpisodeOfCare(EpisodeOfCare<'a>),
    ResourceEventDefinition(EventDefinition<'a>),
    ResourceEvidence(Evidence<'a>),
    ResourceEvidenceVariable(EvidenceVariable<'a>),
    ResourceExampleScenario(ExampleScenario<'a>),
    ResourceExplanationOfBenefit(ExplanationOfBenefit<'a>),
    ResourceFamilyMemberHistory(FamilyMemberHistory<'a>),
    ResourceFlag(Flag<'a>),
    ResourceGoal(Goal<'a>),
    ResourceGraphDefinition(GraphDefinition<'a>),
    ResourceGroup(Group<'a>),
    ResourceGuidanceResponse(GuidanceResponse<'a>),
    ResourceHealthcareService(HealthcareService<'a>),
    ResourceImagingStudy(ImagingStudy<'a>),
    ResourceImmunization(Immunization<'a>),
    ResourceImmunizationEvaluation(ImmunizationEvaluation<'a>),
    ResourceImmunizationRecommendation(ImmunizationRecommendation<'a>),
    ResourceImplementationGuide(ImplementationGuide<'a>),
    ResourceInsurancePlan(InsurancePlan<'a>),
    ResourceInvoice(Invoice<'a>),
    ResourceLibrary(Library<'a>),
    ResourceLinkage(Linkage<'a>),
    ResourceList(List<'a>),
    ResourceLocation(Location<'a>),
    ResourceMeasure(Measure<'a>),
    ResourceMeasureReport(MeasureReport<'a>),
    ResourceMedia(Media<'a>),
    ResourceMedication(Medication<'a>),
    ResourceMedicationAdministration(MedicationAdministration<'a>),
    ResourceMedicationDispense(MedicationDispense<'a>),
    ResourceMedicationKnowledge(MedicationKnowledge<'a>),
    ResourceMedicationRequest(MedicationRequest<'a>),
    ResourceMedicationStatement(MedicationStatement<'a>),
    ResourceMedicinalProduct(MedicinalProduct<'a>),
    ResourceMedicinalProductAuthorization(MedicinalProductAuthorization<'a>),
    ResourceMedicinalProductContraindication(MedicinalProductContraindication<'a>),
    ResourceMedicinalProductIndication(MedicinalProductIndication<'a>),
    ResourceMedicinalProductIngredient(MedicinalProductIngredient<'a>),
    ResourceMedicinalProductInteraction(MedicinalProductInteraction<'a>),
    ResourceMedicinalProductManufactured(MedicinalProductManufactured<'a>),
    ResourceMedicinalProductPackaged(MedicinalProductPackaged<'a>),
    ResourceMedicinalProductPharmaceutical(MedicinalProductPharmaceutical<'a>),
    ResourceMedicinalProductUndesirableEffect(MedicinalProductUndesirableEffect<'a>),
    ResourceMessageDefinition(MessageDefinition<'a>),
    ResourceMessageHeader(MessageHeader<'a>),
    ResourceMolecularSequence(MolecularSequence<'a>),
    ResourceNamingSystem(NamingSystem<'a>),
    ResourceNutritionOrder(NutritionOrder<'a>),
    ResourceObservation(Observation<'a>),
    ResourceObservationDefinition(ObservationDefinition<'a>),
    ResourceOperationDefinition(OperationDefinition<'a>),
    ResourceOperationOutcome(OperationOutcome<'a>),
    ResourceOrganization(Organization<'a>),
    ResourceOrganizationAffiliation(OrganizationAffiliation<'a>),
    ResourceParameters(Parameters<'a>),
    ResourcePatient(Patient<'a>),
    ResourcePaymentNotice(PaymentNotice<'a>),
    ResourcePaymentReconciliation(PaymentReconciliation<'a>),
    ResourcePerson(Person<'a>),
    ResourcePlanDefinition(PlanDefinition<'a>),
    ResourcePractitioner(Practitioner<'a>),
    ResourcePractitionerRole(PractitionerRole<'a>),
    ResourceProcedure(Procedure<'a>),
    ResourceProvenance(Provenance<'a>),
    ResourceQuestionnaire(Questionnaire<'a>),
    ResourceQuestionnaireResponse(QuestionnaireResponse<'a>),
    ResourceRelatedPerson(RelatedPerson<'a>),
    ResourceRequestGroup(RequestGroup<'a>),
    ResourceResearchDefinition(ResearchDefinition<'a>),
    ResourceResearchElementDefinition(ResearchElementDefinition<'a>),
    ResourceResearchStudy(ResearchStudy<'a>),
    ResourceResearchSubject(ResearchSubject<'a>),
    ResourceRiskAssessment(RiskAssessment<'a>),
    ResourceRiskEvidenceSynthesis(RiskEvidenceSynthesis<'a>),
    ResourceSchedule(Schedule<'a>),
    ResourceSearchParameter(SearchParameter<'a>),
    ResourceServiceRequest(ServiceRequest<'a>),
    ResourceSlot(Slot<'a>),
    ResourceSpecimen(Specimen<'a>),
    ResourceSpecimenDefinition(SpecimenDefinition<'a>),
    ResourceStructureDefinition(StructureDefinition<'a>),
    ResourceStructureMap(StructureMap<'a>),
    ResourceSubscription(Subscription<'a>),
    ResourceSubstance(Substance<'a>),
    ResourceSubstanceNucleicAcid(SubstanceNucleicAcid<'a>),
    ResourceSubstancePolymer(SubstancePolymer<'a>),
    ResourceSubstanceProtein(SubstanceProtein<'a>),
    ResourceSubstanceReferenceInformation(SubstanceReferenceInformation<'a>),
    ResourceSubstanceSourceMaterial(SubstanceSourceMaterial<'a>),
    ResourceSubstanceSpecification(SubstanceSpecification<'a>),
    ResourceSupplyDelivery(SupplyDelivery<'a>),
    ResourceSupplyRequest(SupplyRequest<'a>),
    ResourceTask(Task<'a>),
    ResourceTerminologyCapabilities(TerminologyCapabilities<'a>),
    ResourceTestReport(TestReport<'a>),
    ResourceTestScript(TestScript<'a>),
    ResourceValueSet(ValueSet<'a>),
    ResourceVerificationResult(VerificationResult<'a>),
    ResourceVisionPrescription(VisionPrescription<'a>),
}

impl ResourceList<'_> {
    pub fn new(value: &Value) -> ResourceList {
        ResourceList {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    pub fn resource(&self) -> Option<ResourceListEnum> {
        let fhir_type = self.value["resourceType"].as_str().unwrap();
        match fhir_type {
            "Account" => Some(ResourceListEnum::ResourceAccount(Account {
                value: self.value.clone(),
            })),
            "ActivityDefinition" => Some(ResourceListEnum::ResourceActivityDefinition(
                ActivityDefinition {
                    value: self.value.clone(),
                },
            )),
            "AdverseEvent" => Some(ResourceListEnum::ResourceAdverseEvent(AdverseEvent {
                value: self.value.clone(),
            })),
            "AllergyIntolerance" => Some(ResourceListEnum::ResourceAllergyIntolerance(
                AllergyIntolerance {
                    value: self.value.clone(),
                },
            )),
            "Appointment" => Some(ResourceListEnum::ResourceAppointment(Appointment {
                value: self.value.clone(),
            })),
            "AppointmentResponse" => Some(ResourceListEnum::ResourceAppointmentResponse(
                AppointmentResponse {
                    value: self.value.clone(),
                },
            )),
            "AuditEvent" => Some(ResourceListEnum::ResourceAuditEvent(AuditEvent {
                value: self.value.clone(),
            })),
            "Basic" => Some(ResourceListEnum::ResourceBasic(Basic {
                value: self.value.clone(),
            })),
            "Binary" => Some(ResourceListEnum::ResourceBinary(Binary {
                value: self.value.clone(),
            })),
            "BiologicallyDerivedProduct" => Some(
                ResourceListEnum::ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct {
                    value: self.value.clone(),
                }),
            ),
            "BodyStructure" => Some(ResourceListEnum::ResourceBodyStructure(BodyStructure {
                value: self.value.clone(),
            })),
            "Bundle" => Some(ResourceListEnum::ResourceBundle(Bundle {
                value: self.value.clone(),
            })),
            "CapabilityStatement" => Some(ResourceListEnum::ResourceCapabilityStatement(
                CapabilityStatement {
                    value: self.value.clone(),
                },
            )),
            "CarePlan" => Some(ResourceListEnum::ResourceCarePlan(CarePlan {
                value: self.value.clone(),
            })),
            "CareTeam" => Some(ResourceListEnum::ResourceCareTeam(CareTeam {
                value: self.value.clone(),
            })),
            "CatalogEntry" => Some(ResourceListEnum::ResourceCatalogEntry(CatalogEntry {
                value: self.value.clone(),
            })),
            "ChargeItem" => Some(ResourceListEnum::ResourceChargeItem(ChargeItem {
                value: self.value.clone(),
            })),
            "ChargeItemDefinition" => Some(ResourceListEnum::ResourceChargeItemDefinition(
                ChargeItemDefinition {
                    value: self.value.clone(),
                },
            )),
            "Claim" => Some(ResourceListEnum::ResourceClaim(Claim {
                value: self.value.clone(),
            })),
            "ClaimResponse" => Some(ResourceListEnum::ResourceClaimResponse(ClaimResponse {
                value: self.value.clone(),
            })),
            "ClinicalImpression" => Some(ResourceListEnum::ResourceClinicalImpression(
                ClinicalImpression {
                    value: self.value.clone(),
                },
            )),
            "CodeSystem" => Some(ResourceListEnum::ResourceCodeSystem(CodeSystem {
                value: self.value.clone(),
            })),
            "Communication" => Some(ResourceListEnum::ResourceCommunication(Communication {
                value: self.value.clone(),
            })),
            "CommunicationRequest" => Some(ResourceListEnum::ResourceCommunicationRequest(
                CommunicationRequest {
                    value: self.value.clone(),
                },
            )),
            "CompartmentDefinition" => Some(ResourceListEnum::ResourceCompartmentDefinition(
                CompartmentDefinition {
                    value: self.value.clone(),
                },
            )),
            "Composition" => Some(ResourceListEnum::ResourceComposition(Composition {
                value: self.value.clone(),
            })),
            "ConceptMap" => Some(ResourceListEnum::ResourceConceptMap(ConceptMap {
                value: self.value.clone(),
            })),
            "Condition" => Some(ResourceListEnum::ResourceCondition(Condition {
                value: self.value.clone(),
            })),
            "Consent" => Some(ResourceListEnum::ResourceConsent(Consent {
                value: self.value.clone(),
            })),
            "Contract" => Some(ResourceListEnum::ResourceContract(Contract {
                value: self.value.clone(),
            })),
            "Coverage" => Some(ResourceListEnum::ResourceCoverage(Coverage {
                value: self.value.clone(),
            })),
            "CoverageEligibilityRequest" => Some(
                ResourceListEnum::ResourceCoverageEligibilityRequest(CoverageEligibilityRequest {
                    value: self.value.clone(),
                }),
            ),
            "CoverageEligibilityResponse" => {
                Some(ResourceListEnum::ResourceCoverageEligibilityResponse(
                    CoverageEligibilityResponse {
                        value: self.value.clone(),
                    },
                ))
            }
            "DetectedIssue" => Some(ResourceListEnum::ResourceDetectedIssue(DetectedIssue {
                value: self.value.clone(),
            })),
            "Device" => Some(ResourceListEnum::ResourceDevice(Device {
                value: self.value.clone(),
            })),
            "DeviceDefinition" => Some(ResourceListEnum::ResourceDeviceDefinition(
                DeviceDefinition {
                    value: self.value.clone(),
                },
            )),
            "DeviceMetric" => Some(ResourceListEnum::ResourceDeviceMetric(DeviceMetric {
                value: self.value.clone(),
            })),
            "DeviceRequest" => Some(ResourceListEnum::ResourceDeviceRequest(DeviceRequest {
                value: self.value.clone(),
            })),
            "DeviceUseStatement" => Some(ResourceListEnum::ResourceDeviceUseStatement(
                DeviceUseStatement {
                    value: self.value.clone(),
                },
            )),
            "DiagnosticReport" => Some(ResourceListEnum::ResourceDiagnosticReport(
                DiagnosticReport {
                    value: self.value.clone(),
                },
            )),
            "DocumentManifest" => Some(ResourceListEnum::ResourceDocumentManifest(
                DocumentManifest {
                    value: self.value.clone(),
                },
            )),
            "DocumentReference" => Some(ResourceListEnum::ResourceDocumentReference(
                DocumentReference {
                    value: self.value.clone(),
                },
            )),
            "EffectEvidenceSynthesis" => Some(ResourceListEnum::ResourceEffectEvidenceSynthesis(
                EffectEvidenceSynthesis {
                    value: self.value.clone(),
                },
            )),
            "Encounter" => Some(ResourceListEnum::ResourceEncounter(Encounter {
                value: self.value.clone(),
            })),
            "Endpoint" => Some(ResourceListEnum::ResourceEndpoint(Endpoint {
                value: self.value.clone(),
            })),
            "EnrollmentRequest" => Some(ResourceListEnum::ResourceEnrollmentRequest(
                EnrollmentRequest {
                    value: self.value.clone(),
                },
            )),
            "EnrollmentResponse" => Some(ResourceListEnum::ResourceEnrollmentResponse(
                EnrollmentResponse {
                    value: self.value.clone(),
                },
            )),
            "EpisodeOfCare" => Some(ResourceListEnum::ResourceEpisodeOfCare(EpisodeOfCare {
                value: self.value.clone(),
            })),
            "EventDefinition" => Some(ResourceListEnum::ResourceEventDefinition(EventDefinition {
                value: self.value.clone(),
            })),
            "Evidence" => Some(ResourceListEnum::ResourceEvidence(Evidence {
                value: self.value.clone(),
            })),
            "EvidenceVariable" => Some(ResourceListEnum::ResourceEvidenceVariable(
                EvidenceVariable {
                    value: self.value.clone(),
                },
            )),
            "ExampleScenario" => Some(ResourceListEnum::ResourceExampleScenario(ExampleScenario {
                value: self.value.clone(),
            })),
            "ExplanationOfBenefit" => Some(ResourceListEnum::ResourceExplanationOfBenefit(
                ExplanationOfBenefit {
                    value: self.value.clone(),
                },
            )),
            "FamilyMemberHistory" => Some(ResourceListEnum::ResourceFamilyMemberHistory(
                FamilyMemberHistory {
                    value: self.value.clone(),
                },
            )),
            "Flag" => Some(ResourceListEnum::ResourceFlag(Flag {
                value: self.value.clone(),
            })),
            "Goal" => Some(ResourceListEnum::ResourceGoal(Goal {
                value: self.value.clone(),
            })),
            "GraphDefinition" => Some(ResourceListEnum::ResourceGraphDefinition(GraphDefinition {
                value: self.value.clone(),
            })),
            "Group" => Some(ResourceListEnum::ResourceGroup(Group {
                value: self.value.clone(),
            })),
            "GuidanceResponse" => Some(ResourceListEnum::ResourceGuidanceResponse(
                GuidanceResponse {
                    value: self.value.clone(),
                },
            )),
            "HealthcareService" => Some(ResourceListEnum::ResourceHealthcareService(
                HealthcareService {
                    value: self.value.clone(),
                },
            )),
            "ImagingStudy" => Some(ResourceListEnum::ResourceImagingStudy(ImagingStudy {
                value: self.value.clone(),
            })),
            "Immunization" => Some(ResourceListEnum::ResourceImmunization(Immunization {
                value: self.value.clone(),
            })),
            "ImmunizationEvaluation" => Some(ResourceListEnum::ResourceImmunizationEvaluation(
                ImmunizationEvaluation {
                    value: self.value.clone(),
                },
            )),
            "ImmunizationRecommendation" => Some(
                ResourceListEnum::ResourceImmunizationRecommendation(ImmunizationRecommendation {
                    value: self.value.clone(),
                }),
            ),
            "ImplementationGuide" => Some(ResourceListEnum::ResourceImplementationGuide(
                ImplementationGuide {
                    value: self.value.clone(),
                },
            )),
            "InsurancePlan" => Some(ResourceListEnum::ResourceInsurancePlan(InsurancePlan {
                value: self.value.clone(),
            })),
            "Invoice" => Some(ResourceListEnum::ResourceInvoice(Invoice {
                value: self.value.clone(),
            })),
            "Library" => Some(ResourceListEnum::ResourceLibrary(Library {
                value: self.value.clone(),
            })),
            "Linkage" => Some(ResourceListEnum::ResourceLinkage(Linkage {
                value: self.value.clone(),
            })),
            "List" => Some(ResourceListEnum::ResourceList(List {
                value: self.value.clone(),
            })),
            "Location" => Some(ResourceListEnum::ResourceLocation(Location {
                value: self.value.clone(),
            })),
            "Measure" => Some(ResourceListEnum::ResourceMeasure(Measure {
                value: self.value.clone(),
            })),
            "MeasureReport" => Some(ResourceListEnum::ResourceMeasureReport(MeasureReport {
                value: self.value.clone(),
            })),
            "Media" => Some(ResourceListEnum::ResourceMedia(Media {
                value: self.value.clone(),
            })),
            "Medication" => Some(ResourceListEnum::ResourceMedication(Medication {
                value: self.value.clone(),
            })),
            "MedicationAdministration" => Some(ResourceListEnum::ResourceMedicationAdministration(
                MedicationAdministration {
                    value: self.value.clone(),
                },
            )),
            "MedicationDispense" => Some(ResourceListEnum::ResourceMedicationDispense(
                MedicationDispense {
                    value: self.value.clone(),
                },
            )),
            "MedicationKnowledge" => Some(ResourceListEnum::ResourceMedicationKnowledge(
                MedicationKnowledge {
                    value: self.value.clone(),
                },
            )),
            "MedicationRequest" => Some(ResourceListEnum::ResourceMedicationRequest(
                MedicationRequest {
                    value: self.value.clone(),
                },
            )),
            "MedicationStatement" => Some(ResourceListEnum::ResourceMedicationStatement(
                MedicationStatement {
                    value: self.value.clone(),
                },
            )),
            "MedicinalProduct" => Some(ResourceListEnum::ResourceMedicinalProduct(
                MedicinalProduct {
                    value: self.value.clone(),
                },
            )),
            "MedicinalProductAuthorization" => {
                Some(ResourceListEnum::ResourceMedicinalProductAuthorization(
                    MedicinalProductAuthorization {
                        value: self.value.clone(),
                    },
                ))
            }
            "MedicinalProductContraindication" => {
                Some(ResourceListEnum::ResourceMedicinalProductContraindication(
                    MedicinalProductContraindication {
                        value: self.value.clone(),
                    },
                ))
            }
            "MedicinalProductIndication" => Some(
                ResourceListEnum::ResourceMedicinalProductIndication(MedicinalProductIndication {
                    value: self.value.clone(),
                }),
            ),
            "MedicinalProductIngredient" => Some(
                ResourceListEnum::ResourceMedicinalProductIngredient(MedicinalProductIngredient {
                    value: self.value.clone(),
                }),
            ),
            "MedicinalProductInteraction" => {
                Some(ResourceListEnum::ResourceMedicinalProductInteraction(
                    MedicinalProductInteraction {
                        value: self.value.clone(),
                    },
                ))
            }
            "MedicinalProductManufactured" => {
                Some(ResourceListEnum::ResourceMedicinalProductManufactured(
                    MedicinalProductManufactured {
                        value: self.value.clone(),
                    },
                ))
            }
            "MedicinalProductPackaged" => Some(ResourceListEnum::ResourceMedicinalProductPackaged(
                MedicinalProductPackaged {
                    value: self.value.clone(),
                },
            )),
            "MedicinalProductPharmaceutical" => {
                Some(ResourceListEnum::ResourceMedicinalProductPharmaceutical(
                    MedicinalProductPharmaceutical {
                        value: self.value.clone(),
                    },
                ))
            }
            "MedicinalProductUndesirableEffect" => {
                Some(ResourceListEnum::ResourceMedicinalProductUndesirableEffect(
                    MedicinalProductUndesirableEffect {
                        value: self.value.clone(),
                    },
                ))
            }
            "MessageDefinition" => Some(ResourceListEnum::ResourceMessageDefinition(
                MessageDefinition {
                    value: self.value.clone(),
                },
            )),
            "MessageHeader" => Some(ResourceListEnum::ResourceMessageHeader(MessageHeader {
                value: self.value.clone(),
            })),
            "MolecularSequence" => Some(ResourceListEnum::ResourceMolecularSequence(
                MolecularSequence {
                    value: self.value.clone(),
                },
            )),
            "NamingSystem" => Some(ResourceListEnum::ResourceNamingSystem(NamingSystem {
                value: self.value.clone(),
            })),
            "NutritionOrder" => Some(ResourceListEnum::ResourceNutritionOrder(NutritionOrder {
                value: self.value.clone(),
            })),
            "Observation" => Some(ResourceListEnum::ResourceObservation(Observation {
                value: self.value.clone(),
            })),
            "ObservationDefinition" => Some(ResourceListEnum::ResourceObservationDefinition(
                ObservationDefinition {
                    value: self.value.clone(),
                },
            )),
            "OperationDefinition" => Some(ResourceListEnum::ResourceOperationDefinition(
                OperationDefinition {
                    value: self.value.clone(),
                },
            )),
            "OperationOutcome" => Some(ResourceListEnum::ResourceOperationOutcome(
                OperationOutcome {
                    value: self.value.clone(),
                },
            )),
            "Organization" => Some(ResourceListEnum::ResourceOrganization(Organization {
                value: self.value.clone(),
            })),
            "OrganizationAffiliation" => Some(ResourceListEnum::ResourceOrganizationAffiliation(
                OrganizationAffiliation {
                    value: self.value.clone(),
                },
            )),
            "Parameters" => Some(ResourceListEnum::ResourceParameters(Parameters {
                value: self.value.clone(),
            })),
            "Patient" => Some(ResourceListEnum::ResourcePatient(Patient {
                value: self.value.clone(),
            })),
            "PaymentNotice" => Some(ResourceListEnum::ResourcePaymentNotice(PaymentNotice {
                value: self.value.clone(),
            })),
            "PaymentReconciliation" => Some(ResourceListEnum::ResourcePaymentReconciliation(
                PaymentReconciliation {
                    value: self.value.clone(),
                },
            )),
            "Person" => Some(ResourceListEnum::ResourcePerson(Person {
                value: self.value.clone(),
            })),
            "PlanDefinition" => Some(ResourceListEnum::ResourcePlanDefinition(PlanDefinition {
                value: self.value.clone(),
            })),
            "Practitioner" => Some(ResourceListEnum::ResourcePractitioner(Practitioner {
                value: self.value.clone(),
            })),
            "PractitionerRole" => Some(ResourceListEnum::ResourcePractitionerRole(
                PractitionerRole {
                    value: self.value.clone(),
                },
            )),
            "Procedure" => Some(ResourceListEnum::ResourceProcedure(Procedure {
                value: self.value.clone(),
            })),
            "Provenance" => Some(ResourceListEnum::ResourceProvenance(Provenance {
                value: self.value.clone(),
            })),
            "Questionnaire" => Some(ResourceListEnum::ResourceQuestionnaire(Questionnaire {
                value: self.value.clone(),
            })),
            "QuestionnaireResponse" => Some(ResourceListEnum::ResourceQuestionnaireResponse(
                QuestionnaireResponse {
                    value: self.value.clone(),
                },
            )),
            "RelatedPerson" => Some(ResourceListEnum::ResourceRelatedPerson(RelatedPerson {
                value: self.value.clone(),
            })),
            "RequestGroup" => Some(ResourceListEnum::ResourceRequestGroup(RequestGroup {
                value: self.value.clone(),
            })),
            "ResearchDefinition" => Some(ResourceListEnum::ResourceResearchDefinition(
                ResearchDefinition {
                    value: self.value.clone(),
                },
            )),
            "ResearchElementDefinition" => Some(
                ResourceListEnum::ResourceResearchElementDefinition(ResearchElementDefinition {
                    value: self.value.clone(),
                }),
            ),
            "ResearchStudy" => Some(ResourceListEnum::ResourceResearchStudy(ResearchStudy {
                value: self.value.clone(),
            })),
            "ResearchSubject" => Some(ResourceListEnum::ResourceResearchSubject(ResearchSubject {
                value: self.value.clone(),
            })),
            "RiskAssessment" => Some(ResourceListEnum::ResourceRiskAssessment(RiskAssessment {
                value: self.value.clone(),
            })),
            "RiskEvidenceSynthesis" => Some(ResourceListEnum::ResourceRiskEvidenceSynthesis(
                RiskEvidenceSynthesis {
                    value: self.value.clone(),
                },
            )),
            "Schedule" => Some(ResourceListEnum::ResourceSchedule(Schedule {
                value: self.value.clone(),
            })),
            "SearchParameter" => Some(ResourceListEnum::ResourceSearchParameter(SearchParameter {
                value: self.value.clone(),
            })),
            "ServiceRequest" => Some(ResourceListEnum::ResourceServiceRequest(ServiceRequest {
                value: self.value.clone(),
            })),
            "Slot" => Some(ResourceListEnum::ResourceSlot(Slot {
                value: self.value.clone(),
            })),
            "Specimen" => Some(ResourceListEnum::ResourceSpecimen(Specimen {
                value: self.value.clone(),
            })),
            "SpecimenDefinition" => Some(ResourceListEnum::ResourceSpecimenDefinition(
                SpecimenDefinition {
                    value: self.value.clone(),
                },
            )),
            "StructureDefinition" => Some(ResourceListEnum::ResourceStructureDefinition(
                StructureDefinition {
                    value: self.value.clone(),
                },
            )),
            "StructureMap" => Some(ResourceListEnum::ResourceStructureMap(StructureMap {
                value: self.value.clone(),
            })),
            "Subscription" => Some(ResourceListEnum::ResourceSubscription(Subscription {
                value: self.value.clone(),
            })),
            "Substance" => Some(ResourceListEnum::ResourceSubstance(Substance {
                value: self.value.clone(),
            })),
            "SubstanceNucleicAcid" => Some(ResourceListEnum::ResourceSubstanceNucleicAcid(
                SubstanceNucleicAcid {
                    value: self.value.clone(),
                },
            )),
            "SubstancePolymer" => Some(ResourceListEnum::ResourceSubstancePolymer(
                SubstancePolymer {
                    value: self.value.clone(),
                },
            )),
            "SubstanceProtein" => Some(ResourceListEnum::ResourceSubstanceProtein(
                SubstanceProtein {
                    value: self.value.clone(),
                },
            )),
            "SubstanceReferenceInformation" => {
                Some(ResourceListEnum::ResourceSubstanceReferenceInformation(
                    SubstanceReferenceInformation {
                        value: self.value.clone(),
                    },
                ))
            }
            "SubstanceSourceMaterial" => Some(ResourceListEnum::ResourceSubstanceSourceMaterial(
                SubstanceSourceMaterial {
                    value: self.value.clone(),
                },
            )),
            "SubstanceSpecification" => Some(ResourceListEnum::ResourceSubstanceSpecification(
                SubstanceSpecification {
                    value: self.value.clone(),
                },
            )),
            "SupplyDelivery" => Some(ResourceListEnum::ResourceSupplyDelivery(SupplyDelivery {
                value: self.value.clone(),
            })),
            "SupplyRequest" => Some(ResourceListEnum::ResourceSupplyRequest(SupplyRequest {
                value: self.value.clone(),
            })),
            "Task" => Some(ResourceListEnum::ResourceTask(Task {
                value: self.value.clone(),
            })),
            "TerminologyCapabilities" => Some(ResourceListEnum::ResourceTerminologyCapabilities(
                TerminologyCapabilities {
                    value: self.value.clone(),
                },
            )),
            "TestReport" => Some(ResourceListEnum::ResourceTestReport(TestReport {
                value: self.value.clone(),
            })),
            "TestScript" => Some(ResourceListEnum::ResourceTestScript(TestScript {
                value: self.value.clone(),
            })),
            "ValueSet" => Some(ResourceListEnum::ResourceValueSet(ValueSet {
                value: self.value.clone(),
            })),
            "VerificationResult" => Some(ResourceListEnum::ResourceVerificationResult(
                VerificationResult {
                    value: self.value.clone(),
                },
            )),
            "VisionPrescription" => Some(ResourceListEnum::ResourceVisionPrescription(
                VisionPrescription {
                    value: self.value.clone(),
                },
            )),
            _ => None,
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(resource) = self.resource() {
            match resource {
                ResourceListEnum::ResourceAccount(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceActivityDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAdverseEvent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAllergyIntolerance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAppointment(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAppointmentResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceAuditEvent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBasic(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBinary(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBiologicallyDerivedProduct(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBodyStructure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceBundle(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCapabilityStatement(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCarePlan(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCareTeam(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCatalogEntry(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceChargeItem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceChargeItemDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClaim(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClaimResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceClinicalImpression(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCodeSystem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCommunication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCommunicationRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCompartmentDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceComposition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConceptMap(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCondition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceConsent(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceContract(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverageEligibilityRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceCoverageEligibilityResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDetectedIssue(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDevice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceMetric(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDeviceUseStatement(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDiagnosticReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDocumentManifest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceDocumentReference(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEffectEvidenceSynthesis(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEncounter(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEndpoint(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEnrollmentRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEnrollmentResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEpisodeOfCare(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEventDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEvidence(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceEvidenceVariable(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceExampleScenario(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceExplanationOfBenefit(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceFamilyMemberHistory(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceFlag(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGoal(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGraphDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGroup(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceGuidanceResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceHealthcareService(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImagingStudy(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunizationEvaluation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImmunizationRecommendation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceImplementationGuide(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceInsurancePlan(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceInvoice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLibrary(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLinkage(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceList(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceLocation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMeasure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMeasureReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedia(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationAdministration(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationDispense(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationKnowledge(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicationStatement(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProduct(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductAuthorization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductContraindication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductIndication(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductIngredient(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductInteraction(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductManufactured(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductPackaged(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductPharmaceutical(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMedicinalProductUndesirableEffect(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMessageDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMessageHeader(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceMolecularSequence(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNamingSystem(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceNutritionOrder(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceObservation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceObservationDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOperationDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOperationOutcome(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOrganization(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceOrganizationAffiliation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceParameters(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePatient(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePaymentNotice(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePaymentReconciliation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePerson(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePlanDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePractitioner(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourcePractitionerRole(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceProcedure(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceProvenance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceQuestionnaire(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceQuestionnaireResponse(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRelatedPerson(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRequestGroup(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchElementDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchStudy(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceResearchSubject(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRiskAssessment(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceRiskEvidenceSynthesis(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSchedule(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSearchParameter(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceServiceRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSlot(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSpecimen(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSpecimenDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceStructureDefinition(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceStructureMap(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubscription(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstance(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceNucleicAcid(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstancePolymer(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceProtein(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceReferenceInformation(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceSourceMaterial(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSubstanceSpecification(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSupplyDelivery(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceSupplyRequest(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTask(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTerminologyCapabilities(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTestReport(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceTestScript(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceValueSet(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceVerificationResult(val) => {
                    return val.validate();
                }
                ResourceListEnum::ResourceVisionPrescription(val) => {
                    return val.validate();
                }
            }
        }
        return false;
    }
}

//! One bounded requirement evaluator serves manual checks and automatic derivation.
use super::speaker::*;
use crate::phonology::*;
use crate::validation::ValidationErrors;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Requirement {
    Lips,
    LipRounding,
    DentalContact,
    LabiodentalContact,
    AlveolarRidge,
    HardPalate,
    SoftPalate,
    Uvula,
    Epiglottis,
    TongueFront,
    TongueTip,
    TongueBlade,
    AgileTongueTip,
    TongueBody,
    AgileTongueBody,
    TongueRoot,
    AgileTongueRoot,
    LateralControl,
    RetroflexControl,
    PulmonicAirflow,
    ClickAirflow,
    EjectiveAirflow,
    ImplosiveAirflow,
    OralAirflow,
    NasalAirflow,
    MixedAirflow,
    GlottalClosure,
    Voicing(Voicing),
    Phonation(Phonation),
    Aspiration,
    VowelHeight(TongueHeight),
    VowelPosition(TonguePosition),
    Trill(ArticulationPlace),
    Tap(ArticulationPlace),
    AffricateCoordination,
    DoubleArticulation,
    EpiglottalControl,
    Precision(ArticulationPlace),
    VowelPrecision,
    RhoticControl,
    ReleaseControl,
    SyllabicManner(Manner),
    VelarOrPharyngealization,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportGap {
    /// Missing anatomy input, not an assertion that a tooth shape is incapable.
    UnspecifiedContact(Requirement),
    /// No validated requirement model is defined for this non-charted combination.
    ArticulationCombination {
        place: ArticulationPlace,
        manner: Manner,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductionAssessment {
    Possible,
    Impossible(Vec<Requirement>),
    Unsupported(Vec<SupportGap>),
}
#[derive(Debug, Clone)]
pub struct ProductionRequirements {
    all_of: Vec<Requirement>,
    unsupported: Vec<SupportGap>,
}
impl ProductionRequirements {
    pub fn all_of(&self) -> &[Requirement] {
        &self.all_of
    }
    pub fn unsupported(&self) -> &[SupportGap] {
        &self.unsupported
    }
    pub fn for_sound(sound: &SoundDefinition) -> Self {
        use Requirement::*;
        let mut r = Self {
            all_of: vec![],
            unsupported: vec![],
        };
        match sound.features() {
            SoundFeatures::Consonant(f) => {
                r.all_of.push(match f.airflow {
                    AirflowMechanism::Pulmonic => PulmonicAirflow,
                    AirflowMechanism::Click => ClickAirflow,
                    AirflowMechanism::Ejective => EjectiveAirflow,
                    AirflowMechanism::Implosive => ImplosiveAirflow,
                });
                r.place(f.place);
                if let Some(release) = f.affricate_release {
                    r.place(release);
                }
                match f.manner {
                    Manner::Trill => r.all_of.push(Trill(f.place)),
                    Manner::TapFlap | Manner::LateralTapFlap => r.all_of.push(Tap(f.place)),
                    Manner::Affricate => r.all_of.push(AffricateCoordination),
                    _ => {}
                }
                if f.place == ArticulationPlace::Pharyngeal && f.manner == Manner::Plosive {
                    r.unsupported.push(SupportGap::ArticulationCombination {
                        place: f.place,
                        manner: f.manner,
                    });
                }
                r.all_of
                    .extend([Voicing(f.voicing), Phonation(f.phonation)]);
                if f.aspiration == crate::phonology::Aspiration::Aspirated {
                    r.all_of.push(Aspiration);
                }
                r.all_of.push(if f.manner == Manner::Nasal {
                    NasalAirflow
                } else {
                    OralAirflow
                });
                if matches!(
                    f.manner,
                    Manner::LateralApproximant
                        | Manner::LateralFricative
                        | Manner::LateralTapFlap
                        | Manner::LateralPlosive
                ) {
                    r.all_of.push(LateralControl);
                }
                if f.place == ArticulationPlace::Glottal && f.manner == Manner::Plosive {
                    r.all_of.push(GlottalClosure);
                }
                if matches!(
                    f.place,
                    ArticulationPlace::LabialVelar | ArticulationPlace::LabialPalatal
                ) && !matches!(f.manner, Manner::Plosive | Manner::Nasal)
                {
                    r.all_of.push(LipRounding);
                }
                if f.modifiers.contains(&Modifier::Syllabic) {
                    r.all_of.push(SyllabicManner(f.manner));
                }
            }
            SoundFeatures::Vowel(f) => {
                r.all_of.extend([
                    PulmonicAirflow,
                    OralAirflow,
                    TongueBody,
                    VowelHeight(f.height),
                    VowelPosition(f.position),
                    Voicing(f.voicing),
                    Phonation(f.phonation),
                ]);
                if f.roundness == Roundness::Rounded {
                    r.all_of.push(LipRounding);
                }
            }
        }
        for modifier in sound.modifiers() {
            match modifier {
                Modifier::Nasalized => {
                    if !matches!(sound.features(), SoundFeatures::Consonant(f) if f.manner == Manner::Nasal)
                    {
                        r.all_of.push(MixedAirflow);
                    }
                }
                Modifier::Labialized | Modifier::MoreRounded | Modifier::LessRounded => {
                    r.all_of.push(LipRounding)
                }
                Modifier::Palatalized => r.all_of.extend([HardPalate, AgileTongueBody]),
                Modifier::Velarized => r.all_of.extend([SoftPalate, AgileTongueBody]),
                Modifier::Pharyngealized => r.all_of.push(TongueRoot),
                Modifier::VelarizedOrPharyngealized => r.all_of.push(VelarOrPharyngealization),
                Modifier::AdvancedTongueRoot | Modifier::RetractedTongueRoot => {
                    r.all_of.push(AgileTongueRoot)
                }
                Modifier::Dental => {
                    if matches!(sound.features(), SoundFeatures::Consonant(f) if f.place == ArticulationPlace::Labiodental)
                    {
                        r.all_of.push(LabiodentalContact);
                    } else {
                        r.all_of.extend([DentalContact, TongueFront]);
                    }
                }
                Modifier::Linguolabial => r.all_of.extend([Lips, TongueFront]),
                Modifier::Apical => r.all_of.push(TongueTip),
                Modifier::Laminal => r.all_of.push(TongueBlade),
                Modifier::Rhoticity => r.all_of.push(RhoticControl),
                Modifier::NasalRelease => r.all_of.extend([MixedAirflow, ReleaseControl]),
                Modifier::LateralRelease => r.all_of.extend([LateralControl, ReleaseControl]),
                Modifier::NoAudibleRelease => r.all_of.push(ReleaseControl),
                Modifier::Syllabic | Modifier::NonSyllabic => {}
                Modifier::Raised
                | Modifier::Lowered
                | Modifier::Advanced
                | Modifier::Retracted
                | Modifier::Centralized
                | Modifier::MidCentralized => {
                    r.all_of.push(match sound.features() {
                        SoundFeatures::Consonant(f) => Precision(f.place),
                        SoundFeatures::Vowel(_) => VowelPrecision,
                    });
                }
            }
        }
        r
    }
    fn place(&mut self, place: ArticulationPlace) {
        use ArticulationPlace as P;
        use Requirement::*;
        match place {
            P::Bilabial => self.all_of.push(Lips),
            P::Labiodental => self.all_of.extend([Lips, LabiodentalContact]),
            P::Linguolabial => self.all_of.extend([Lips, TongueFront]),
            P::Dental => self.all_of.extend([DentalContact, TongueFront]),
            P::Alveolar => self.all_of.extend([AlveolarRidge, TongueFront]),
            P::Postalveolar => self.all_of.extend([AlveolarRidge, HardPalate, TongueFront]),
            P::AlveoloPalatal => {
                self.all_of
                    .extend([AlveolarRidge, HardPalate, TongueFront, AgileTongueBody])
            }
            P::Retroflex => self.all_of.extend([AgileTongueTip, RetroflexControl]),
            P::Palatal => self.all_of.extend([HardPalate, AgileTongueBody]),
            P::Velar => self.all_of.extend([SoftPalate, AgileTongueBody]),
            P::Uvular => self.all_of.extend([Uvula, AgileTongueBody]),
            P::Pharyngeal => self.all_of.push(TongueRoot),
            P::Epiglottal => self.all_of.extend([Epiglottis, EpiglottalControl]),
            P::Glottal => {}
            P::LabialVelar => {
                self.all_of
                    .extend([Lips, SoftPalate, AgileTongueBody, DoubleArticulation])
            }
            P::LabialPalatal => {
                self.all_of
                    .extend([Lips, HardPalate, AgileTongueBody, DoubleArticulation])
            }
            P::PostalveolarVelar => self.all_of.extend([
                AlveolarRidge,
                HardPalate,
                SoftPalate,
                TongueFront,
                AgileTongueBody,
                DoubleArticulation,
            ]),
        }
    }
    fn assess(&self, a: &SpeakerAnatomy) -> ProductionAssessment {
        use Requirement::*;
        let mut missing = vec![];
        let mut unsupported = self.unsupported.clone();
        for requirement in &self.all_of {
            let satisfied = match *requirement {
                Lips => a.lips != LipControl::None,
                LipRounding => a.lips == LipControl::Flexible,
                DentalContact | LabiodentalContact => {
                    let contact = if *requirement == DentalContact {
                        a.oral_cavity.dental_contact
                    } else {
                        a.oral_cavity.labiodental_contact
                    };
                    match contact {
                        ContactCapability::Available => true,
                        ContactCapability::Unavailable => false,
                        ContactCapability::Unspecified => {
                            unsupported.push(SupportGap::UnspecifiedContact(*requirement));
                            true
                        }
                    }
                }
                AlveolarRidge => a.oral_cavity.has_alveolar_ridge,
                HardPalate => a.oral_cavity.has_hard_palate,
                SoftPalate => a.oral_cavity.has_soft_palate,
                Uvula => a.oral_cavity.has_uvula,
                Epiglottis => a.oral_cavity.has_epiglottis,
                TongueFront => {
                    a.tongue.tip != TonguePartControl::None
                        || a.tongue.blade != TonguePartControl::None
                }
                TongueTip => a.tongue.tip != TonguePartControl::None,
                TongueBlade => a.tongue.blade != TonguePartControl::None,
                AgileTongueTip => a.tongue.tip == TonguePartControl::Agile,
                TongueBody => a.tongue.body != TonguePartControl::None,
                AgileTongueBody => a.tongue.body == TonguePartControl::Agile,
                TongueRoot => a.tongue.root != TonguePartControl::None,
                AgileTongueRoot => a.tongue.root == TonguePartControl::Agile,
                LateralControl => a.tongue.can_perform_lateral_release,
                RetroflexControl => a.tongue.can_curl_for_retroflex,
                PulmonicAirflow => a.airflow.pulmonic != PulmonicControl::None,
                ClickAirflow => {
                    a.airflow.can_produce_clicks && a.tongue.body == TonguePartControl::Agile
                }
                EjectiveAirflow => a.larynx.can_produce_ejectives,
                ImplosiveAirflow => a.larynx.can_produce_implosives,
                OralAirflow => a.airflow.velic_port != VelicPortControl::NasalOnly,
                NasalAirflow => a.airflow.velic_port != VelicPortControl::OralOnly,
                MixedAirflow => a.airflow.velic_port == VelicPortControl::Controllable,
                GlottalClosure => a.larynx.can_close_glottis,
                Voicing(crate::phonology::Voicing::Voiced) => a.larynx.voicing.voiced,
                Voicing(crate::phonology::Voicing::Voiceless) => a.larynx.voicing.voiceless,
                Phonation(crate::phonology::Phonation::Modal) => true,
                Phonation(crate::phonology::Phonation::Breathy) => {
                    a.larynx.can_produce_breathy_voice
                }
                Phonation(crate::phonology::Phonation::Creaky) => a.larynx.can_produce_creaky_voice,
                Aspiration => a.larynx.can_aspirate,
                VowelHeight(height) => a.tongue.vowel_heights.contains(&height),
                VowelPosition(position) => a.tongue.vowel_positions.contains(&position),
                Trill(place) => a.articulation.trills.contains(&place),
                Tap(place) => a.articulation.taps.contains(&place),
                AffricateCoordination => a.articulation.can_coordinate_affricates,
                DoubleArticulation => a.articulation.can_double_articulate,
                EpiglottalControl => a.articulation.can_articulate_epiglottals,
                Precision(place) => {
                    a.articulation.can_fine_adjust
                        && match place {
                            ArticulationPlace::Bilabial | ArticulationPlace::Labiodental => {
                                a.lips == LipControl::Flexible
                            }
                            ArticulationPlace::Linguolabial
                            | ArticulationPlace::Dental
                            | ArticulationPlace::Alveolar
                            | ArticulationPlace::Postalveolar
                            | ArticulationPlace::Retroflex => {
                                a.tongue.tip == TonguePartControl::Agile
                                    || a.tongue.blade == TonguePartControl::Agile
                            }
                            ArticulationPlace::Glottal | ArticulationPlace::Epiglottal => {
                                a.larynx.can_adjust_constriction
                            }
                            ArticulationPlace::Pharyngeal => {
                                a.tongue.root == TonguePartControl::Agile
                            }
                            _ => a.tongue.body == TonguePartControl::Agile,
                        }
                }
                VowelPrecision => {
                    a.articulation.can_fine_adjust && a.tongue.body == TonguePartControl::Agile
                }
                RhoticControl => a.articulation.can_rhoticize,
                ReleaseControl => a.articulation.can_control_release,
                VelarOrPharyngealization => {
                    (a.oral_cavity.has_soft_palate && a.tongue.body == TonguePartControl::Agile)
                        || a.tongue.root != TonguePartControl::None
                }
                SyllabicManner(manner) => a.articulation.syllabic_manners.contains(&manner),
            };
            if !satisfied && !missing.contains(requirement) {
                missing.push(*requirement);
            }
        }
        if !missing.is_empty() {
            ProductionAssessment::Impossible(missing)
        } else if !unsupported.is_empty() {
            ProductionAssessment::Unsupported(unsupported)
        } else {
            ProductionAssessment::Possible
        }
    }
}
impl SpeakerAnatomy {
    pub fn assess_sound(
        &self,
        sound: &SoundDefinition,
    ) -> Result<ProductionAssessment, ValidationErrors> {
        self.validate_anatomical_consistency()?;
        Ok(ProductionRequirements::for_sound(sound).assess(self))
    }
}
#[derive(Debug, Clone)]
pub struct PossibleSoundSet {
    catalog: SoundCatalog,
    assessments: BTreeMap<SoundId, ProductionAssessment>,
}
impl PossibleSoundSet {
    pub fn derive(
        catalog: &SoundCatalog,
        anatomy: &SpeakerAnatomy,
    ) -> Result<Self, ValidationErrors> {
        anatomy.validate_anatomical_consistency()?;
        let assessments = catalog
            .iter()
            .map(|sound| {
                (
                    sound.id().clone(),
                    ProductionRequirements::for_sound(sound).assess(anatomy),
                )
            })
            .collect();
        Ok(Self {
            catalog: catalog.clone(),
            assessments,
        })
    }
    pub fn assessment(&self, id: &SoundId) -> Option<&ProductionAssessment> {
        self.assessments.get(id)
    }
    pub fn possible_ids(&self) -> impl Iterator<Item = &SoundId> {
        self.assessments
            .iter()
            .filter_map(|(id, a)| (*a == ProductionAssessment::Possible).then_some(id))
    }
    pub(crate) fn matches_catalog(&self, catalog: &SoundCatalog) -> bool {
        self.catalog.iter().eq(catalog.iter())
    }
}

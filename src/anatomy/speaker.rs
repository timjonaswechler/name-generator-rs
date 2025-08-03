//! Speaker-Specific Anatomical Profiles
//!
//! Defines different speaker types with their anatomical
//! capabilities and constraints for phoneme production.

use serde::{Deserialize, Serialize};

/// Repräsentiert die vollständige anatomische Ausstattung eines Sprechers,
/// die für die Lautproduktion relevant ist. Dieses Modell trennt die passiven
/// Strukturen (wie Zähne und Gaumen) von den aktiv steuerbaren Organen (wie Zunge und Lippen).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerAnatomy {
    /// Die passiven Strukturen im Mundraum.
    pub oral_cavity: OralCavity,
    /// Die Fähigkeiten der Lippen.
    pub lips: LipControl,
    /// Die detaillierten Kontrollfähigkeiten der Zunge.
    pub tongue: TongueControl,
    /// Die Kontrollfähigkeiten des Kehlkopfes (Stimmbildung).
    pub larynx: LarynxControl,
    /// Die Kontrolle über die verschiedenen Luftstrommechanismen.
    pub airflow: AirflowControl,
}

/// Beschreibt die festen, passiven Artikulationsorte im Mundraum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OralCavity {
    /// Die Konfiguration der Zähne.
    pub teeth: TeethConfiguration,
    /// Gibt an, ob ein klar definierter Zahndamm (Alveolarfortsatz) vorhanden ist.
    /// Notwendig für alveolare und postalveolare Laute.
    pub has_alveolar_ridge: bool,
    /// Gibt an, ob ein harter Gaumen vorhanden ist.
    /// Notwendig für palatale und postalveolare Laute.
    pub has_hard_palate: bool,
    /// Gibt an, ob ein weicher Gaumen (Velum) vorhanden ist.
    /// Notwendig für velare Laute und die Steuerung des nasalen Luftstroms.
    pub has_soft_palate: bool,
    /// Gibt an, ob ein Zäpfchen (Uvula) vorhanden ist.
    /// Notwendig für uvulare Laute.
    pub has_uvula: bool,
    /// Gibt an, ob ein Kehldeckel (Epiglottis) für die Artikulation genutzt werden kann.
    /// Notwendig für epiglottale Laute.
    pub has_epiglottis: bool,
}

/// Fähigkeiten zur Steuerung der Lippen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LipControl {
    /// Keine Lippen vorhanden (z.B. Vögel). Keine labialen Laute möglich.
    None,
    /// Starre Lippen mit begrenzter Bewegung. Evtl. simple bilabiale Plosive möglich.
    Rigid,
    /// Flexible Lippen mit voller Kontrolle (z.B. Menschen). Ermöglicht bilabiale und labiodentale Laute.
    Flexible,
}

/// Detaillierte Kontrollfähigkeiten der verschiedenen Teile der Zunge.
/// Dies ist entscheidend, da die Zunge der vielseitigste Artikulator ist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TongueControl {
    /// Kontrolle über die Zungenspitze (Apex).
    /// Entscheidend für apikale Laute (einige dentale, alveolare) und Retroflexe.
    pub tip: TonguePartControl,
    /// Kontrolle über das Zungenblatt (Lamina).
    /// Entscheidend für laminale Laute (viele dentale, alveolare, postalveolare).
    pub blade: TonguePartControl,
    /// Kontrolle über den Zungenrücken (Dorsum).
    /// Entscheidend für dorsale Laute: palatal, velar, uvular.
    pub body: TonguePartControl,
    /// Kontrolle über die Zungenwurzel.
    /// Entscheidend für radikale/pharyngeale Laute.
    pub root: TonguePartControl,
    /// Fähigkeit, die Zungenspitze nach hinten zu krümmen.
    /// Notwendig für retroflexe Laute.
    pub can_curl_for_retroflex: bool,
    /// Fähigkeit, die Zungenränder zu senken, um Luft seitlich entweichen zu lassen.
    /// Notwendig für laterale Laute (z.B. [l]).
    pub can_perform_lateral_release: bool,
}

/// Der Grad der Kontrolle über einen bestimmten Teil der Zunge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TonguePartControl {
    /// Der Teil ist nicht vorhanden oder nicht beweglich.
    None,
    /// Der Teil kann nur grob bewegt werden.
    Limited,
    /// Der Teil ist agil und fein steuerbar (menschliches Niveau).
    Agile,
}

/// Beschreibt die Zahnkonfiguration, die die Artikulation beeinflusst.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeethConfiguration {
    /// Keine Zähne. Verhindert labiodentale und interdentale Laute.
    None,
    /// Spitze Reißzähne (Fangs). Könnten den Kontakt für dentale/alveolare Laute stören.
    Fangs,
    /// Flache Zähne, die den Mund nicht vollständig schließen. Begrenzte dentale Fähigkeiten.
    Flat,
    /// Menschliche Zahnkonfiguration. Erlaubt volle dentale, interdentale und labiodentale Artikulation.
    Human,
}

/// Fähigkeiten zur Steuerung des Kehlkopfes (Larynx) für die Phonation (Stimmgebung).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LarynxControl {
    /// Die Fähigkeit, die Stimmlippen schwingen zu lassen.
    pub voicing: VoicingControl,
    /// Fähigkeit, den Kehlkopf anzuheben und Druck aufzubauen.
    /// Notwendig für Ejektive.
    pub can_produce_ejectives: bool,
}

/// Grad der Stimmkontrolle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoicingControl {
    /// Keine Kontrolle über die Stimmlippen (immer stimmlos oder immer stimmhaft).
    None,
    /// Grundlegende Unterscheidung zwischen stimmhaft und stimmlos.
    Basic,
    /// Erweiterte Kontrolle, die Aspiration, Hauchstimme, Knarrstimme etc. ermöglicht.
    Advanced,
}

/// Steuerung der verschiedenen Luftstrommechanismen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirflowControl {
    /// Kontrolle über den pulmonischen Luftstrom (aus der Lunge).
    pub pulmonic: PulmonicControl,
    /// Kontrolle über den velaren Port (Durchgang zur Nasenhöhle).
    pub velic_port: VelicPortControl,
    /// Fähigkeit, Klicklaute (velarischer Luftstrom) zu erzeugen.
    /// Erfordert komplexe, unabhängige Zungenkontrolle (vorne und hinten).
    pub can_produce_clicks: bool,
}

/// Kontrolle über den Lungen-basierten Luftstrom.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PulmonicControl {
    /// Kein Lungen-basierter Luftstrom (alternative Methode).
    None,
    /// Grundlegende Kontrolle über Ausatmung für Lauterzeugung.
    Basic,
    /// Fortgeschrittene Kontrolle für komplexe Lautstärke- und Längenvariationen.
    Advanced,
}

/// Kontrolle über den Zugang zur Nasenhöhle durch Heben/Senken des weichen Gaumens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VelicPortControl {
    /// Keine Verbindung oder keine Kontrolle (immer geschlossen oder immer offen).
    None,
    /// Fähigkeit, zwischen oralem und nasalem Luftstrom zu wechseln.
    /// Setzt `OralCavity.has_soft_palate = true` voraus.
    Controllable,
}

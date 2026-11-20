use crate::phonology::macros::define_ipa_phoneme;
use crate::phonology::{Modifier, Roundness, TongueHeight, TonguePosition};
use std::borrow::Cow;

// |                | Front
// | :----------    | :----------------------------------------
// | **Close**      | `i` → `I`             • `y` → `Y`
// | **Near-close** | `ɪ` → `SMALL_CAP_I`   • `ʏ` → `SMALL_CAP_Y` |
// | **Close-mid**  | `e` → `E`             • `ø` → `O_WITH_STROKE`
// | **Mid**        | `e̞` → `MID_E`        • `ø̞` → `O_WITH_STROKE_RAISED`
// | **Open-mid**   | `ɛ` → `EPSILON`       • `œ` → `OE`
// | **Near-open**  | `æ` → `AE`            •
// | **Open**       | `a` → `A`             • `ɶ` → `SMALL_CAP_OE`

// |                | Central
// | :----------    | :----------------------------------------
// | **Close**      | `ɨ` → `I_BAR`             • `ʉ` → `U_BAR`
// | **Near-close** |
// | **Close-mid**  | `ɘ` → `REVERSED_E`        • `ɵ` → `O_BAR`
// | **Mid**        | `ə` → `SCHWA`
// | **Open-mid**   | `ɜ` → `REVERSED_EPSILON`  • `ɞ` → `CLOSED_REVERSED_EPSILON`
// | **Near-open**  | `ɐ` → `TURNED_A`
// | **Open**       | `ä` → `A_DIAERESIS`       •

// |                | Back
// | :----------    | :----------------------------------------
// | **Close**      | `ɯ` → `TURNED_M`         • `u` → `U`
// | **Near-close** |                          •`ʊ` → `UPSILON`
// | **Close-mid**  | `ɤ` → `RAMS_HORN`        • `o` → `O`
// | **Mid**        | `ɤ̞` → `MID_RAMS_HORN`   • `o̞` → `MID_O`
// | **Open-mid**   | `ʌ` → `TURNED_V`         • `ɔ` → `OPEN_O`
// | **Near-open**  |
// | **Open**       | `ɑ` → `SCRIPT_A`         • `ɒ` → `TURNED_SCRIPT_A`

define_ipa_phoneme! {
    name: I,
    id: "i",
    ipa: "i",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Y,
    id: "y",
    ipa: "y",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_I,
    id: "small_cap_i",
    ipa: "ɪ",
    vocalic:{
        position: TonguePosition::NearFront,
        height: TongueHeight::NearClose,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_Y,
    id: "small_cap_y",
    ipa: "ʏ",
    vocalic:{
        position: TonguePosition::NearFront,
        height: TongueHeight::NearClose,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: E,
    id: "e",
    ipa: "e",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: O_WITH_STROKE,
    id: "o_with_stroke",
    ipa: "ø",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: E_RAISED,
    id: "e_raised",
    ipa: "e̞",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Mid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[Modifier::Lowered])
    }
}
define_ipa_phoneme! {
    name: O_WITH_STROKE_RAISED,
    id: "o_with_stroke_raised",
    ipa: "ø̞",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Mid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[Modifier::Lowered])
    }
}
define_ipa_phoneme! {
    name: EPSILON,
    id: "epsilon",
    ipa: "ɛ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: OE,
    id: "oe",
    ipa: "œ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: AE,
    id: "ae",
    ipa: "æ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::NearOpen,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: A,
    id: "a",
    ipa: "a",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_OE,
    id: "small_cap_oe",
    ipa: "ɶ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Open,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: I_BAR,
    id: "i_bar",
    ipa: "ɨ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: U_BAR,
    id: "u_bar",
    ipa: "ʉ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_E,
    id: "reversed_e",
    ipa: "ɘ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: O_BAR,
    id: "o_bar",
    ipa: "ɵ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: SCHWA,
    id: "schwa",
    ipa: "ə",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Mid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_EPSILON,
    id: "reversed_epsilon",
    ipa: "ɜ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: CLOSED_REVERSED_EPSILON,
    id: "closed_reversed_epsilon",
    ipa: "ɞ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TURNED_A,
    id: "turned_a",
    ipa: "ɐ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::NearOpen,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: A_CENTRALIZED,
    id: "a_centralized",
    ipa: "ä",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[Modifier::Centralized])

    }
}

define_ipa_phoneme! {
    name: TURNED_M,
    id: "turned_m",
    ipa: "ɯ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: U,
    id: "u",
    ipa: "u",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: UPSILON,
    id: "upsilon",
    ipa: "ʊ",
    vocalic: {
        position: TonguePosition::NearBack,
        height: TongueHeight::NearClose,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN,
    id: "rams_horn",
    ipa: "ɤ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: O,
    id: "o",
    ipa: "o",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN_RAISED,
    id: "rams_horn_raised",
    ipa: "ɤ̞",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Mid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[Modifier::Lowered])
    }
}
define_ipa_phoneme! {
    name: O_RAISED,
    id: "o_raised",
    ipa: "o̞",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Mid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[Modifier::Lowered])
    }
}
define_ipa_phoneme! {
    name: TURNED_V,
    id: "turned_v",
    ipa: "ʌ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: OPEN_O,
    id: "open_o",
    ipa: "ɔ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: SCRIPT_A,
    id: "script_a",
    ipa: "ɑ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TURNED_SCRIPT_A,
    id: "turned_script_a",
    ipa: "ɒ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Open,
        roundness: Roundness::Rounded,
        modifiers: Cow::Borrowed(&[]),
    }
}
pub fn all_vowels() -> Vec<&'static crate::phonology::SoundDefinition> {
    ALL_VOWELS.to_vec()
}
const ALL_VOWELS: &[&crate::phonology::SoundDefinition] = &[
    &I,
    &Y,
    &SMALL_CAP_I,
    &SMALL_CAP_Y,
    &E,
    &O_WITH_STROKE,
    &E_RAISED,
    &O_WITH_STROKE_RAISED,
    &EPSILON,
    &OE,
    &AE,
    &A,
    &SMALL_CAP_OE,
    &I_BAR,
    &U_BAR,
    &REVERSED_E,
    &O_BAR,
    &SCHWA,
    &REVERSED_EPSILON,
    &CLOSED_REVERSED_EPSILON,
    &TURNED_A,
    &A_CENTRALIZED,
    &TURNED_M,
    &U,
    &UPSILON,
    &RAMS_HORN,
    &O,
    &RAMS_HORN_RAISED,
    &O_RAISED,
    &TURNED_V,
    &OPEN_O,
    &SCRIPT_A,
    &TURNED_SCRIPT_A,
];

use crate::define_ipa_phoneme;
use crate::phonology::{Diacritics, Roundness, TongueHeight, TonguePosition, EMPTY_DIACRITICS};
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
    ipa: "i",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: Y,
    ipa: "y",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_I,
    ipa: "ɪ",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::NearClose,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_Y,
    ipa: "ʏ",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::NearClose,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: E,
    ipa: "e",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: O_WITH_STROKE,
    ipa: "ø",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: E_RAISED,
    ipa: "e̞",
    vocalic:{
        position: TonguePosition::Front,
        height: TongueHeight::Mid,
        roundness: Roundness::Unrounded,
        diacritics: Cow::Borrowed(&[Diacritics::Raised])
    }
}
define_ipa_phoneme! {
    name:O_WITH_STROKE_RAISED,
    ipa: "ø̞",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Mid,
        roundness: Roundness::Rounded,
        diacritics: Cow::Borrowed(&[Diacritics::Raised])
    }
}
define_ipa_phoneme! {
    name: EPSILON,
    ipa: "ɛ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: OE,
    ipa: "œ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: AE,
    ipa: "æ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::NearOpen,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: A,
    ipa: "a",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: SMALL_CAP_OE,
    ipa: "ɶ",
    vocalic: {
        position: TonguePosition::Front,
        height: TongueHeight::Open,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}

define_ipa_phoneme! {
    name: I_BAR,
    ipa: "ɨ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: U_BAR,
    ipa: "ʉ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_E,
    ipa: "ɘ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: O_BAR,
    ipa: "ɵ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: SCHWA,
    ipa: "ə",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Mid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_EPSILON,
    ipa: "ɜ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: CLOSED_REVERSED_EPSILON,
    ipa: "ɞ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: TURNED_A,
    ipa: "ɐ",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::NearOpen,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: A_CENTRALIZED,
    ipa: "ä",
    vocalic: {
        position: TonguePosition::Central,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        diacritics: Cow::Borrowed(&[Diacritics::Centralized])

    }
}

define_ipa_phoneme! {
    name: TURNED_M,
    ipa: "ɯ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Close,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: U,
    ipa: "u",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Close,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: UPSILON,
    ipa: "ʊ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::NearClose,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN,
    ipa: "ɤ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: O,
    ipa: "o",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::CloseMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN_RAISED,
    ipa: "ɤ̞",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Mid,
        roundness: Roundness::Rounded,
        diacritics: Cow::Borrowed(&[Diacritics::Raised])
    }
}
define_ipa_phoneme! {
    name: O_RAISED,
    ipa: "o̞",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Mid,
        roundness: Roundness::Rounded,
        diacritics: Cow::Borrowed(&[Diacritics::Raised])
    }
}
define_ipa_phoneme! {
    name: TURNED_V,
    ipa: "ʌ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: OPEN_O,
    ipa: "ɔ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::OpenMid,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: SCRIPT_A,
    ipa: "ɑ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Open,
        roundness: Roundness::Unrounded,
        diacritics: EMPTY_DIACRITICS,
    }
}
define_ipa_phoneme! {
    name: TURNED_SCRIPT_A,
    ipa: "ɒ",
    vocalic: {
        position: TonguePosition::Back,
        height: TongueHeight::Open,
        roundness: Roundness::Rounded,
        diacritics: EMPTY_DIACRITICS,
    }
}

//! Beispiel für komplexe Patterns mit verschiedenen Silbenstrukturen
//!
//! Dieses Beispiel zeigt, wie man ein Pattern erstellt, das verschiedene
//! Silbenstrukturen am Anfang, in der Mitte und am Ende hat:
//!
//! - Start: cv, vc, c oder cvc
//! - Mitte: cv, vc oder nichts (optional)
//! - Ende: cv, cvc, vc, cc oder c

use crate::core::Name;
use crate::symbol_types::SymbolMapDefinition;
use crate::{define_name_category, define_phonetic_rules, define_symbol_profile};

// Definiere ein Symbol-Profil für deutsche Klänge
define_symbol_profile! {
    GermanSounds {
        syllables: ["ach", "ang", "berg", "dorf", "fels", "heim", "hof", "stein", "wald", "bach"],
        simple_vowels: ["a", "e", "i", "o", "u"],
        complex_vowels: ["a", "e", "i", "o", "u", "au", "ei", "eu", "ie", "ue", "oe", "ae"],
        simple_consonants: ["b", "d", "f", "g", "h", "k", "l", "m", "n", "p", "r", "s", "t", "w", "z"],
        beginning_clusters: ["bl", "br", "dr", "fl", "fr", "gl", "gr", "kl", "kr", "pl", "pr", "schl", "schr", "spr", "st", "tr"],
        ending_clusters: ["ch", "ck", "ff", "ll", "mm", "nn", "pp", "rr", "ss", "tt", "tz", "ng", "nk", "rst", "rts"],
    }
}

// Definiere phonetische Regeln für deutsche Klänge
define_phonetic_rules! {
    GermanRules {
        forbidden_sequences: ["aa", "ee", "ii", "oo", "uu", "yyy", "zzz"],
        preferred_sequences: ["ch", "sch", "st", "ng", "nk", "tz"],
        vowel_consonant_compatibility: {
            'a' => ['r', 'l', 'n', 'm'],
            'e' => ['r', 'l', 'n', 'm'],
            'i' => ['n', 'r'],
            'o' => ['r', 'l', 'n'],
            'u' => ['r', 'l', 'n'],
        },
        consonant_vowel_compatibility: {
            'r' => ['a', 'e', 'i', 'o', 'u'],
            'l' => ['a', 'e', 'i', 'o', 'u'],
            'n' => ['a', 'e', 'i', 'o', 'u'],
            'm' => ['a', 'e', 'i', 'o', 'u'],
        },
        max_consecutive_vowels: 2,
        max_consecutive_consonants: 3,
        preferred_weight_multiplier: 2.0,
        compatible_weight_multiplier: 1.5,
    }
}

// Definiere verschiedene Kategorien für unterschiedliche Komplexitätsgrade

// Kategorie 1: Einfaches Pattern - nur Start und Ende
// Start: cv, vc, c oder cvc; Ende: cv, cvc, vc, cc oder c
define_name_category! {
    SimpleGermanName {
        pattern: "<cv|vc|c|cvc><cv|cvc|vc|cc|c>",
        symbol_profile: GermanSounds,
        phonetic_rules: GermanRules,
    }
}

// Kategorie 2: Mittleres Pattern - Start, optionale Mitte, Ende
// Start: cv, vc, c oder cvc; Mitte: cv, vc oder leer; Ende: cv, cvc, vc, cc oder c
define_name_category! {
    MediumGermanName {
        pattern: "<cv|vc|c|cvc><cv|vc|><cv|cvc|vc|cc|c>",
        symbol_profile: GermanSounds,
        phonetic_rules: GermanRules,
    }
}

// Kategorie 3: Komplexes Pattern - mehrere optionale Mittelteile
define_name_category! {
    ComplexGermanName {
        pattern: "<cv|vc|c|cvc><cv|vc|><cv|vc|><cv|cvc|vc|cc|c>",
        symbol_profile: GermanSounds,
        phonetic_rules: GermanRules,
    }
}

// Kategorie 4: Sehr komplexes Pattern mit verschiedenen Optionen
define_name_category! {
    VeryComplexGermanName {
        pattern: "<cv|vc|c|cvc><cv|vc|cvc|><cv|vc|><cv|cvc|vc|cc|c|cv>",
        symbol_profile: GermanSounds,
        phonetic_rules: GermanRules,
    }
}

// Kategorie 5: Minimalistisches Pattern
define_name_category! {
    MinimalGermanName {
        pattern: "<cv|c><c|cv>",
        symbol_profile: GermanSounds,
        phonetic_rules: GermanRules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_pattern_generation() {
        let mut rng = thread_rng();

        // Teste alle Pattern-Varianten
        let simple_name = Name::<SimpleGermanName>::new().generate(&mut rng);
        let medium_name = Name::<MediumGermanName>::new().generate(&mut rng);
        let complex_name = Name::<ComplexGermanName>::new().generate(&mut rng);
        let very_complex_name = Name::<VeryComplexGermanName>::new().generate(&mut rng);
        let minimal_name = Name::<MinimalGermanName>::new().generate(&mut rng);

        // Alle sollten nicht-leere Namen generieren
        assert!(!simple_name.is_empty());
        assert!(!medium_name.is_empty());
        assert!(!complex_name.is_empty());
        assert!(!very_complex_name.is_empty());
        assert!(!minimal_name.is_empty());

        println!("Einfacher Name: {}", simple_name);
        println!("Mittlerer Name: {}", medium_name);
        println!("Komplexer Name: {}", complex_name);
        println!("Sehr komplexer Name: {}", very_complex_name);
        println!("Minimaler Name: {}", minimal_name);
    }

    #[test]
    fn test_multiple_generations() {
        let mut rng = thread_rng();
        let generator = Name::<MediumGermanName>::new();

        // Generiere mehrere Namen um Vielfalt zu zeigen
        println!("Beispiel-Namen mit mittlerem Pattern:");
        for i in 0..10 {
            let name = generator.generate(&mut rng);
            println!("{}: {}", i + 1, name);
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_pattern_lengths() {
        let mut rng = thread_rng();

        // Teste, dass verschiedene Patterns verschiedene Längen haben können
        let mut simple_lengths = Vec::new();
        let mut complex_lengths = Vec::new();

        for _ in 0..20 {
            let simple = Name::<SimpleGermanName>::new().generate(&mut rng);
            let complex = Name::<ComplexGermanName>::new().generate(&mut rng);

            simple_lengths.push(simple.len());
            complex_lengths.push(complex.len());
        }

        // Komplexe Namen sollten tendenziell länger sein
        let avg_simple = simple_lengths.iter().sum::<usize>() as f64 / simple_lengths.len() as f64;
        let avg_complex =
            complex_lengths.iter().sum::<usize>() as f64 / complex_lengths.len() as f64;

        println!("Durchschnittliche Länge einfacher Namen: {:.1}", avg_simple);
        println!(
            "Durchschnittliche Länge komplexer Namen: {:.1}",
            avg_complex
        );

        // Komplexe Namen sollten mindestens so lang wie einfache sein
        assert!(avg_complex >= avg_simple);
    }
}

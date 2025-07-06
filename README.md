# Name Generator

A flexible and extensible name generation system for Rust, supporting both traditional pattern-based generation and advanced phonetically-grounded language profiles.

## Features

### Two Generation Systems

1. **Pattern-Based Generation** (Traditional)
   - Simple pattern strings like `"<s><v><c>"`
   - Custom symbol maps for different sound sets
   - Phonetic rules for sound compatibility
   - Fast and lightweight

2. **LanguageProfile System** (New)
   - Phonetically-grounded language profiles using IPA notation
   - YAML-based language definitions
   - Syllable structure modeling (onset, nucleus, coda)
   - Vowel harmony and phonetic rules
   - More linguistically accurate

### Key Features

- **Backward Compatibility**: Existing pattern-based code continues to work unchanged
- **Type Safety**: Strong typing prevents misuse and provides clear APIs
- **Extensibility**: Easy to add new languages and sound profiles
- **Phonetic Accuracy**: IPA-based phoneme system for realistic pronunciation
- **Configurable**: Adjustable syllable patterns, frequency weights, and harmony rules

## Quick Start

### Pattern-Based Generation

```rust
use name_generator::core::Name;
use name_generator::categories::examples::StandardStar;
use rand::thread_rng;

let mut rng = thread_rng();
let generator = Name::<StandardStar>::new();
let name = generator.generate(&mut rng);
println!("Generated name: {}", name);
```

### LanguageProfile Generation

```rust
use name_generator::core::Name;
use name_generator::categories::profile_examples::GermanLanguageProfile;
use rand::thread_rng;

let mut rng = thread_rng();
let generator = Name::<GermanLanguageProfile>::new();
let name = generator.generate(&mut rng);
println!("German name: {}", name);
```

### Direct Profile Usage

```rust
use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use rand::thread_rng;

let profile = LanguageProfile::load_builtin("german").unwrap();
let generator = LanguageProfileGenerator::new(&profile);
let mut rng = thread_rng();
let name = generator.generate(&mut rng);
println!("Direct profile name: {}", name);
```

## Language Profiles

The system includes built-in language profiles:

- **German**: Reflects German phonology with consonant clusters, vowel system
- **English**: Models English phonetic patterns and diphthongs

### Profile Structure

Language profiles are defined in YAML format with:

```yaml
name: "German"
phonetic_inventory:
  phonemes:
    - ipa: "[p]"
      phoneme_type: "Consonant"
      grapheme: "p"
      frequency: 0.8
  phoneme_groups:
    front_vowels: ["[i]", "[ɪ]", "[e]", "[ɛ]"]
syllable_structure:
  patterns:
    - pattern: "CV"
      frequency: 0.3
    - pattern: "CVC"
      frequency: 0.4
  onsets:
    - phonemes: ["[p]"]
      frequency: 0.8
```

## Examples

Run the included examples to see the system in action:

```bash
# Compare different generation approaches
cargo run --example profile_comparison

# See the full LanguageProfile system demonstration
cargo run --example language_profile_demo
```

## Creating Custom Language Profiles

1. Create a YAML file in the `languages/` directory
2. Define phonemes with IPA notation and grapheme mappings
3. Specify syllable patterns and phoneme clusters
4. Set frequency weights for natural distribution

## Architecture

- **`phonetics`**: IPA-based phoneme system and utilities
- **`language_profile`**: Profile definitions and loading system
- **`generators`**: Profile-based name generation engine
- **`core`**: Unified API supporting both systems
- **`categories`**: Example implementations and categories

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

All 32+ tests pass, ensuring system reliability and backward compatibility.

## Migration Guide

Existing pattern-based code requires no changes. To use LanguageProfiles:

1. Import the new category types from `categories::profile_examples`
2. Use the same `Name<T>` API - the system automatically detects profile categories
3. Optionally use direct profile loading for advanced use cases

## Dependencies

- `rand`: Random number generation
- `serde` + `serde_yaml`: Profile serialization
- `lazy_static`: Static data management
- `paste`: Macro utilities

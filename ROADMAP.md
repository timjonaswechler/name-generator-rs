# Roadmap

## End goal, users, and present scope

Build an IPA-based constructed-language generator for worldbuilders and teams creating fictional worlds. Its first useful application is plausible names; the same linguistic foundation should later support vocabulary, word formation, and grammar. Names are the first application, not the final scope.

Names should be generated from reusable linguistic rules, not lookup tables of premade names. A static catalogue of IPA sounds is compatible with this goal: it supplies sound data, not finished names.

Humans will read, pronounce, and discuss the results. Practical pronunciation and consistent, readable spelling matter. IPA describes sounds; it does not guarantee that arbitrary sequences are easy to pronounce or sound natural.

The present Rust library models and validates configurations, derives possible sounds within a bounded articulatory model, and validates supplied word forms with duration, stress, tone, IPA output, and separate spelling. Automatic inventory selection, syllable sampling, and name generation are not implemented.

## Agreed boundaries and distinct responsibilities

- **Sound foundation:** use IPA for now; defer fictional sounds beyond IPA.
- **Race anatomy:** determines which sounds are physically possible within the model, not which language is spoken.
- **Language inventory:** selects sounds from that possible set. The same anatomy can support multiple different languages.
- **Phonotactics:** governs permitted sound combinations within syllables and across syllable boundaries.
- **Prosody:** include segment duration, word stress, and syllable tone. Anatomy constrains capabilities; the language chooses which distinctions and rules to use.
- **Spelling:** renders sounds and prosody consistently for human readers; it is separate from internal IPA and sound-combination rules. Leaving a prosodic feature unwritten must be an explicit choice.
- **Reuse:** anatomy, inventories, and phonotactics must remain useful beyond names, rather than being buried inside a name-only generator.

The intended progression is: IPA catalogue and production requirements → anatomical possibilities → language inventory → syllables and phonotactics → words and names → broader language generation. Spelling is a separate presentation layer for generated sound sequences.

## Evidence and status conventions

**Supported** means implemented with relevant local test evidence; **partial** means only part of a milestone exists; **planned** means it is not implemented in the active Rust path; **unverified** means evidence is insufficient. Model-refactor checks passed: `cargo test --locked --offline --all-targets`, `cargo clippy --locked --offline --all-targets -- -D warnings`, and the registered example. Regression and acceptance evidence is in [anatomy regressions](tests/anatomy_regressions.rs) and [model contracts](tests/model_contracts.rs), with [prosody contracts](tests/prosody_contracts.rs), [chart-reference fixtures](tests/ipa_chart_contracts.rs), and [diacritic requirement cases](tests/diacritic_requirements.rs). These checks establish software behavior, not complete IPA coverage or physiological accuracy.

- [Active module exports](src/lib.rs) separate anatomy, phonology, syllables, language, and validation. The [registered example](examples/lang.rs) supplies human anatomy, manually chosen phonemes, and syllable settings, then prints a configuration summary and IPA for a manually supplied prosodic word form; it contains no random name-generation call.
- [Cargo configuration](Cargo.toml) disables automatic example discovery and registers only `lang`. The other [example drafts](examples/) are proposals, not implemented library behavior, as also explained in the [README](README.md). Their placeholder generation methods are not evidence of a working generator.
- The [existing analysis](ANALYSE_UND_EMPFEHLUNGEN.md) is historical context. Its recommendations are not product requirements, and its old build/test results are not used as evidence for the current implementation. Its automatic-example-discovery failure report does not describe the current Cargo setting. Current compilation is covered by the model-refactor checks above.
- [Lua helpers](languages/common.lua) and a [language profile](languages/german.lua) contain additional rules and spelling data. No loader or generation connection was found in the inspected active Rust modules. Their integration and runtime behavior remain unverified.

## Checked IPA coverage boundary

The reference is the Association's [official IPA chart](https://www.internationalphoneticassociation.org/content/ipa-chart): the inspected sub-charts are the 2026 reissue, revised to 2015/2005. [Reference fixtures](tests/ipa_chart_contracts.rs) independently transcribe the printed pulmonic and vowel tables, non-pulmonic symbols/ejective examples, and other symbols, including the chart's tie-bar examples. Those entries have structured features and readable names. The 31 listed diacritic properties have typed representations as primary fields or modifiers.

This is a declared base-chart scope, not a claim to enumerate every possible modified sound. Anatomical requirements are a bounded engineering model, not biological facts established by the chart. A complete IPA string parser, empirical validation of fictional anatomy, and phrase-level grouping, linking, and global intonation remain outside the current word-level implementation.

## Milestones

These are proposed delivery checkpoints for the agreed goals, not approved API designs or implementation tasks. Completion criteria and permitted/prohibited cases are future acceptance targets, not claims that the current code passes them. In patterns, `C` means a consonantal onset/coda slot and `V` means a nucleus slot, normally a vowel but also available to an explicitly syllabic consonant. A dot marks a syllable boundary.

### 1. IPA sound catalogue and production requirements

**Purpose / capability:** provide inspectable sound descriptions and the requirements needed to determine whether an anatomy can produce them.

**Inputs → outputs:** a defined IPA coverage scope and articulatory descriptions → catalogue entries with IPA notation, consistent features, and production requirements. **Dependencies:** settle the coverage checklist, including how modifiers and combinations are counted.

**Current status: supported for the declared base-chart scope; arbitrary derived combinations remain partial.** The [catalogue](src/phonology/catalog.rs) provides stable IDs, unique IPA notation, [readable descriptions](src/phonology/descriptions.rs), [vowels](src/phonology/vowels.rs), and [pulmonic](src/phonology/consonants/pulmonics.rs), [click](src/phonology/consonants/clicks.rs), [ejective](src/phonology/consonants/ejectives.rs), and [implosive](src/phonology/consonants/implosives.rs) entries. [Diacritics](src/phonology/diacritics.rs) are represented as typed properties, not an exhaustive list of sound combinations. The reference fixtures verify printed-symbol coverage and selected metadata corrections; additional derived entries are not an exhaustive phonetic audit.

**Completion criteria:** audit each entry against the declared coverage checklist; provide consistent notation, features, and production requirements for every included sound. Record omissions explicitly; do not call the catalogue complete while checklist items are missing.

**Cases:** permit a `/p/` entry with compatible bilabial plosive features; make contradictory place/group combinations unrepresentable or reject them rather than silently treating them as another sound.

### 2. Anatomical capabilities and automatic possible-sound determination

**Purpose / capability:** let a user define a race's vocal anatomy and obtain the set of sounds it can physically produce within the model.

**Inputs → outputs:** a consistent anatomy plus milestone 1's catalogue and requirements → anatomically possible sounds, with exclusions distinguishable from unsupported or unknown requirements. **Dependencies:** milestone 1 and anatomical consistency checks.

**Current status: supported within the declared capability model.** [Anatomy data](src/anatomy/speaker.rs), a [human preset](src/anatomy/builder.rs), and [consistency checks with separate warnings](src/anatomy/errors.rs) feed a [central evaluator](src/anatomy/production.rs). Manual validation and automatic derivation share requirements for trills, taps, affricate phases, laterals, double/epiglottal articulations, and modifiers. Dental contact can be specified independently of tooth shape. Unspecified contacts and non-charted combinations without a validated rule remain unresolved rather than silently possible. Timing, stress, and pitch capabilities are checked separately. Possible means the declared requirements are met, not that physiological feasibility has been empirically proven.

**Completion criteria:** derive the possible set from the catalogue without requiring manual sound selection; check each supported sound's requirements; distinguish missing support from physical impossibility instead of accepting unknowns as possible. Reject internally inconsistent anatomy.

**Cases:** permit `/p/` for an otherwise suitable human anatomy; exclude it when lips are absent. A sound with unimplemented requirements must not be reported as confirmed possible.

### 3. Language-specific sound inventories

**Purpose / capability:** create distinct languages for the same anatomy by selecting and organizing different subsets of possible sounds.

**Inputs → outputs:** milestone 2's possible set and language-specific choices → a validated language inventory. **Dependencies:** milestones 1–2; this inventory is not identical to the anatomy's entire possible set.

**Current status: supported for manual selection.** A [language inventory](src/phonology/builder.rs) selects stable sound IDs from a matching possible set and rejects duplicates, unknown IDs, impossible sounds, and unsupported sounds. [LanguageDraft::build](src/language/builder.rs) validates anatomy, inventory, phonotactics, prosody, and optional spelling together and returns an immutable `LanguageProfile`. Editing or deserializing a draft requires rebuilding; automatic language-specific inventory selection remains planned.

**Completion criteria:** demonstrate two distinct valid inventories under one anatomy; reject selections outside its possible set. Provide an accessible, fully validated language configuration; changing anatomy or other dependent settings must not leave incompatible selections accepted.

**Cases:** permit two human-compatible inventories, one using `/p, t, a, i, u/` and another `/m, n, a, i, u/`. Prohibit including `/p/` for a no-lips anatomy. Anatomical possibility does not require every language to include that sound.

### 4. Syllables and phonotactics

**Purpose / capability:** configure and generate syllables with a recognizable language-specific sound structure, rather than concatenate arbitrary IPA symbols.

**Inputs → outputs:** a validated inventory, syllable patterns, and positional sound/cluster rules → validated syllable rules and legal syllable outputs. **Dependencies:** milestone 3 and consistent pattern interpretation.

**Current status: partial.** [Syllable rules](src/syllables.rs), a [single pattern parser](src/syllables/patterns.rs), and [cross-validation](src/syllables/builder.rs) support forbidden/optional/required positions, weighted sound sequences, word-position restrictions, and explicit forbidden boundary pairs. Complex nuclei need at least one syllabic core; explicitly non-syllabic vowels can form glides in nuclei or margins. [Relative weights](src/weights.rs) reject negative and non-finite values; all-zero choice sets are rejected. Supplied word forms are checked, but no syllable sampler or general constraint solver is implemented.

**Completion criteria:** demonstrate syllable generation obeying the selected inventory, patterns, and position rules. Reject malformed or unsatisfiable configurations rather than emit illegal syllables. Pattern validation and interpretation must agree; any configured selection weights need defined, checked semantics.

**Cases:** for a `CV`-only profile containing `/p, t, a, i, u/`, permit `/pa/`; prohibit `/pta/` and `/pat/`. Reject a pattern requiring unavailable sounds or clusters. A nucleus-less pattern such as `CCC` is invalid for this pattern model; syllabic consonants occupy a `V` nucleus slot rather than removing the nucleus.

### 5. Multi-syllable words and usable names

**Purpose / capability:** generate new words and names that share a language's sound identity and can be read and discussed by people.

**Inputs → outputs:** milestone 4's syllable rules, word-position and cross-syllable constraints, and a spelling profile → valid multi-syllable sound sequences with consistent readable names. **Dependencies:** milestones 1–4, explicit boundary rules, and the separate spelling layer below.

**Current status: partial; automatic word/name generation is planned.** [LanguageProfile](src/language/builder.rs) validates supplied multi-syllable forms, including boundary, word-position, and prosody restrictions, and provides separate IPA and [spelling](src/orthography.rs) output. The [example](examples/lang.rs) validates a configuration and a manually supplied prosodic form, not a generated name. No random generation, practical pronunciation review, or naturalness assessment has been completed.

**Completion criteria:** produce names from rules without a premade-name table. Enforce coda–onset transitions across boundaries and word-initial/final restrictions before accepting output. Demonstrate permitted and prohibited boundary cases, consistent spelling, and handling of unsatisfiable rules without returning an invalid name. Review representative names with intended human users and record pronunciation difficulties; structural validity alone is not a naturalness guarantee.

**Cases:** for a `CV`/`CVC` profile banning `/t.t/`, permit `/pa.ta/` but prohibit `/pat.ta/`, even though its individual syllables are legal. Do not use a word-initial-only cluster medially. A readable spelling must not hide an invalid underlying sequence.

#### Readable spelling is not IPA or phonotactics

Keep the internal sound sequence distinct from its written form. A spelling profile maps sounds to consistent written names, with enough pronunciation guidance for its intended readers. An illustrative profile could map `/ʃ/` to `sh`, so `/ʃa/` becomes `sha`; this is an example, not a universal or agreed spelling convention. Missing mappings must be reported, not silently dropped. Easier-looking text alone cannot make difficult sound sequences easy to pronounce; inventories and combination rules must address that separately.

#### Duration, stress, and tone

[Prosody rules](src/prosody.rs) are now part of the foundation, separate from catalogue sound identity:

- Segment duration supports unmarked, extra-short, half-long, and long values. A long vowel or consonant still occupies one sound slot.
- Word stress supports no stress, fixed offsets from either edge, or lexical stress, with one primary stress and optional secondary stresses. Fixed rules reject too-short words rather than silently changing the rule.
- Syllable tones support five relative pitch levels, contours, and upstep/downstep. Languages can forbid tones, make them optional, or require them. Toned nuclei need a voiced component in this model.
- IPA retains these annotations. [Prosody spelling](src/prosody/spelling.rs) requires mappings or explicit omission; missing mappings are errors. The current formatter places stress before a syllable, duration after each segment, and tone letters after the syllable.

[Acceptance cases](tests/prosody_contracts.rs) distinguish /ma/ from /maː/, initial from final stress, and level from contour tones. They reject forbidden annotations, missing required tones, unavailable anatomical capabilities, and silently discarded written distinctions. These are word-level modeling/validation capabilities, not a stress/tone generator or a full theory of prosody.

### 6. Broader language generation

**Purpose / capability:** extend the same foundation to vocabulary, word formation, and grammar rather than build a separate name-only system.

**Inputs → outputs:** the reusable sound system and word-generation capabilities, plus later agreed language-level rules → vocabulary, word-formation, and grammatical outputs. **Dependencies:** milestones 1–5 for the existing foundation; each later subsystem needs its own scope before implementation.

**Current status: planned.** The [active modules](src/lib.rs) and [language configuration](src/language.rs) provide no vocabulary, word-formation, or grammar generation API.

**Completion criteria:** after defining those later scopes, demonstrate vocabulary, word-formation examples, and grammatical expressions reusing the existing language foundation. Detailed semantic, morphological, and grammatical requirements remain open, as does phrase-level prosody beyond the current word representation.

**Cases:** permit later language systems to reuse the same inventory and sound-combination rules; prohibit treating name generation as project completion or bypassing that foundation with an unrelated sound engine. This does not preclude storing vocabulary once it is generated.

## Engineering choices and open decisions

Implemented engineering choices are distinct from the agreed product goals: explicit stable sound IDs, typed production requirements, a draft-to-validated-profile boundary, relative weights, separate spelling, and prosodic annotations on segments/syllables rather than duplicate catalogue entries. The public API and configuration schema changed intentionally; no compatibility layer is maintained. The following remain recommendations or questions:

- The base-chart coverage checklist is tested. Further derived/composite sound profiles need their own audited notation/features and bounded capability rules; structural checks are not a general IPA parser or biological proof.
- Recommend reproducible seeding or a caller-supplied random source for debugging and comparison; determinism and compatibility guarantees have not been agreed.
- Manual inventory selection exists; decide how generated inventories and linguistic preferences should work. Anatomy-derived possibilities must remain separate from this choice.
- Extend the initial forbidden-boundary-pair model as needed, and identify the target reading audience before evaluating name usability. Spelling conventions, pronunciation guidance, and handling of difficult sounds remain design choices.
- Decide whether Lua becomes a supported configuration/rule interface or remains experimental. Its presence is not proof of integration.

Completion of this roadmap's documentation criteria does not prove that future generated languages will sound natural. Clarity and perceived linguistic plausibility require qualitative review; those judgments are subjective.

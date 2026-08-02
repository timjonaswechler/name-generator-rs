# Projektanalyse und Empfehlungen

## Zweck dieses Dokuments

Dieses Dokument hält den aktuellen Stand des Crates `name_generator`, die gefundenen Stärken und Probleme sowie Empfehlungen für die weitere Entwicklung fest.

Die Analyse bezieht sich auf den gegenwärtigen Arbeitsstand. Bereits vorhandene, nicht committete Änderungen wurden nicht verändert.

## Zielbild des Projekts

Das Projekt soll aus anatomischen und sprachlichen Eigenschaften eine mögliche Sprache beziehungsweise ein Laut- und Regelsystem erzeugen, aus dem später reproduzierbar Namen oder Wörter generiert werden können.

Die fachlich sinnvolle Verarbeitungskette ist:

```text
Anatomie
  ↓ bestimmt produzierbare Phone
Artikulatorisch möglicher Lautraum
  ↓ sprachspezifische Auswahl
Phoneminventar
  ↓ phonotaktische Einschränkungen
Silbenstruktur
  ↓ deterministische Generierung mit Seed
Silben, Wörter und Namen
```

Dabei sollten folgende Begriffe getrennt bleiben:

- **Phon/Phone:** Ein tatsächlich produzierbarer Sprachlaut.
- **Phonem:** Ein Laut, der innerhalb einer konkreten Sprache bedeutungsunterscheidend verwendet wird.
- **Phoneminventar:** Die von einer Sprache ausgewählte Menge von Phonemen.
- **Phonotaktik:** Regeln dafür, wie Phoneme innerhalb und zwischen Silben kombiniert werden dürfen.
- **Anatomie:** Bestimmt Möglichkeiten und Einschränkungen der Lautproduktion, aber nicht automatisch das Phoneminventar einer Sprache.
- **Deterministisch beziehungsweise reproduzierbar:** Dieselbe Konfiguration und derselbe Seed erzeugen dieselbe Ausgabe.

## Zusammenfassung

Das Projekt besitzt bereits eine ungewöhnlich detaillierte Grundlage:

- anatomische Eigenschaften eines Sprechers,
- strukturierte IPA-Merkmale,
- Validierung von Phonemen gegen anatomische Fähigkeiten,
- Modelle für Onset, Nukleus, Coda und Silbenmuster,
- erste Typestate-Ansätze für den Konfigurationsaufbau.

Der derzeit aktive Rust-Code ist jedoch hauptsächlich ein **Konfigurations- und Validierungssystem**. Die eigentliche Sprach-, Silben-, Wort- oder Namensgenerierung ist noch nicht implementiert. Insbesondere fehlen eine Seed-Schnittstelle, die gewichtete Auswahl und eine erreichbare abgeschlossene Sprachkonfiguration.

## Stärken

### 1. Gute fachliche Aufteilung

`src/lib.rs` trennt die Domänen Anatomie, Phonologie, Silben, Sprache und Validierung nachvollziehbar:

- `src/anatomy.rs` und `src/anatomy/`
- `src/phonology.rs` und `src/phonology/`
- `src/syllables.rs` und `src/syllables/`
- `src/language.rs` und `src/language/`
- `src/validation/`

Diese Trennung eignet sich grundsätzlich für eine schrittweise Verarbeitung von anatomischen Möglichkeiten bis zur Wortgenerierung.

### 2. Detailliertes Anatomiemodell

`src/anatomy/speaker.rs` modelliert unter anderem:

- Lippenkontrolle,
- Zahnkonfiguration,
- Alveolarkamm, harten und weichen Gaumen sowie Uvula,
- Zungenspitze, Zungenblatt, Zungenkörper und Zungenwurzel,
- Retroflexion und laterale Artikulation,
- Stimmkontrolle,
- pulmonischen Luftstrom, Velumkontrolle und Klickfähigkeit.

Damit kann das System auch von der menschlichen Anatomie abweichende Sprecher modellieren.

### 3. Merkmalsbasierte IPA-Repräsentation

Phoneme werden in `src/phonology/phonemes.rs` nicht ausschließlich als freie IPA-Zeichen gespeichert. Konsonanten und Vokale besitzen strukturierte artikulatorische Merkmale. Das ist eine wesentlich bessere Grundlage für Validierung und spätere regelbasierte Auswahl als reine Zeichenketten.

### 4. Aggregierte Validierungsfehler

Die Validierungsinfrastruktur kann mehrere Fehler gemeinsam zurückgeben. Das ist für komplexe Sprachkonfigurationen sinnvoller als ein Abbruch beim ersten Fehler.

### 5. Ansatz für compilezeitlich geführten Aufbau

Die Typestates für Onset, Nukleus, Coda und Sprache zeigen die richtige Absicht: Unvollständige Konfigurationen sollen nicht versehentlich als fertige Sprache verwendet werden.

## Befunde

### Kritisch: Die eigentliche Generierung fehlt

Im Rust-Code existieren derzeit keine Generatoren für:

- Phoneminventare aus Eigenschaften,
- Silben,
- Wörter,
- Namen.

Ebenso fehlen:

- eine öffentliche Seed-Schnittstelle,
- die Verwendung eines reproduzierbaren RNGs,
- gewichtete Auswahlverfahren,
- ein klarer Generierungszustand.

`rand` und `rand_chacha` sind in `Cargo.toml` eingetragen, werden für den aktiven Rust-Datenfluss aber nicht verwendet. Die vorhandenen Gewichte beeinflussen deshalb aktuell keine Laufzeitentscheidung.

**Empfehlung:** Eine explizite Generierungsschnittstelle einführen, die einen Seed oder einen vom Aufrufer bereitgestellten RNG akzeptiert.

Beispiel für das Zielbild:

```rust
let language = LanguageBuilder::new("Beispielsprache")
    .anatomy(anatomy)
    .inventory_preferences(preferences)
    .phonotactics(phonotactics)
    .build()?;

let mut generator = NameGenerator::from_seed(language, 42);
let first = generator.generate()?;
```

Bei gleicher Konfiguration und gleichem Seed muss dieselbe Ausgabesequenz entstehen.

### Kritisch: `Initialized` ist nicht erreichbar

`src/language.rs` definiert `NotInitialized`, `Initializing` und `Initialized`. In `src/language/builder.rs` fehlt jedoch eine Transition von `Initializing` nach `Initialized`.

Die Getter sind nur für eine vollständig typisierte `Initialized`-Konfiguration implementiert, dieser Zustand kann über das öffentliche Interface aber nicht erzeugt werden.

**Empfehlung:** Eine abschließende `build()`- oder `finish()`-Methode implementieren. Diese Methode sollte alle domänenübergreifenden Invarianten erneut prüfen und anschließend eine unveränderliche fertige Sprache zurückgeben.

### Hoch: Anatomie begrenzt Laute, erzeugt aber noch keine Sprache

Der aktuelle Datenfluss validiert ein bereits manuell ausgewähltes Phoneminventar gegen die Anatomie. Er erzeugt das Inventar nicht aus Eigenschaften.

Das ist fachlich nicht grundsätzlich falsch, muss aber klar benannt werden:

```text
Aktuell:
manuell gewähltes Inventar → Prüfung gegen Anatomie

Angestrebtes Ziel:
Anatomie → mögliche Phone → gewichtete sprachliche Auswahl → Phoneminventar
```

**Empfehlung:** Zwischen Anatomie und Phoneminventar ein eigenes Modul für den artikulatorisch möglichen Lautraum einführen. Die Spracherzeugung wählt anschließend aus diesem Lautraum anhand von Präferenzen und Wahrscheinlichkeiten.

### Hoch: Ungültige Kombinationen aus `Place` und `Subplace`

In `src/phonology/phonemes.rs` sind `Place` und `Subplace` unabhängige Felder. Dadurch sind fachlich widersprüchliche Kombinationen darstellbar, beispielsweise:

```text
Place::Labial + Subplace::Uvular
```

In `src/phonology/builder.rs` werden unbekannte Kombinationen aktuell über Fallbacks unter anderem als bilabial, alveolar oder velar behandelt. Dadurch wird ein anderes Merkmal geprüft als das konfigurierte.

**Empfehlung:** Keine stillen Fallbacks verwenden. Entweder:

1. Artikulationsorte hierarchisch modellieren, sodass ungültige Kombinationen nicht erzeugt werden können, oder
2. jede Kombination explizit validieren und bei Widersprüchen einen strukturierten Fehler zurückgeben.

Eine mögliche hierarchische Form wäre:

```rust
enum Place {
    Labial(LabialPlace),
    Coronal(CoronalPlace),
    Dorsal(DorsalPlace),
    Laryngeal(LaryngealPlace),
}
```

### Hoch: Laryngeale und implosive Laute werden unvollständig geprüft

In `src/phonology/builder.rs` enthält der Zweig für `Place::Laryngeal` keine eigentliche Validierung. Pharyngeale, epiglottale und glottale Anforderungen werden daher nicht systematisch gegen Zungenwurzel, Epiglottis und Larynx geprüft.

`AirflowMechanism::Implosive` wird erkannt, aber nicht vollständig durch das Anatomiemodell abgesichert.

**Empfehlung:** Für jede unterstützte Lautklasse explizite Anforderungen definieren. Nicht implementierte Klassen sollten bis dahin einen klaren `unsupported_feature`-Fehler erzeugen, statt still akzeptiert zu werden.

### Hoch: Silbenvalidierung enthält wahrscheinlich vertauschte Zweige

In den Validatoren für Onset, Nukleus und Coda werden Bitmasken anhand leerer beziehungsweise gesetzter Bestände gebildet. Mehrere Einzelzweige scheinen anschließend die jeweils falsche Liste zu validieren.

Betroffene Bereiche:

- `src/syllables/errors/onset.rs`
- `src/syllables/errors/nucleus.rs`
- `src/syllables/errors/coda.rs`

Dadurch können typische Konfigurationen unvollständig oder gegen die falschen Daten geprüft werden.

**Empfehlung:** Für jede Kombination aus leeren und gefüllten Beständen tabellengetriebene Tests schreiben. Anschließend kann die Bitmaskenlogik korrigiert oder durch direkt lesbare Bedingungen ersetzt werden.

### Hoch: Fehler für zu kleinen Nukleus wird verworfen

`src/syllables/builder.rs` erkennt einen Nukleus mit zu wenigen Einträgen und fügt einen Fehler hinzu. Die spätere Rückgabe berücksichtigt diesen bereits gesammelten Fehler im Erfolgszweig jedoch nicht zuverlässig.

**Empfehlung:** Alle lokalen Validierungen in einem Fehlercontainer sammeln und ausschließlich dann `Ok` zurückgeben, wenn dieser Container am Ende leer ist.

### Hoch: Domänenübergreifende Konsistenz hängt von der Aufrufreihenfolge ab

Anatomie, Phonologie und Silben können während `Initializing` ersetzt werden. Wird beispielsweise die Anatomie nach dem Setzen der Phonologie verändert, wird die bereits gesetzte Phonologie nicht zwingend erneut geprüft.

**Empfehlung:** Setter sollten möglichst nur Rohkonfiguration sammeln. Die vollständige Cross-Validierung sollte zentral in `build()` stattfinden. Alternativ müsste jede Änderung alle abhängigen Konfigurationen erneut validieren, was fehleranfälliger ist.

### Mittel: Stimmhaftigkeit und Phonation sind zu indirekt modelliert

Stimmhaftigkeit befindet sich derzeit in einer Liste von `Diacritics`. Dadurch können widersprüchliche Angaben gleichzeitig vorkommen. Die Ermittlung in `src/phonology/builder.rs` hängt außerdem von der Reihenfolge der Diakritika und von heuristischen Standardwerten ab.

**Empfehlung:** Primäre Phonation als eigenes Merkmal modellieren, beispielsweise:

```rust
enum Phonation {
    Voiceless,
    ModalVoiced,
    Aspirated,
    Breathy,
    Creaky,
}
```

Diakritika sollten nur zusätzliche Modifikationen darstellen. Widersprüchliche Kombinationen müssen validiert werden.

### Mittel: IPA-Zeichen und Merkmalsstruktur können widersprüchlich sein

`ipa`, `name` und die strukturierte Phonemkonfiguration können unabhängig voneinander verändert werden. Das System kann deshalb beispielsweise ein IPA-Zeichen für einen bilabialen Nasal mit Merkmalen eines uvularen Frikativs verbinden.

**Empfehlung:** Den eingebauten IPA-Katalog als kanonische Quelle behandeln. Externe Konfigurationen sollten Phoneme bevorzugt über stabile IDs referenzieren. Falls benutzerdefinierte Phoneme erlaubt sind, müssen IPA-Darstellung und Merkmale gemeinsam validiert werden.

### Mittel: Builder-Invarianten können umgangen werden

Mehrere Konfigurationen und ihre Felder sind öffentlich mutierbar und deserialisierbar. Dadurch können Duplikatprüfungen, Gewichtsvalidierung und andere Builder-Regeln umgangen werden.

**Empfehlung:**

- Felder fertiger Domänenobjekte privat halten,
- kontrollierte Konstruktoren verwenden,
- Deserialisierung zunächst in separate DTOs durchführen,
- DTOs über eine validierende Konvertierung in Domänenobjekte überführen.

### Mittel: Gewichte besitzen keine ausreichenden Invarianten

Gewichte sind unbeschränkte `f32`-Werte. Dadurch sind negative Werte, `NaN`, unendliche Werte oder uneinheitliche Interpretationen möglich.

**Empfehlung:** Einen validierten Gewichtstyp einführen:

```rust
struct Weight(f64);
```

Dieser sollte nur endliche, nichtnegative Werte akzeptieren. Es sollte außerdem festgelegt werden, ob Gewichte relative Häufigkeiten oder bereits normalisierte Wahrscheinlichkeiten sind. Relative Gewichte sind meist einfacher zu konfigurieren.

### Mittel: Silbenmuster werden unterschiedlich validiert und geparst

Die dokumentierte explizite Syntax wie `(C)(V)(CC)` wird nicht durchgehend mit demselben Parser verarbeitet. Validierung und Speicherung können deshalb zu unterschiedlichen Ergebnissen kommen.

**Empfehlung:** Genau einen Parser verwenden, der entweder ein gültiges typisiertes Pattern oder einen strukturierten Fehler zurückgibt. Rohstrings sollten nach dem Parsen nicht mehr die fachliche Quelle darstellen.

### Mittel: Patterns werden nicht vollständig gegen Inventare geprüft

Die Cross-Validierung prüft hauptsächlich Onset-, Nukleus- und Coda-Bestände. Nicht ausreichend geprüft werden unter anderem:

- ob mindestens ein Pattern vorhanden ist,
- ob ein Pattern einen Onset verlangt, obwohl kein Onset verfügbar ist,
- ob erforderliche Clustergrößen vorhanden sind,
- ob Patterngewichte gültig sind.

**Empfehlung:** `build()` muss jedes Pattern gegen die tatsächlich verfügbaren positionsspezifischen Inventare prüfen.

### Mittel: Anatomische Validierung verwendet teilweise Strings

Typisierte Merkmale werden in `src/phonology/builder.rs` in Strings wie `"bilabial"`, `"click"` oder `"voiced"` übersetzt und anschließend an anatomische Validatoren übergeben.

Das verliert Typinformation und ermöglicht Tippfehler sowie unvollständige Fallbehandlung.

**Empfehlung:** Die Enums direkt an die Validatoren übergeben. Die Anatomieschicht sollte auf `Place`, `AirflowMechanism`, `Phonation` und weitere fachliche Typen reagieren, nicht auf freie Strings.

### Mittel: Rust- und Lua-Architektur sind nicht verbunden

`languages/common.lua` und `languages/german.lua` enthalten bereits weitergehende Regeln, darunter Zufallsauswahl und Übergangsregeln. Im Rust-Code ist jedoch keine aktive Ladeschicht erkennbar.

Zusätzlich verwenden die Lua-Regeln `math.random()`, was ohne kontrolliertes Seeding nicht in das gewünschte reproduzierbare Modell passt.

**Empfehlung:** Eine klare Entscheidung treffen:

1. Rust ist die kanonische Engine und Lua beschreibt ausschließlich Daten beziehungsweise Regeln über ein kontrolliertes Interface, oder
2. die Sprachprofile werden vollständig als Rust-/Serde-Konfigurationen modelliert.

Zwei unabhängige Generierungsmodelle sollten vermieden werden.

### Niedrig: `symmetrics_score` ist noch keine belastbare Domäneneigenschaft

Der Wert ist öffentlich, frei veränderbar und wird im untersuchten Datenfluss nicht nachvollziehbar berechnet.

**Empfehlung:** Zunächst fachlich definieren, was Symmetrie bedeutet. Denkbare getrennte Maße sind:

- Voicing-Paarigkeit,
- Gleichmäßigkeit über Artikulationsorte,
- Gleichmäßigkeit über Artikulationsarten,
- Vokalraumsymmetrie,
- Vollständigkeit natürlicher Lautserien.

Der Score sollte aus dem Inventar berechnet und nicht separat gespeichert werden, sofern er ausschließlich abgeleitet ist.

### Niedrig: Fehlertexte und Kommentare sind teilweise inkonsistent

Beispielsweise verwendet eine leere Coda stellenweise einen Onset-bezogenen Fehler. Einige Kommentare für `word_final_only` sprechen fälschlich vom Wortanfang.

**Empfehlung:** Fehlercodes positionsspezifisch testen und Kommentare an die tatsächliche Semantik angleichen.

## Test- und Build-Status

Zum Analysezeitpunkt gilt:

- `cargo check` ist erfolgreich.
- `cargo test --lib` ist erfolgreich: sechs Tests bestehen.
- Die vorhandenen Tests decken hauptsächlich Makros und generische Validierungsinfrastruktur ab.
- `cargo check --all-targets` schlägt fehl, weil mehrere Dateien unter `examples/` von Cargo als eigenständige Beispiele kompiliert werden, aber keine lauffähigen Beispielprogramme sind.

Betroffen sind unter anderem:

- `examples/improved_lib.rs`
- `examples/improved_phonemes.rs`
- `examples/improved_speaker.rs`
- `examples/language_facade.rs`

**Empfehlung:** Entwurfsdateien aus `examples/` verschieben oder über explizite `[[example]]`-Einträge und passende Dateistruktur in echte ausführbare Beispiele umwandeln. Das Ziel sollte ein erfolgreiches `cargo test --all-targets` sein.

## Empfohlene Zielmodule

### 1. `ArticulatoryCapability`

Verantwortung:

- anatomische Konsistenz prüfen,
- aus einer Anatomie die Menge produzierbarer Phone bestimmen,
- begründen können, warum ein Laut möglich oder unmöglich ist.

Mögliches Interface:

```rust
fn capabilities(anatomy: &SpeakerAnatomy) -> Result<PhoneSpace, ValidationErrors>;
```

### 2. `PhonemeInventoryGenerator`

Verantwortung:

- aus dem möglichen Lautraum ein sprachspezifisches Inventar auswählen,
- Inventargröße, Symmetrie, Seltenheit und gewünschte Klangcharakteristik berücksichtigen,
- alle Zufallsentscheidungen über einen übergebenen RNG treffen.

Mögliches Interface:

```rust
fn generate_inventory<R: Rng + ?Sized>(
    phone_space: &PhoneSpace,
    preferences: &InventoryPreferences,
    rng: &mut R,
) -> Result<PhonemeInventory, GenerationError>;
```

### 3. `Phonotactics`

Verantwortung:

- erlaubte Silbenmuster,
- positionsspezifische Phoneme und Cluster,
- sonoritäts- oder merkmalbasierte Regeln,
- später Regeln zwischen Silben.

Patterns sollten typisiert und nicht erst während jeder Generierung neu aus Strings interpretiert werden.

### 4. `LanguageProfile`

Verantwortung:

- vollständig validierte und danach unveränderliche Sprachkonfiguration,
- zentrale Verbindung aus Anatomie, Phoneminventar und Phonotaktik,
- einziger Eingangspunkt für den Generator.

### 5. `NameGenerator`

Verantwortung:

- reproduzierbare Generierung aus `LanguageProfile` und Seed,
- Auswahl von Pattern, Onset, Nukleus und Coda anhand relativer Gewichte,
- Anwendung von Regeln zwischen Silben,
- optionale Umwandlung von IPA in eine Orthografie.

## Empfohlene Umsetzungsreihenfolge

### Phase 1: Bestehende Invarianten stabilisieren

1. Silbenvalidierungszweige korrigieren und mit Tests absichern.
2. Verlorenen Nukleusfehler korrigieren.
3. Patternparser vereinheitlichen.
4. Gewichte validieren.
5. `LanguageConfiguration::build()` beziehungsweise `finish()` ergänzen.
6. `cargo test --all-targets` reparieren.

Diese Phase sollte abgeschlossen sein, bevor neue Generierungslogik ergänzt wird.

### Phase 2: Domänenmodell schärfen

1. Phone und Phonem begrifflich und typseitig trennen.
2. `Place`/`Subplace` durch gültige hierarchische Typen ersetzen.
3. Phonation als eigenes Merkmal modellieren.
4. Stringbasierte Anatomieprüfung durch typisierte Prüfung ersetzen.
5. fertige Domänenobjekte unveränderlich machen.
6. Symmetrie und andere Inventareigenschaften fachlich definieren.

### Phase 3: Deterministisches Phoneminventar

1. Seed- beziehungsweise RNG-Injektion einführen.
2. möglichen Lautraum aus Anatomie bestimmen.
3. Inventarpräferenzen definieren.
4. gewichtete, reproduzierbare Auswahl implementieren.
5. natürliche Serien und Symmetrie als Auswahlregeln ergänzen.
6. Snapshot- oder Golden-Tests mit festen Seeds schreiben.

### Phase 4: Silben- und Wortgenerierung

1. gewichtete Patternauswahl implementieren.
2. Onset, Nukleus und Coda reproduzierbar auswählen.
3. positionsspezifische Einschränkungen anwenden.
4. Cross-Syllable-Regeln modellieren.
5. Silben zu Wörtern und Namen zusammensetzen.
6. optional eine getrennte Orthografieschicht ergänzen.

### Phase 5: Sprachprofile und Lua

1. festlegen, ob Lua langfristig benötigt wird,
2. ein kanonisches Konfigurationsschema definieren,
3. Lua gegebenenfalls nur als kontrollierten Adapter verwenden,
4. RNG und Seed aus Rust an Lua weiterreichen,
5. deutsche und weitere Sprachprofile als Integrations- und Regressionstests verwenden.

## Empfohlene Tests

Mindestens folgende Testgruppen sollten ergänzt werden:

### Anatomie

- Jeder Artikulationsort mit erfüllten und fehlenden anatomischen Voraussetzungen.
- Pulmonische, ejektive, implosive und Klick-Luftströme.
- Stimmhaftigkeit und verschiedene Phonationsarten.
- Gerundete und ungerundete Vokale.
- Ungültige anatomische Binnenkombinationen.

### Phonologie

- Ungültige `Place`/`Subplace`-Kombinationen.
- Doppelte Phoneme.
- widersprüchliche Merkmale und Diakritika.
- Unicode-Normalisierung von IPA-Darstellungen.
- Berechnung von Inventareigenschaften wie Symmetrie.

### Silben

- Jede Kombination aus Phonemen und Clustern für Onset, Nukleus und Coda.
- implizite und explizite Patternsyntax.
- ungültige Pattern.
- Patternanforderungen gegen leere Inventare.
- negative, nicht endliche und sehr große Gewichte.

### Sprache

- erfolgreicher Übergang in den fertigen Zustand.
- vollständige Cross-Validierung unabhängig von der Builder-Aufrufreihenfolge.
- Deserialisierung ungültiger Konfigurationen.

### Determinismus

- gleicher Seed und gleiche Konfiguration ergeben dieselbe Sequenz,
- verschiedene Seeds erzeugen unterschiedliche Sequenzen,
- Änderungen an irrelevanten Metadaten verändern die Sequenz nicht,
- serialisierte und erneut geladene Konfigurationen erzeugen dieselben Ergebnisse.

## Vorgeschlagene Definition von „fertig“ für eine erste Version

Eine erste belastbare Version wäre erreicht, wenn:

1. eine Anatomie validiert werden kann,
2. daraus ein artikulatorisch möglicher Lautraum entsteht,
3. daraus mit Eigenschaften und Seed ein Phoneminventar erzeugt wird,
4. eine gültige Phonotaktik aufgebaut wird,
5. die Sprache in einen unveränderlichen fertigen Zustand übergeht,
6. mit demselben Seed dieselben Namen entstehen,
7. `cargo test --all-targets` erfolgreich ist,
8. Kerninvarianten durch Tests abgesichert sind.

## Prioritätenübersicht

| Priorität | Maßnahme |
|---|---|
| 1 | Silbenvalidatoren und Nukleusfehler korrigieren |
| 2 | Erreichbaren fertigen Sprachzustand mit zentraler Cross-Validierung schaffen |
| 3 | Ungültige Artikulationsmerkmale und stille Fallbacks beseitigen |
| 4 | Typisierte Phonation und typisierte Anatomieprüfung einführen |
| 5 | Seed-/RNG-Interface festlegen |
| 6 | Artikulatorischen Lautraum und Inventargenerator implementieren |
| 7 | Silben-, Wort- und Namensgenerator implementieren |
| 8 | Lua-Strategie vereinheitlichen |
| 9 | Dokumentation und vollständige Beispiele ergänzen |

## Schlussfolgerung

Das Projekt hat bereits eine starke fachliche Grundlage und kann deutlich mehr werden als ein einfacher zufälliger Namensgenerator. Besonders wertvoll sind das detaillierte Anatomiemodell und die merkmalsbasierte Repräsentation von IPA-Lauten.

Vor der Implementierung weiterer Lauttabellen oder komplexer Sprachregeln sollten jedoch die bestehenden Invarianten, die Typestate-Abschlusslogik und die Silbenvalidierung stabilisiert werden. Danach empfiehlt sich eine klare Trennung zwischen anatomisch möglichen Phonen, sprachlich ausgewählten Phonemen, Phonotaktik und der eigentlichen reproduzierbaren Generierung.

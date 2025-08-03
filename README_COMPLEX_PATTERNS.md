# Komplexe Pattern-Generierung - Deutsche Namen

## Überblick

Dieses Beispiel zeigt, wie Sie komplexe Patterns für die Namensgenerierung erstellen können, die verschiedene Silbenstrukturen unterstützen:

- **Start**: cv, vc, c oder cvc
- **Mitte**: cv, vc oder leer (optional)
- **Ende**: cv, cvc, vc, cc oder c

## Verwendung

```rust
use name_generator::categories::complex_pattern_example::*;
use name_generator::core::Name;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // Mittlere Komplexität (Ihr gewünschtes Pattern)
    let generator = Name::<MediumGermanName>::new();
    let name = generator.generate(&mut rng);
    println!("Deutscher Name: {}", name);
}
```

## Verfügbare Kategorien

### 1. `SimpleGermanName`
- Pattern: `<cv|vc|c|cvc><cv|cvc|vc|cc|c>`
- Nur Start und Ende, keine Mitte
- Durchschnittliche Länge: ~4-5 Zeichen

### 2. `MediumGermanName` (Ihr gewünschtes Pattern)
- Pattern: `<cv|vc|c|cvc><cv|vc|><cv|cvc|vc|cc|c>`
- Start + optionale Mitte + Ende
- Durchschnittliche Länge: ~5-6 Zeichen

### 3. `ComplexGermanName`
- Pattern: `<cv|vc|c|cvc><cv|vc|><cv|vc|><cv|cvc|vc|cc|c>`
- Start + zwei optionale Mittelteile + Ende
- Durchschnittliche Länge: ~6-7 Zeichen

### 4. `MinimalGermanName`
- Pattern: `<cv|c><c|cv>`
- Sehr kurze Namen
- Durchschnittliche Länge: ~3 Zeichen

## Pattern-Syntax

- `<cv>`: Konsonant-Vokal
- `<vc>`: Vokal-Konsonant
- `<c>`: Einzelner Konsonant
- `<cvc>`: Konsonant-Vokal-Konsonant
- `<cc>`: Zwei Konsonanten
- `|`: Alternative (oder)
- `<x|>`: Optionaler Teil (x oder leer)

## Beispiele

Das System generiert authentisch klingende deutsche Namen wie:
- `hageuw` (6 Zeichen)
- `katiwig` (7 Zeichen)
- `amwiras` (7 Zeichen)
- `larat` (5 Zeichen)
- `bs` (2 Zeichen)

## Phonetische Regeln

Das System verwendet deutsche Phonetik-Regeln:
- Verbotene Sequenzen: `aa`, `ee`, `ii`, `oo`, `uu`
- Bevorzugte Sequenzen: `ch`, `sch`, `st`, `ng`, `nk`, `tz`
- Vokal-Konsonant-Kompatibilität
- Maximale aufeinanderfolgende Vokale: 2
- Maximale aufeinanderfolgende Konsonanten: 3

## Laufzeit

```bash
cargo run --example pattern_demo
```

Dieses Beispiel zeigt die Vielseitigkeit und Flexibilität des Pattern-Systems für die Erstellung realistischer Namen mit kontrollierten Silbenstrukturen.

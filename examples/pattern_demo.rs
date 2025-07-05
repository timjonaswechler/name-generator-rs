use name_generator::categories::complex_pattern_example::*;
use name_generator::core::Name;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    println!("=== Deutsche Namen mit verschiedenen Silbenstrukturen ===\n");
    
    // Ihr spezifisches Pattern demonstrieren:
    // - Start: cv, vc, c oder cvc
    // - Mitte: cv, vc oder nichts (optional)
    // - Ende: cv, cvc, vc, cc oder c
    
    println!("Pattern-Analyse:");
    println!("Start: cv (Konsonant-Vokal), vc (Vokal-Konsonant), c (Konsonant) oder cvc (Konsonant-Vokal-Konsonant)");
    println!("Mitte: cv, vc oder leer (optional)");
    println!("Ende: cv, cvc, vc, cc oder c");
    println!();
    
    // Demo mit mittlerer Komplexität
    println!("🎯 Mittlere deutsche Namen (Ihr gewünschtes Pattern):");
    let generator = Name::<MediumGermanName>::new();
    for i in 0..15 {
        let name = generator.generate(&mut rng);
        println!("   {}: {} (Länge: {})", i + 1, name, name.len());
    }
    
    println!("\n🔥 Komplexere Varianten:");
    let complex_generator = Name::<ComplexGermanName>::new();
    for i in 0..10 {
        let name = complex_generator.generate(&mut rng);
        println!("   {}: {} (Länge: {})", i + 1, name, name.len());
    }
    
    println!("\n💎 Minimale Varianten:");
    let minimal_generator = Name::<MinimalGermanName>::new();
    for i in 0..10 {
        let name = minimal_generator.generate(&mut rng);
        println!("   {}: {} (Länge: {})", i + 1, name, name.len());
    }
    
    println!("\n📊 Statistiken:");
    analyze_patterns(&mut rng);
}

fn analyze_patterns(rng: &mut impl rand::Rng) {
    let medium_generator = Name::<MediumGermanName>::new();
    let complex_generator = Name::<ComplexGermanName>::new();
    let minimal_generator = Name::<MinimalGermanName>::new();
    
    let sample_size = 100;
    
    // Analysiere Längen
    let mut medium_lengths = Vec::new();
    let mut complex_lengths = Vec::new();
    let mut minimal_lengths = Vec::new();
    
    for _ in 0..sample_size {
        medium_lengths.push(medium_generator.generate(rng).len());
        complex_lengths.push(complex_generator.generate(rng).len());
        minimal_lengths.push(minimal_generator.generate(rng).len());
    }
    
    let avg_medium = medium_lengths.iter().sum::<usize>() as f64 / sample_size as f64;
    let avg_complex = complex_lengths.iter().sum::<usize>() as f64 / sample_size as f64;
    let avg_minimal = minimal_lengths.iter().sum::<usize>() as f64 / sample_size as f64;
    
    println!("Durchschnittliche Namenslängen (basierend auf {} Samples):", sample_size);
    println!("  Minimale Namen: {:.1} Zeichen", avg_minimal);
    println!("  Mittlere Namen: {:.1} Zeichen", avg_medium);
    println!("  Komplexe Namen: {:.1} Zeichen", avg_complex);
    
    // Finde kürzeste und längste Namen
    println!("\nLängenverteilung:");
    println!("  Minimale Namen: {}-{} Zeichen", minimal_lengths.iter().min().unwrap(), minimal_lengths.iter().max().unwrap());
    println!("  Mittlere Namen: {}-{} Zeichen", medium_lengths.iter().min().unwrap(), medium_lengths.iter().max().unwrap());
    println!("  Komplexe Namen: {}-{} Zeichen", complex_lengths.iter().min().unwrap(), complex_lengths.iter().max().unwrap());
}

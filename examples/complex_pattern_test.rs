use name_generator::categories::complex_pattern_example::*;
use name_generator::core::Name;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    println!("=== Test der komplexen Pattern-Generierung ===\n");
    
    // Teste alle verschiedenen Pattern-Typen
    println!("1. Einfache deutsche Namen (cv|vc|c|cvc)(cv|cvc|vc|cc|c):");
    let simple_gen = Name::<SimpleGermanName>::new();
    for i in 0..10 {
        let name = simple_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n2. Mittlere deutsche Namen (cv|vc|c|cvc)(cv|vc|)(cv|cvc|vc|cc|c):");
    let medium_gen = Name::<MediumGermanName>::new();
    for i in 0..10 {
        let name = medium_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n3. Komplexe deutsche Namen (cv|vc|c|cvc)(cv|vc|)(cv|vc|)(cv|cvc|vc|cc|c):");
    let complex_gen = Name::<ComplexGermanName>::new();
    for i in 0..10 {
        let name = complex_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n4. Sehr komplexe deutsche Namen:");
    let very_complex_gen = Name::<VeryComplexGermanName>::new();
    for i in 0..10 {
        let name = very_complex_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n5. Minimale deutsche Namen (cv|c)(c|cv):");
    let minimal_gen = Name::<MinimalGermanName>::new();
    for i in 0..10 {
        let name = minimal_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
}

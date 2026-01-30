fn main() {
    println!("Hello, world!");

    // Generate neutrons based on 2^n
    let mut neutron_count: u128 = 1;
    let mut generation = 0_u128;
    loop {
        println!("Neutron count at generation {generation}: {neutron_count}");
        neutron_count *= 2;
        generation += 1;
    }
}

struct Neutron {
    properties: NeutronProperties,
}

struct NeutronProperties {
    energy: Energy, //Measured in eV
}
impl Default for NeutronProperties {
    fn default() -> Self {
        Self {
            energy: Energy(0_f64),
        }
    }
}

struct Energy(f64);

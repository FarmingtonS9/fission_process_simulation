fn main() {
    println!("Hello, world!");

    // U235
    let u_235 = Atom {
        proton: Proton { number: 92 },
        neutron: Neutron { number: 143 },
        electron: Electron { number: 92 },
    };

    println!("Nucleon: {}", u_235.nucleon());

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
    number: u8,
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

struct Proton {
    number: u8,
}

struct Electron {
    number: u8,
}

struct Atom {
    proton: Proton,
    neutron: Neutron,
    electron: Electron,
}
impl Atom {
    fn nucleon(&self) -> u8 {
        self.proton.number + self.neutron.number
    }
}

struct Energy(f64);

// const PROTON: Particle = Particle {
//     p_type: ParticleType::Proton,
//     charge: ChargeType::Positive,
// };
// const NEUTRON: Particle = Particle {
//     p_type: ParticleType::Neutron,
//     charge: ChargeType::Neutral,
// };
// const ELECTRON: Particle = Particle {
//     p_type: ParticleType::Electron,
//     charge: ChargeType::Negative,
// };

fn main() {
    println!("Hello, world!");

    // U235
    let u_235 = Atom {
        proton: Particle::proton(92),
        neutron: Particle::neutron(143),
        electron: Particle::electron(92),
        position: Position::origin(),
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

struct Particle {
    p_type: ParticleType,
    number: u8,
}
impl Particle {
    fn proton(number: u8) -> Self {
        Self {
            p_type: ParticleType::Proton,
            number,
        }
    }
    fn neutron(number: u8) -> Self {
        Self {
            p_type: ParticleType::Neutron,
            number,
        }
    }
    fn electron(number: u8) -> Self {
        Self {
            p_type: ParticleType::Electron,
            number,
        }
    }
    fn positron(number: u8) -> Self {
        Self {
            p_type: ParticleType::Electron,
            number,
        }
    }
}

// May change to hadrons and baryons.
enum ParticleType {
    Proton,
    Neutron,
    Electron,
}

enum ChargeType {
    Positive,
    Negative,
    Neutral,
}

struct Neutron {
    properties: NeutronProperties,
}
struct NeutronProperties {
    energy: Energy, //Measured in eV
    mass: Mass,
    charge: ChargeType,
}
impl Default for NeutronProperties {
    fn default() -> Self {
        Self {
            energy: Energy(0_f64),
            mass: Mass(1_f64),
            charge: ChargeType::Neutral,
        }
    }
}
struct Atom {
    proton: Particle,
    neutron: Particle,
    electron: Particle,
    position: Position,
}
impl Atom {
    fn nucleon(&self) -> u16 {
        (self.proton.number + self.neutron.number).into()
    }
}

struct Energy(f64);

struct Mass(f64);

struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

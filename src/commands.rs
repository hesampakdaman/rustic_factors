use crate::algorithms;
use crate::primality_test;
use crate::traits::PrimalityTest;
use crate::Factorization;
use bnum::types::U512;
use std::collections::BTreeMap;

type Command = Box<dyn Fn(&U512) -> String>;
pub struct Commands(BTreeMap<String, Command>);

impl Commands {
    pub fn execute(&self, input: &U512, name: &str) -> Result<String, String> {
        match self.0.get(name) {
            Some(cmd) => Ok(cmd(&input)),
            None => Err(format!(
                "Unknown algorithm. Available options: {}",
                self.available_methods()
            )),
        }
    }

    fn available_methods(&self) -> String {
        self.0
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl Default for Commands {
    fn default() -> Self {
        let mut commands = Commands(BTreeMap::new());
        commands.0.insert(
            "miller_rabin".to_string(),
            Box::new(|n| {
                format!(
                    "is {} prime? {}",
                    n,
                    primality_test::MillerRabin::is_prime(n)
                )
            }),
        );
        commands.0.insert(
            "pollards_rho".to_string(),
            Box::new(|n| Factorization::new::<algorithms::PollardsRho>(n).to_string()),
        );
        commands.0.insert(
            "fermats_factorization_method".to_string(),
            Box::new(|n| {
                Factorization::new::<algorithms::FermatsFactorizationMethod>(n).to_string()
            }),
        );
        commands.0.insert(
            "trial_division".to_string(),
            Box::new(|n| Factorization::new::<algorithms::TrialDivision>(n).to_string()),
        );
        commands
    }
}

use crate::traits::Command;
use crate::{algorithms, primality_test};
use std::collections::BTreeMap;

type CommandTraitObj = Box<dyn Command>;
pub struct CommandMap(BTreeMap<String, CommandTraitObj>);

impl CommandMap {
    pub fn get(&self, name: &str) -> Result<&CommandTraitObj, String> {
        self.0.get(name).ok_or(format!(
            "Unknown algorithm. Available options: {}",
            self.available_methods()
        ))
    }

    pub fn available_methods(&self) -> String {
        self.0
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl Default for CommandMap {
    fn default() -> Self {
        let mut commands = CommandMap(BTreeMap::new());
        commands.0.insert(
            "fermats_factorization_method".to_string(),
            Box::new(algorithms::FermatsFactorizationMethod),
        );
        commands.0.insert(
            "miller_rabin".to_string(),
            Box::new(primality_test::MillerRabin),
        );
        commands.0.insert(
            "pollards_rho".to_string(),
            Box::new(algorithms::PollardsRho),
        );
        commands.0.insert(
            "trial_division".to_string(),
            Box::new(algorithms::TrialDivision),
        );
        commands
    }
}

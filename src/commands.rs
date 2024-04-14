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

    fn add(mut self, name: &str, cmd: CommandTraitObj) -> Self {
        self.0.insert(name.to_string(), cmd);
        self
    }
}

impl Default for CommandMap {
    fn default() -> Self {
        CommandMap(BTreeMap::new())
            .add(
                "fermats_factorization_method",
                Box::new(algorithms::FermatsFactorizationMethod),
            )
            .add("miller_rabin", Box::new(primality_test::MillerRabin))
            .add("pollards_rho", Box::new(algorithms::PollardsRho))
            .add("trial_division", Box::new(algorithms::TrialDivision))
    }
}

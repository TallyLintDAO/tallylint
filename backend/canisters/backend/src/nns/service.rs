use std::collections::BTreeMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::domain::NeuronProfile;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct NeuronService {
  pub neurons: BTreeMap<String, NeuronProfile>,
}
impl NeuronService {
  // ...

  pub fn search_by_id(&self, target_id: u64) -> Option<&NeuronProfile> {
    for neuron_profile in self.neurons.values() {
      if neuron_profile.id == target_id {
        return Some(neuron_profile);
      }
    }
    None
  }

  pub fn search_by_owner(
    &self,
    target_owner: Principal,
  ) -> Vec<&NeuronProfile> {
    self
      .neurons
      .values()
      .filter(|neuron_profile| neuron_profile.owner == target_owner)
      .collect()
  }
}

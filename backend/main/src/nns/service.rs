use std::collections::BTreeMap;

use candid::Principal;

use super::{api::NeuronId, domain::NeuronProfile};

#[derive(Debug, Default)]
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

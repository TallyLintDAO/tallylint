<template>
  <div class="nns-container">
    <div class="q-pa-md q-gutter-md">
      <span>NNS Neuron</span>
      <br />
      <q-input v-model="hotkey" label="Hotkey" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from "vue"
import {
  GovernanceCanister,
  ListProposalsRequest,
  Option,
  Proposal,
  ProposalRewardStatus,
  ProposalStatus,
  Topic,
} from "@dfinity/nns"

const hotkey = ref()
const proposalList = ref<
  Array<{
    id: Option<bigint>
    status: ProposalStatus
    proposal: Option<Proposal>
  }>
>([])

onMounted(() => {
  // getNNSProposal();
})

const getNNSProposal = () => {
  const neuron = GovernanceCanister.create()
  const listType: ListProposalsRequest = {
    limit: 10,
    beforeProposal: undefined,
    includeRewardStatus: [ProposalRewardStatus.AcceptVotes],
    excludeTopic: [Topic.Unspecified],
    includeStatus: [ProposalStatus.Open],
    includeAllManageNeuronProposals: true,
  }
  neuron
    .listProposals({
      request: listType,
    })
    .then((res) => {
      console.log("getListProposals", res)
      proposalList.value = res.proposals.map(({ id, status, proposal }) => ({
        id,
        status,
        proposal,
      }))
      console.log("proposalList", proposalList.value)
    })
}
</script>

<style lang="scss"></style>

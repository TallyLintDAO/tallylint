<template>
    <div class="nns-container">
        <div class="q-pa-md row items-start q-gutter-md">
            <span>NNS Proposal</span>
            <br/>
            <q-card v-for="item in proposalList" :key="Number(item.id)">
                <q-card-section>
                    <p>ID: {{ item.id }}</p>
                    <p>Status: {{ item.status }}</p>
                    <!--<p>Proposal: {{ item.proposal.title }}</p>-->
                </q-card-section>
            </q-card>
        </div>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted } from 'vue';
    import {
        GovernanceCanister,
        ListProposalsRequest, Option, Proposal,
        ProposalRewardStatus,
        ProposalStatus,
        Topic
    } from "@dfinity/nns";

    const proposalList = ref<
        Array<{
            id: Option<bigint>;
            status: ProposalStatus;
            proposal: Option<Proposal>;
        }>
        >([]);

    onMounted(() => {
        getNNSProposal();
    });

    const getNNSProposal = () => {
        const neuron = GovernanceCanister.create();
        const listType: ListProposalsRequest = {
            limit: 10,
            beforeProposal: undefined,
            includeRewardStatus: [ProposalRewardStatus.AcceptVotes],
            excludeTopic: [Topic.Unspecified],
            includeStatus: [ProposalStatus.Open],
        };
        neuron.listProposals({
            request: listType
        }).then(res => {
            console.log("getListProposals", res)
            proposalList.value = res.proposals.map(({id, status, proposal}) => ({id, status, proposal}));
            console.log("proposalList", proposalList.value)
        })
    }

</script>

<style lang="scss">

</style>

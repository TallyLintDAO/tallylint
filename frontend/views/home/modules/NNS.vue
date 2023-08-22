<template>
    <div class="nns-container">
        <div class="container">

        </div>
    </div>
</template>

<script lang="ts" setup>
    import { onMounted } from 'vue';
    import {
        GovernanceCanister,
        ListProposalsRequest,
        ProposalRewardStatus,
        ProposalStatus,
        Topic
    } from "@dfinity/nns";

    onMounted(() => {
        getNNSProposal();
    });

    const getNNSProposal = () => {
        const neuron = GovernanceCanister.create();
        const listType: ListProposalsRequest = {
            limit: 10,
            beforeProposal: BigInt(0),
            includeRewardStatus:[ProposalRewardStatus.Unknown],
            excludeTopic:[Topic.Unspecified],
            includeStatus: [ProposalStatus.Accepted],
        };
        neuron.listProposals({
            request: listType
        }).then(res => {
            console.log("getListProposals", res)
        })
    }

</script>

<style lang="scss">

</style>

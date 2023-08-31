<template>
    <div class="wallet-container">
        <div class="q-pa-md row items-start q-gutter-md">
            <div v-if="walletList.length===0">
                no data
            </div>
            <div v-else style="width: 100%">
                <q-btn flat color="primary" icon="file_download" label="Export CSV"
                       @click="exportToCSV"/>
                <q-list bordered separator >
                    <q-item v-for="transaction in walletList"
                            :key="transaction.hash"
                            clickable v-ripple="true">
                        <q-item-section>
                            <q-item-label caption>
                                {{ new Date(Number(transaction.timestamp)).toLocaleString() }}
                            </q-item-label>
                            <div class="row">
                                <div class="col">
                                    {{ transaction.type }}
                                </div>
                                <div class="col">
                                    {{ transaction.details?.currency.symbol }}
                                    {{currencyCalculate(transaction.details?.amount,transaction.details?.currency.decimals)}}
                                </div>
                                <div class="col">
                                    <q-icon name="arrow_right_alt" />
                                </div>
                                <div class="col">
                                    {{ showUsername("",transaction.details?.to) }}
                                    <a :href="'https://dashboard.internetcomputer.org/transaction/' + transaction.hash" target="_blank">
                                        <q-icon name="open_in_new" />
                                    </a>
                                </div>
                                <div class="col">
                                    <q-icon name="reorder" />
                                </div>
                            </div>
                        </q-item-section>
                    </q-item>
                </q-list>
            </div>

        </div>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted } from 'vue';
    import { getICPTransactions, InferredTransaction } from "@/api/rosetta";
    import { currencyCalculate, showUsername } from "@/utils/common";

    const address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739";
    const walletList = ref<InferredTransaction[]>([]);

    onMounted(() => {
        getWalletHistory();
    });

    const getWalletHistory = async () => {
        getICPTransactions(address).then(res => {
            console.log("getWalletHistory", res)
            if (res.total && res.total != 0) {
                walletList.value = res.transactions;
            }
        })
    }
    const exportToCSV = async () => {

    }


</script>

<style lang="scss"></style>

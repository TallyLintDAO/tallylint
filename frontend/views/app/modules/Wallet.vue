<template>
    <div class="wallet-container">
        <div class="q-pa-md row items-start q-gutter-md">
            <div v-if="walletList.length===0">
                no data
            </div>
            <div v-else style="width: 100%">
                <q-btn flat color="primary" icon="file_download" label="Export CSV"
                       @click="exportToCSV"/>
                <q-list bordered separator>
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
                                    <q-icon name="arrow_right_alt"/>
                                </div>
                                <div class="col">
                                    {{ showUsername("",transaction.details?.to) }}
                                    <a :href="'https://dashboard.internetcomputer.org/transaction/' + transaction.hash"
                                       target="_blank">
                                        <q-icon name="open_in_new"/>
                                    </a>
                                </div>
                                <div class="col">
                                    <q-icon name="reorder"/>
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
    import { exportFile } from "quasar";
    import { getICPPriceHistory } from "@/api/token";

    const address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739";
    const walletList = ref<InferredTransaction[]>([]);

    onMounted(() => {
        getWalletHistory();
        getICPPriceHistory();
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
        const columnNames = ['Hash', 'Type', 'Status', 'Timestamp', 'From', 'To', 'Amount', 'Fee', 'Memo']
        // 生成包含列名和数据的数组
        const data = [columnNames, ...walletList.value.map(transaction => [
            transaction.hash,
            transaction.type,
            transaction.details?.status,
            transaction.timestamp,
            transaction.details?.from,
            transaction.details?.to,
            transaction.details?.amount,
            transaction.details?.fee?.amount,
        ])];

        // 将数据转换为 CSV 格式的字符串
        const csvContent = data.map(row => row.join(",")).join("\n");

        // 使用 exportFile 函数导出 CSV 文件
        exportFile(address + ".csv", csvContent, "text/csv");
    }


</script>

<style lang="scss"></style>

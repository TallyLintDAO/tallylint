<template>
    <div class="wallet-container">
        <div class="row items-start">
            <div class="header">
                <q-select v-model="model" :options="options" label="Cost Basis Method"/>
                <q-btn flat color="primary" icon="file_download" label="Export CSV"
                       @click="exportToCSV"/>
            </div>
            <div v-if="walletList.length===0">
                <q-spinner-cube size="xl"
                                color="primary"
                />
            </div>

            <div v-else style="width: 100%">

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
                                    {{ transaction.details.currency.symbol }}
                                    {{ transaction.details.amount }}
                                    <br/>
                                    {{'$'+ transaction.details.cost +' cost basis'}}
                                </div>
                                <div class="col">
                                    <q-icon name="arrow_right_alt"/>
                                </div>
                                <div class="col">
                                    {{ showUsername("",transaction.details.to || "") }}
                                    <a :href="'https://dashboard.internetcomputer.org/transaction/' + transaction.hash"
                                       target="_blank">
                                        <q-icon name="open_in_new"/>
                                    </a>
                                    <br/>
                                    {{ '≈ $' +
                                    transaction.details.value +
                                    ' · $ ' +
                                    transaction.details.profit + ' profit'
                                    }}
                                </div>
                                <div class="col">
                                    <q-icon name="more_vert"/>
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
    import { showUsername } from "@/utils/common";
    import { exportFile } from "quasar";
    import { useRoute } from "vue-router";

    const route = useRoute();

    const address = route.params.address;
    const walletList = ref<InferredTransaction[]>([]);
    const options = ['FIFO'];
    const model = ref('FIFO')

    onMounted(async () => {
        getWalletHistory();
    });

    const getWalletHistory = async () => {
        //@ts-ignore TODO 传递进来的可能是单个地址，也可能是多个地址，需处理
        getICPTransactions(address).then(res => {
            console.log("getWalletHistory", res)
            if (res.total && res.total != 0) {
                walletList.value = res.transactions;
            }
        })
    }
    const exportToCSV = async () => {
        const columnNames = ['Hash', 'Type', 'Status', 'Timestamp', 'From', 'To', 'Amount', 'Fee', 'Memo',
            'Price', 'Cost', 'Income', 'Profit']
        // 生成包含列名和数据的数组
        const data = [columnNames, ...walletList.value.map(transaction => [
            transaction.hash,
            transaction.type,
            transaction.details.status,
            new Date(Number(transaction.timestamp)).toLocaleString(),
            transaction.details?.from,
            transaction.details?.to,
            transaction.details.amount,
            transaction.details.fee.amount,
            '',
            transaction.details.price,
            transaction.details.cost,
            transaction.details.value,
            transaction.details.profit,
        ])];

        // 将数据转换为 CSV 格式的字符串
        const csvContent = data.map(row => row.join(",")).join("\n");

        // 使用 exportFile 函数导出 CSV 文件
        exportFile(address + ".csv", csvContent, "text/csv");
    }


</script>

<style lang="scss"></style>

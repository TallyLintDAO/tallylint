<template>
    <div class="transactions-container">
        <div class="column ">
            <div class="header q-gutter-md row q-mb-md">
                <q-select v-model="model" :options="options" label="Cost Basis Method"/>
                <q-btn flat color="primary" icon="file_download" label="Export CSV"
                       @click="exportToCSV"/>
            </div>
            <div v-if="walletList.length===0">
                <q-spinner-cube size="xl" color="primary"/>
            </div>
            <div v-else>
                <q-list bordered separator>
                    <template v-for="(transactions, date) in groupedTransactions" :key="date">
                        <q-item>
                            <q-item-section>
                                <q-item-label caption>{{ date }}</q-item-label>
                            </q-item-section>
                        </q-item>
                        <q-item v-for="transaction in transactions" :key="transaction.hash" clickable v-ripple="true">
                            <!-- transaction log -->
                            <div class="row items-center" style="width: 100%">
                                <div class="col">
                                    <q-icon v-if="transaction.type==='SEND'" class="text-red-5" size="md"
                                            name="arrow_upward"/>
                                    <q-icon v-if="transaction.type==='RECEIVE'" class="text-green-6" size="md"
                                            name="arrow_downward"/>
                                    {{ transaction.type }}
                                </div>
                                <div class="col">
                                    {{ transaction.details.currency.symbol }}
                                    {{ transaction.details.amount }}
                                    <br/>
                                    <span v-if="transaction.type==='SEND'">
                                        {{'$'+ transaction.details.cost +' cost basis'}}
                                    </span>
                                </div>
                                <div class="col">
                                    <q-icon size="md" name="arrow_forward"/>
                                </div>
                                <div class="col">
                                    {{ showUsername("",transaction.details.to || "") }}
                                    <a :href="'https://dashboard.internetcomputer.org/transaction/' + transaction.hash"
                                       target="_blank">
                                        <q-icon name="open_in_new"/>
                                    </a>
                                    <br/>
                                    <span>
                                    {{ '≈ $' +
                                    transaction.details.value}}
                                         <q-tooltip>
                                            This is the market price of the sent coin by CoinGecko
                                             <br/>
                                             ${{transaction.details.price}} / ICP
                                         </q-tooltip>
                                    </span>
                                    <b v-if="transaction.type==='SEND'"
                                       :class="{
                                        'text-green-6': transaction.details.profit > 0,
                                        'text-red-5': transaction.details.profit < 0}">
                                        {{
                                        ' · $ ' +
                                        transaction.details.profit + ' profit'
                                        }}
                                    </b>
                                </div>
                                <div class="col">
                                    <q-icon size="sm" name="more_vert"/>
                                </div>
                            </div>
                        </q-item>
                    </template>
                </q-list>
            </div>

        </div>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted, computed } from 'vue';
    import { getICPTransactions, InferredTransaction } from "@/api/rosetta";
    import { showUsername } from "@/utils/avatars";
    import { exportFile } from "quasar";
    import { useRoute } from "vue-router";

    const route = useRoute();

    const address = route.params.address;
    const walletList = ref<InferredTransaction[]>([]);
    const options = ['FIFO'];
    const model = ref('FIFO')

    const groupedTransactions = computed<{ [key: string]: InferredTransaction[] }>(() => {
        const groups = {};
        walletList.value.forEach((transaction) => {
            const date = new Date(Number(transaction.timestamp)).toLocaleDateString();
            if (!groups[date]) {
                groups[date] = [];
            }
            groups[date].push(transaction);
        });
        console.log("groups", groups)
        return groups;
    });

    onMounted(() => {
        getWalletHistory();
    });

    const getWalletHistory = async () => {
        //@ts-ignore TODO 传递进来的可能是单个地址，也可能是多个地址，需处理
        getICPTransactions(address, true).then(res => {
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

<style lang="scss">
    .transactions-container {
        .header {
            .q-select {
                width: 150px;
            }
        }
    }

</style>

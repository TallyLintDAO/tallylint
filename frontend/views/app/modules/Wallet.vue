<template xmlns:v-slot="http://www.w3.org/1999/XSL/Transform">
    <div class="wallet-container">
        <div class="buttons q-mb-md">
            <q-btn color="primary" @click="addWallet = true">Add Wallet</q-btn>
        </div>
        <q-table
                grid
                title="Wallets"
                :rows="rows"
                :columns="columns"
                row-key="address"
        >
            <template v-slot:top-right>
                <q-btn color="primary" @click="addWallet = true">Add Wallet</q-btn>
            </template>
            <template v-slot:item="props">
                <div class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3">
                    <q-card @click="toDetail(props.row.address)" class="cursor-pointer">
                        <q-list>
                            <q-item v-for="col in props.cols.filter(col => col.name !== 'desc')" :key="col.name">
                                <q-item-section>
                                    <q-item-label>{{ col.label }}</q-item-label>
                                    <q-item-label caption>{{ col.value }}</q-item-label>
                                </q-item-section>
                            </q-item>
                        </q-list>
                    </q-card>
                </div>
            </template>
        </q-table>
        <q-dialog v-model="addWallet" transition-show="flip-down">
            <q-card style="min-width: 350px">
                <q-card-section>
                    <div class="text-h6">Your Wallet</div>
                </q-card-section>
                <q-card-section class="q-pt-none">
                    <q-form @submit="onSubmit"
                            ref="walletForm"
                            class="q-gutter-md">
                        <q-input
                                filled
                                v-model="wallet.address"
                                label="Wallet address *"
                                hint="The correct address allows us to read the"
                                lazy-rules
                                :rules="[ val => val && val.length > 0 || 'Please type something']"
                        />
                        <q-select filled v-model="wallet.type" :options="types" label="Wallet Type"/>
                        <q-input
                                filled
                                v-model="wallet.name"
                                label="Wallet name *"
                                hint="Identify your wallet quickly"
                                lazy-rules
                                :rules="[ val => val && val.length > 0 || 'Please type something']"
                        />
                        <div class="q-gutter-sm justify-end flex">
                            <q-btn flat label="Cancel" v-close-popup="true"/>
                            <q-btn label="Submit" type="submit" color="primary"/>
                        </div>
                    </q-form>
                </q-card-section>
            </q-card>
        </q-dialog>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted } from 'vue';
    import { QForm } from 'quasar';
    import router from "@/router";

    const columns = [
        {
            name: 'address',
            required: true,
            label: 'Address',
            field: row => row.address,
        },
        {name: 'type', label: 'Type', field: 'type'},
        {name: 'name', label: 'Name', field: 'name'},
        {name: 'transactions', label: 'Transactions', field: 'transactions'},
    ]
    const types = ["NNS", "Plug", "Stoic", "AstorMe"]
    const addWallet = ref(false);

    const wallet = ref({
        address: "",
        type: "NNS",
        name: "",
        transactions: 0,
    });
    const walletPrototype = ({
        address: "",
        type: "NNS",
        name: "",
        transactions: 0,
    });
    const walletForm = ref<QForm | null>(null);

    const rows = ref([
        {
            address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739",
            type: "NNS",
            name: "wallet1",
            transactions: 70,
        }
    ])

    const toDetail = (address: string) => {
        router.push('/app/transactions/' + address);
    }

    const onSubmit = () => {
        walletForm.value?.validate().then(success => {
            if (success) {
                rows.value.push({...wallet.value});
                wallet.value = {...walletPrototype};
                addWallet.value = false;
                // walletForm.value?.resetValidation()
            } else {
                // 数据验证失败
                // 用户至少输入了一个无效值
            }
        })
    }

</script>

<style lang="scss">
</style>

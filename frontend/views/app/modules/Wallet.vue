<template>
    <div class="wallet-container">
        <div class="buttons q-mb-md">
            <q-btn color="primary" @click="addWallet = true">Add Wallet</q-btn>
        </div>
        <q-table
                grid
                title="Wallets"
                :rows="rows"
                :columns="columns"
                row-key="name"
        />
        <q-dialog v-model="addWallet" persistent>
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
    const walletForm = ref<QForm | null>(null);

    const rows = ref([
        {
            address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739",
            type: "NNS",
            name: "wallet1",
            transactions: 70,
        }
    ])

    const onSubmit = () => {
        walletForm.value?.validate().then(success => {
            if (success) {
                rows.value.push(wallet.value);
                console.log("rows", rows.value)
                walletForm.value?.resetValidation()
            } else {
                // 数据验证失败
                // 用户至少输入了一个无效值
            }
        })
    }

</script>

<style lang="scss" scoped>

</style>

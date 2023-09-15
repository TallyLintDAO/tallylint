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
                v-model:selected="selected"
                row-key="address"
        >
            <template v-slot:top-right >
                <div class="q-gutter-md">
                    <q-btn color="primary" @click="addWallet = true">Add Wallet</q-btn>
                    <q-btn color="red" disable >Remove Wallet</q-btn>
                </div>
            </template>
            <template v-slot:item="props">
                <div class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition">
                    <q-card @click="toDetail(props.row.address)" class="cursor-pointer"
                            :class="props.selected ? 'bg-grey-2' : ''">
                        <q-card-section>
                            <q-card-section>
                                <q-checkbox dense v-model="selected" :val="props.row.address" :label="props.row.name"/>
                            </q-card-section>
                            <div class="text-h6">{{props.row.name}}</div>
                            <q-list>
                                <q-item v-for="col in props.cols.filter(col => col.name !== 'desc')" :key="col.name">
                                    <q-item-section>
                                        <q-item-label>{{ col.label }}</q-item-label>
                                        <q-item-label caption>{{ col.value }}</q-item-label>
                                    </q-item-section>
                                </q-item>
                            </q-list>
                        </q-card-section>
                    </q-card>
                </div>
            </template>
        </q-table>
        <q-dialog v-model="addWallet">
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
                        <q-select filled v-model="wallet.from" :options="froms" label="From"/>
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
                            <q-btn :loading="loading" label="Submit" type="submit" color="primary"/>
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
    import { addUserWallet, getUserWallet } from "@/api/user";

    const columns = [
        {
            name: 'address',
            required: true,
            label: 'Address',
            field: row => row.address,
        },
        {name: 'from', label: 'Type', field: 'from'},
        {name: 'name', label: 'Name', field: 'name'},
        {name: 'transactions', label: 'Transactions', field: 'transactions'},
    ]
    const froms = ["NNS", "Plug", "Stoic", "AstorMe"]
    const addWallet = ref(false);
    const loading = ref(false);
    const selected = ref([]);

    const wallet = ref({
        address: "",
        from: "NNS",
        name: "",
        transactions: 0,
    });
    const walletPrototype = ({
        address: "",
        from: "NNS",
        name: "",
        transactions: 0,
    });
    const walletForm = ref<QForm | null>(null);

    const rows = ref([
        {
            address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739",
            from: "NNS",
            name: "wallet1",
            transactions: 70,
        }
    ])

    onMounted(() => {
        getWallets();
    });

    const toDetail = (address: string) => {
        router.push('/app/transactions/' + address);
    }

    const getWallets = () => {
        getUserWallet().then((res) => {
            console.log("getUserWallet", res)
        })
    }

    const onSubmit = async () => {
        loading.value = true;
        const validationSuccess = await walletForm.value?.validate();
        if (validationSuccess) {
            // const {address, name, from} = wallet.value;
            // const res = await addUserWallet(address, name, from);
            // console.log("wallet res", res)
            // if (res.Ok) {
                rows.value.push({...wallet.value});
                wallet.value = {...walletPrototype};
                addWallet.value = false;
            // }
            // reset方法好像没效果，待测试。
            // walletForm.value?.resetValidation()
        } else {
            // 数据验证失败
            // 用户至少输入了一个无效值
        }
        loading.value = false;
    }

</script>

<style lang="scss">
    .grid-style-transition {
        transition: transform .28s, background-color .28s
    }
</style>

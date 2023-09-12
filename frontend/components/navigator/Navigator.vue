<template>
    <div class="navigator-container">
        <q-toolbar class="navbar">
            <img alt="logo" src="@/assets/dfinity.svg" class="cursor-pointer" @click="onHome"/>
            <q-toolbar-title>
                TaxLint
            </q-toolbar-title>
        </q-toolbar>
    </div>
</template>
<script lang="ts" setup>
    import { ref } from 'vue';
    import { useRouter } from 'vue-router';

    const router = useRouter();
    const onHome = () => router.push('/');

</script>
<style lang="scss" scoped>
    .navigator-container {
        width: 100%;
        height: 63px;
        display: flex;
        background-color: #ffffff !important;
        box-shadow: 0px 0px 10px 10px rgb(36 51 54 / 5%);
        .navbar {
            width: 1200px;
            margin: auto;
            padding: 0 10px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
    }
</style>

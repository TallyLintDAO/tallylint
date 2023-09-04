<template>
    <q-layout view="lHh lpR lFr">
        <q-header elevated class="bg-primary text-white">
            <q-toolbar>
                <q-btn dense flat round icon="menu" @click="toggleLeftDrawer"/>
                <q-toolbar-title>
                    <q-avatar>
                        <img src="@/assets/dfinity.svg">
                    </q-avatar>
                    IC TaxLint
                </q-toolbar-title>
            </q-toolbar>
        </q-header>

        <q-drawer v-model="leftDrawerOpen" side="left" show-if-above :breakpoint="430">
            <q-img class="absolute-top" src="https://cdn.quasar.dev/img/material.png" style="height: 150px">
                <div class="absolute-bottom bg-transparent">
                    <q-avatar size="56px" class="q-mb-sm">
                        <img src="https://cdn.quasar.dev/img/boy-avatar.png">
                    </q-avatar>
                    <div class="text-weight-bold">Razvan Stoenescu</div>
                    <div>@rstoenescu</div>
                </div>
            </q-img>
            <q-scroll-area style="height: calc(100% - 150px); margin-top: 150px; border-right: 1px solid #ddd">
                <q-list padding>
                    <q-item active clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="inbox"/>
                        </q-item-section>
                        <q-item-section>
                            Wallet
                        </q-item-section>
                    </q-item>
                    <q-item clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="star"/>
                        </q-item-section>
                        <q-item-section>
                            Star
                        </q-item-section>
                    </q-item>

                    <q-item clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="send"/>
                        </q-item-section>
                        <q-item-section>
                            Send
                        </q-item-section>
                    </q-item>
                    <q-item clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="drafts"/>
                        </q-item-section>
                        <q-item-section>
                            Drafts
                        </q-item-section>
                    </q-item>
                </q-list>
            </q-scroll-area>
        </q-drawer>

        <q-page-container>
            <router-view/>
        </q-page-container>

    </q-layout>
</template>

<script lang="ts" setup>
    import { ref } from 'vue'

    const leftDrawerOpen = ref(false)
    const toggleLeftDrawer = () => {
        leftDrawerOpen.value = !leftDrawerOpen.value
    }

</script>

<style lang="scss">
    .home-container {
        /*background-color: rgb(246, 246, 246);*/
        min-height: 100vh;
        /*color: black;*/
        /*width: 100%;*/
        /*height: 100%;*/
    }
</style>

<template>
    <q-layout view="lHh lpR lFr" class="home-container">
        <q-header bordered class="open-header text-dark">
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
                    <div class="text-weight-bold">User 1</div>
                    <div>@user</div>
                </div>
            </q-img>
            <q-scroll-area style="height: calc(100% - 150px); margin-top: 150px; border-right: 1px solid #ddd">
                <q-list padding>
                    <q-item clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="drafts"/>
                        </q-item-section>
                        <q-item-section>
                            Dashboard
                        </q-item-section>
                    </q-item>
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
                            NNS
                        </q-item-section>
                    </q-item>

                    <q-item clickable v-ripple="true">
                        <q-item-section avatar>
                            <q-icon name="send"/>
                        </q-item-section>
                        <q-item-section>
                            Email Set
                        </q-item-section>
                    </q-item>

                </q-list>
            </q-scroll-area>
        </q-drawer>

        <q-page-container>
            <div class="q-pa-md q-gutter-md">
                <router-view/>
            </div>
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
        min-height: 100vh;
        .q-header {
            background-color: #0000001a;
            -webkit-backdrop-filter: blur(7px);
            backdrop-filter: blur(7px);
        }
    }
</style>

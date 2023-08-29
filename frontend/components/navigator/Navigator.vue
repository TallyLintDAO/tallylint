<template>
    <div class="navigator-container">
        <nav class="navbar">
            <div class="logo">
                <img alt="logo" src="@/assets/dfinity.svg" @click="onHome"/>
            </div>
            <div class="nav-right">
                <!-- username -->
                <q-btn v-if="signedIn" color="primary" :label=showedUsername
                       @mouseover="showing=true"
                       @mouseout="showing=false">
                    <q-menu v-model="showing">
                        <q-item clickable v-close-popup="true">
                            <q-item-section @click="onLogOut">Logout</q-item-section>
                        </q-item>
                    </q-menu>
                </q-btn>
                <q-btn v-else color="white" text-color="black" class="login-button" @click="onLogin">
                    Login
                </q-btn>
            </div>
        </nav>
    </div>
</template>
<script lang="ts" setup>
    import { ref, watch, computed, onMounted } from 'vue';
    import { useRouter } from 'vue-router';
    import { initAuth, signIn, signOut } from "@/api/auth";
    import { showUsername } from "@/utils/common";

    const router = useRouter();

    const showing = ref(false);
    // 与 II 认证相关的信息
    const clientReady = ref(false);
    const signedIn = ref(false); // 是否登录
    const userPrincipal = ref();

    const onHome = () => router.push('/');

    const showedUsername = computed<string>(() => {
        if (!signedIn.value) return ''; // 没有登录返回空，按道理显示登录按钮不会调用本方法的
        let name = '';
        // if (userInfo.value.name) name = userInfo.value.name;
        return showUsername(name, userPrincipal.value);
    });

    const onLogin = async () => {
        const auth = await initAuth();
        signIn(auth.client) // 理论上有链接对象才会进入这个方法
            .then((ii) => {
                console.log("ii", ii)
                signedIn.value = true;
                auth.info = ii
                console.log("ii", ii.principal)
                userPrincipal.value = ii.principal;
                // 每次成功获取到登录信息后就调用一次注册
                // setCurrentIdentity(ii.identity, ii.principal);
            })
            .catch((e) => {
                console.error("e", e)
            });
    };

    const onLogOut = async () => {
        console.log("onLogout")
        const auth = await initAuth();
        signedIn.value = false;
        // clearCurrentIdentity();
        await signOut(auth.client);
    };

</script>
<style lang="scss" scoped>
    .navigator-container {
        color: white;
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

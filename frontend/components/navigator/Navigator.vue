<template>
    <div class="navigator-container">
        <nav class="navbar">
            <div class="logo">
                <img alt="logo" src="@/assets/dfinity.svg" @click="onHome"/>
                TaxLint
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
                <q-btn v-else color="white" text-color="black" class="login-button" @click="onLogin"
                       :loading="loading">
                    Login
                </q-btn>
            </div>
        </nav>
    </div>
</template>
<script lang="ts" setup>
    import { ref, computed, onMounted } from 'vue';
    import { useRouter } from 'vue-router';
    import { initAuth, signIn, signOut } from "@/api/auth";
    import { showUsername } from "@/utils/common";
    import { setCurrentIdentity } from "@/api/canister_pool";
    import { useUserStore } from "@/stores/user";
    import { getUserAutoRegister } from "@/api/user";

    const router = useRouter();
    const userStore = useUserStore();

    const showing = ref(false);
    // 与 II 认证相关的信息
    const clientReady = ref(false);
    const signedIn = ref(false); // 是否登录
    const userPrincipal = ref();


    const loading = ref(false);

    const onHome = () => router.push('/');

    const showedUsername = computed<string>(() => {
        if (!signedIn.value) return ''; // 没有登录返回空，按道理显示登录按钮不会调用本方法的
        let name = '';
        // if (userInfo.value.name) name = userInfo.value.name;
        return showUsername(name, userPrincipal.value);
    });

    const onLogin = async () => {
        const auth = await initAuth();
        loading.value = true;
        signIn(auth.client) // 理论上有链接对象才会进入这个方法
            .then((ii) => {
                signedIn.value = true;
                auth.info = ii
                userPrincipal.value = ii.principal;
                // 保存登录状态到actor，方便调用
                setCurrentIdentity(ii.identity, ii.principal);
                // 保存 principal 到状态
                userStore.setPrincipal(ii.principal).then(() => {
                    // 每次成功获取到登录信息后就调用一次注册
                    getUserInfoFromServices();
                });
            })
            .catch((e) => {
                console.error("e", e)
            }).finally(()=>{
            loading.value = false;
        });
    };

    //从后台获取用户信息，并且设置
    const getUserInfoFromServices = () => {
        getUserAutoRegister()
            .then((info) => {
                console.log('get user info', info);
                if (info.Ok) {

                } else if (info.Err) {
                    console.error('no information for unregister user: ', info);
                } else {
                    throw new Error("info not ok & info not err") ;
                }
            })
            .catch((e) => {
                console.error('mounted get user info failed', e);
                onLogOut();
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

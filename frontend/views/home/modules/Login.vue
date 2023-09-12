<template>
    <div class="container login-container q-mt-lg">
        <h5>Please login to access our app</h5>
        <q-btn color="primary" class="login-button" @click="onLogin"
               :loading="loading" no-caps>
            Launch app
        </q-btn>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted } from 'vue';
    import { initAuth, signIn } from "@/api/auth";
    import { setCurrentIdentity } from "@/api/canister_pool";
    import { useUserStore } from "@/stores/user";
    import { getUserAutoRegister } from "@/api/user";
    import { useRouter } from "vue-router";

    const router = useRouter();
    const userStore = useUserStore();

    // 与 II 认证相关的信息
    const clientReady = ref(false);
    const signedIn = ref(false); // 是否登录

    const loading = ref(false);

    const onLogin = async () => {
        const auth = await initAuth();
        loading.value = true;
        signIn(auth.client) // 理论上有链接对象才会进入这个方法
            .then((ii) => {
                signedIn.value = true;
                auth.info = ii
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
            }).finally(() => {
            loading.value = false;
        });
    };

    //从后台获取用户信息，并且设置
    const getUserInfoFromServices = () => {
        getUserAutoRegister()
            .then((info) => {
                console.log('get user info', info);
                if (info.Ok) {
                    router.push({
                        path: '/app',
                    });
                } else if (info.Err) {
                    console.error('no information for unregister user: ', info);
                } else {
                    throw new Error("info not ok & info not err");
                }
            })
            .catch((e) => {
                console.error('mounted get user info failed', e);
            });
    };
</script>

<style lang="scss" scoped>
    .login-container {
    }
</style>

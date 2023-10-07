<template>
    <div class="container login-container q-mt-lg">
        <div class="description row">
            <div class="col">
                <h4 style="margin-bottom: 0">Welcome to TaxLint🧾</h4>
                <span class="text-body1">TaxLint is designed to help users of ICPs to count and manage their tax
                information more easily.With TaxLint, users will be able to record,
                track and calculate tax information related to their transactions and assets in the IC ecosystem.</span>
                <br/>
                <span class="text-body1">In TaxLint, which is 100% on-chain, users get a decentralised,
                private and secure tax experience.</span>
            </div>
            <div class="col">
                <q-img src="@/assets/images/profit.png"></q-img>
            </div>
        </div>
        <h5 style="margin: 0">Login to access our app</h5>
        <div class="q-gutter-sm row items-center">
            <q-btn color="primary" class="login-button" @click="onLogin"
                   :loading="loading" no-caps>
                Launch app
            </q-btn>
        </div>
        <div class="q-mt-sm">
            <a href="https://discord.gg/2q72rBYB5C" target="_Blank">
                <q-icon class="cursor-pointer" size="md" name="discord"/>
            </a>
            <q-chip icon="bookmark">give us more suggestions!</q-chip>
        </div>
        <!--<q-banner inline-actions class="text-white bg-red q-mt-md">-->
            <!--Alpha Warning: there may be issues.-->
        <!--</q-banner>-->
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
                    //直接跳转到应用中，在应用里获取userInfo，加快速度。
                    router.push({
                        path: '/app',
                    });
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
                console.error('mounted get user info failed: ', e);
            });
    };
</script>

<style lang="scss" scoped>
    .login-container {
    }
</style>

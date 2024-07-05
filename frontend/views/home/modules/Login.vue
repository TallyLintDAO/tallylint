<template>
  <div class="container login-container q-my-lg">
    <div class="description row">
      <div class="col-6 col-xs-12">
        <h4 style="margin-bottom: 0">Welcome to TaxLint🧾</h4>
        <span class="text-body1"
          >TaxLint is designed to help users of ICPs to count and manage their
          tax information more easily.With TaxLint, users will be able to
          record, track and calculate tax information related to their
          transactions and assets in the IC ecosystem.</span
        >
        <br />
        <span class="text-body1"
          >In TaxLint, which is 100% on-chain, users get a decentralised,
          private and secure tax experience.</span
        >
        <div class="button-container">
          <q-btn
            color="primary"
            class="login-button"
            @click="onLogin()"
            :loading="loading"
            no-caps
          >
            Launch app
          </q-btn>
        </div>
      </div>
      <div class="col-6 col-xs-12 order-xs-first">
        <q-img src="@/assets/images/profit.png"></q-img>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth"
import { setCurrentIdentity } from "@/api/canister_pool"
import { getUserAutoRegister } from "@/api/user"
import { useUserStore } from "@/stores/user"
import { ref } from "vue"
import { useRouter } from "vue-router"

const router = useRouter()
const userStore = useUserStore()

// 与 II 认证相关的信息
const signedIn = ref(false) // 是否登录

const loading = ref(false)

const onLogin = async () => {
  const auth = await initAuth()
  loading.value = true
  //TODO 先不使用登录缓存，有点问题
  // if (!auth.info) {
  //检查用户是否已登录，未登录就登录
  signIn(auth.client) // 理论上有链接对象才会进入这个方法
    .then((ii) => {
      signedIn.value = true
      auth.info = ii
      loginSuccess(ii)
    })
    .catch((e) => {
      console.error("e", e)
    })
    .finally(() => {
      loading.value = false
    })
  // } else {
  //   //存在auth.info，说明用户已登录，不需要再登录
  //   loginSuccess(auth.info)
  // }
}

const loginSuccess = (ii: IdentityInfo) => {
  // 保存登录状态到actor，方便调用
  setCurrentIdentity(ii.identity, ii.principal)
  // 保存 principal 到状态
  userStore.setPrincipal(ii.principal).then(() => {
    //直接跳转到应用中，在应用里获取userInfo，加快速度。
    router.push({
      path: "/app",
    })
  })
}

//从后台获取用户信息，并且设置
const getUserInfoFromServices = () => {
  getUserAutoRegister()
    .then((info) => {
      console.log("get user info", info)
      if (info.Ok) {
        router.push({
          path: "/app",
        })
      } else if (info.Err) {
        console.error("no information for unregister user: ", info)
      } else {
        throw new Error("info not ok & info not err")
      }
    })
    .catch((e) => {
      console.error("mounted get user info failed: ", e)
    })
}
</script>

<style lang="scss" scoped>
.login-container {
  .description {
    position: relative;
  }
  .button-container {
    position: absolute;
    bottom: 0;
    left: 0;
  }
}
</style>

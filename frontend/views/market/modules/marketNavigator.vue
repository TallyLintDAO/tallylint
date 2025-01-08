<template>
  <div class="navigator-container">
    <q-toolbar class="navbar">
      <img
        alt="logo"
        src="@/assets/tallylint.png"
        class="cursor-pointer"
        @click="onHome"
      />
      <q-btn
        v-if="!signedIn"
        color="primary"
        class="login-button"
        @click="onLogin()"
        :loading="loading"
        no-caps
      >
        Log In
      </q-btn>
      <div v-else>
        <q-btn flat round color="primary">
          <q-avatar color="primary" text-color="white">
            {{ showAvatar }}
          </q-avatar>
          <q-icon name="expand_more" size="sm"></q-icon>
          <!-- <q-icon name="expand_less" size="sm"></q-icon> -->
          <!-- <div>
            {{ showUser }}
          </div> -->
          <q-menu
            transition-show="jump-down"
            transition-hide="jump-up"
            class="q-elevation-8"
          >
            <q-list style="min-width: 200px">
              <div class="q-pa-md q-gutter-sm row items-center">
                <q-avatar color="primary" text-color="white">
                  {{ showAvatar }}
                </q-avatar>

                <div class="q-ml-sm">
                  <div class="text-h6">{{ showUser }}</div>
                  <div class="text-caption text-grey-6">
                    {{ showPId }}
                    <q-icon
                      name="content_copy"
                      class="cursor-pointer"
                      @click="copyPid()"
                    />
                  </div>
                </div>
              </div>
              <q-separator />
              <q-item clickable v-close-popup>
                <q-item-section>Profile</q-item-section>
              </q-item>
              <q-item clickable v-close-popup>
                <q-item-section>Settings</q-item-section>
              </q-item>
              <q-separator />
              <q-item clickable v-close-popup>
                <q-item-section>Log Out</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
      </div>
    </q-toolbar>
    <div class="nav-tab">
      <q-tabs v-model="tab" narrow-indicator dense align="justify" class="">
        <q-tab :ripple="false" name="all" label="All" />
        <q-tab :ripple="false" name="icp" label="ICP" />
        <q-tab :ripple="false" name="eth" label="ETH" />
      </q-tabs>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn, signOut } from "@/api/auth"
import { clearCurrentIdentity, setCurrentIdentity } from "@/api/canister_pool"
import { getUserAutoRegister } from "@/api/user"
import { useUserStore } from "@/stores/user"
import {
  extractColorByName,
  showAvatarName,
  showUsername,
} from "@/utils/avatars"
import { showMessageError, showMessageSuccess } from "@/utils/message"
import { copyToClipboard } from "quasar"
import { computed, onMounted, ref } from "vue"
import { useRouter } from "vue-router"

const router = useRouter()
const userStore = useUserStore()
const onHome = () => router.push("/")
// 与 II 认证相关的信息
const signedIn = ref(false) // 是否登录
const loading = ref(false)

const tab = ref("mails")
const username = ref("")

const principal = computed(() => userStore.principal)

onMounted(() => {
  doInitAuth()
})

const doInitAuth = () => {
  initAuth().then((ai) => {
    if (ai.info) {
      signedIn.value = true
      setCurrentIdentity(ai.info.identity, ai.info.principal)
      // 保存 principal 到用户信息状态
      userStore.setPrincipal(ai.info.principal).then(() =>
        // 获取用户信息
        getUserInfoFromServices(),
      )
    }
  })
}

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
  //获取登录信息
  signedIn.value = true
  console.log("loginSuccess", signedIn.value, ii)
  // 保存 principal 到用户信息状态
  userStore.setPrincipal(ii.principal).then(() =>
    // 获取用户信息
    getUserInfoFromServices(),
  )
}

//从后台获取用户信息，并且设置
const getUserInfoFromServices = () => {
  getUserAutoRegister()
    .then((info) => {
      if (info.Ok) {
        username.value = info.Ok.name
      } else if (info.Err) {
        console.error("no information for unregister user: ", info)
      } else {
        throw new Error("info not ok & info not err")
      }
    })
    .catch((e) => {
      console.error("mounted get user info failed: ", e)
      showMessageError("mounted get user info failed: " + e)
      onLogOut()
    })
}

const onLogOut = async () => {
  console.log("onLogout")
  const auth = await initAuth()
  signedIn.value = false
  clearCurrentIdentity()
  await signOut(auth.client)

  // TODO 返回首页还要刷新页面，感觉不是很友好
  //返回首页后，刷新页面，防止出现缓存问题。
  // 如果不刷新页面，会导致A用户登出后，再登录B用户的账号，结果会读取A用户缓存的问题
  setTimeout(() => {
    window.location.reload()
  }, 500)
}

const copyPid = () => {
  copyToClipboard(principal.value)
    .then(() => {
      showMessageSuccess(`copy ${principal.value} success`)
    })
    .catch(() => {
      showMessageError("copy failed")
    })
}

const showAvatar = computed<string>(() => {
  const m = showAvatarName(username.value, principal.value)
  return m ? m : "A"
})
// 根据名字，定义头像颜色
const backgroundColor = computed<string>(() => {
  return extractColorByName(username.value)
})
// 根据名字，定义用户名
const showUser = computed<string>(() => {
  return showUsername(username.value, principal.value)
})
// 展示缩写的principal id
const showPId = computed<string>(() => {
  return showUsername("", principal.value)
})
</script>
<style lang="scss" scoped>
.navigator-container {
  width: 100%;
  display: flex;
  flex-wrap: wrap;
  background-color: #ffffff !important;
  box-shadow: 0px 0px 10px 10px rgb(36 51 54 / 5%);
  .navbar {
    // width: 1200px;
    height: 63px;
    margin: auto 10px;
    padding: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    img {
      height: 51px;
    }
  }
  .nav-tab {
    margin-left: 40px;
  }
}
</style>

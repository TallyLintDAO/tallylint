<template>
  <q-layout view="lHh lpR lFr" class="home-container">
    <q-header bordered class="open-header text-dark">
      <q-toolbar>
        <q-btn dense flat round icon="menu" @click="toggleLeftDrawer" />
        <q-toolbar-title>
          <q-avatar>
            <img src="@/assets/dfinity.svg" />
          </q-avatar>
          IC TaxLint
        </q-toolbar-title>
        <q-btn color="primary" label="Logout" @click="onLogOut()" />
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      show-if-above
      :breakpoint="430"
    >
      <q-img
        class="absolute-top"
        src="@/assets/material.png"
        style="height: 150px"
      >
        <div class="absolute-bottom bg-transparent">
          <q-avatar size="56px" class="q-mb-sm" :style="{ backgroundColor }">
            {{ showAvatar }}
          </q-avatar>
          <div class="text-weight-bold">{{ showUser }}</div>
          <div>@user</div>
        </div>
      </q-img>
      <q-scroll-area
        style="
          height: calc(100% - 150px);
          margin-top: 150px;
          border-right: 1px solid #ddd;
        "
      >
        <q-list padding>
          <q-item
            v-for="(item, index) in menuItems"
            :key="index"
            clickable
            v-ripple="true"
            :to="item.route"
            :active="item.route === $route.path"
          >
            <q-item-section avatar>
              <q-icon :name="item.icon" />
            </q-item-section>
            <q-item-section>
              {{ item.label }}
            </q-item-section>
          </q-item>
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <div class="q-pa-md q-gutter-md">
        <router-view />
      </div>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed } from "vue"
import { initAuth, signOut } from "@/api/auth"
import { clearCurrentIdentity, setCurrentIdentity } from "@/api/canister_pool"
import { getUserAutoRegister } from "@/api/user"
import { useUserStore } from "@/stores/user"
import {
  extractColorByName,
  showAvatarName,
  showUsername,
} from "@/utils/avatars"
import { goHome } from "@/router/routers"
import { useRouter } from "vue-router"

const userStore = useUserStore()
const router = useRouter()

const menuItems = [
  { icon: "drafts", label: "Dashboard", route: "/app" },
  { icon: "inbox", label: "Wallet", route: "/app/wallet" },
  { icon: "star", label: "NNS", route: "/app/nns" },
  { icon: "send", label: "Email Set", route: "" },
]
const leftDrawerOpen = ref(false)
// 与 II 认证相关的信息
const clientReady = ref(false)
const signedIn = ref(false) // 是否登录

const principal = computed(() => userStore.principal)
const username = ref()

onMounted(() => {
  doInitAuth()
})

const doInitAuth = () => {
  initAuth().then((ai) => {
    clientReady.value = true
    if (ai.info) {
      setCurrentIdentity(ai.info.identity, ai.info.principal)
      // 保存 principal 到用户信息状态
      userStore.setPrincipal(ai.info.principal).then(() =>
        // 获取用户信息
        getUserInfoFromServices(),
      )
    }
  })
}

//从后台获取用户信息，并且设置
const getUserInfoFromServices = () => {
  getUserAutoRegister()
    .then((info) => {
      console.log("APP get user info", info)
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
      onLogOut()
    })
}

const onLogOut = async () => {
  console.log("onLogout")
  const auth = await initAuth()
  signedIn.value = false
  clearCurrentIdentity()
  await signOut(auth.client)
  goHome(router)
}

const toggleLeftDrawer = () => {
  leftDrawerOpen.value = !leftDrawerOpen.value
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

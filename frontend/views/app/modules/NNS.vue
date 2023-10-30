<template>
  <div class="nns-container">
    <div class="q-pa-md q-gutter-md">
      <q-table
        grid
        title="NNS"
        :rows="rows"
        :columns="columns"
        selection="multiple"
        :filter="filter"
        row-key="address"
      >
        <template v-slot:top>
          <div class="q-gutter-sm">
            <q-btn color="primary" @click="addNNSVisible = true"
              >Add NNS Address</q-btn
            >
            <el-tooltip effect="dark" placement="top-start">
              <template #content>
                Your Principal ID is {{ principal }}
                <q-icon
                  name="content_copy"
                  class="cursor-pointer"
                  @click="copyPid()"
                />
              </template>
              <q-btn
                flat
                color="primary"
                label="Help"
                icon="lightbulb_outline"
                @click="goHelp()"
              />
            </el-tooltip>
          </div>

          <q-space />
          <q-input
            borderless
            dense
            debounce="300"
            v-model="filter"
            placeholder="Search"
          >
            <template v-slot:append>
              <q-icon name="search" />
            </template>
          </q-input>
        </template>
        <template v-slot:item="props">
          <div
            class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition"
          >
            <q-card>
              <q-card-section>
                <q-card-section>
                  <div class="row justify-between items-center">
                    <div class="flex q-gutter-xs">
                      <img
                        class="head-icon"
                        src="@/assets/dfinity.svg"
                        alt="NNS Icon"
                      />
                      <div class="text-h6">{{ props.row.name }}</div>
                    </div>
                    <q-btn flat icon="more_vert">
                      <q-menu>
                        <q-list style="min-width: 100px">
                          <!-- <q-item clickable v-close-popup="true">
                            <q-item-section> Delete </q-item-section>
                          </q-item> -->
                        </q-list>
                      </q-menu>
                    </q-btn>
                  </div>
                </q-card-section>
                <q-list>
                  <q-item v-for="col in props.cols" :key="col.name">
                    <q-item-section>
                      <q-item-label>{{ col.label }}</q-item-label>
                      <q-item-label v-if="col.name === 'address'" caption>
                        <router-link :to="'/app/transactions/' + col.value">
                          {{ col.value }}
                        </router-link>
                      </q-item-label>
                      <q-item-label v-else caption>{{
                        col.value
                      }}</q-item-label>
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-card-section>
            </q-card>
          </div>
        </template>
      </q-table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { initAuth } from "@/api/auth"
import { DOCS_URL, NNS_HELP } from "@/api/constants/docs"
import { useUserStore } from "@/stores/user"
import { showMessageError, showMessageSuccess } from "@/utils/message"
import { HttpAgent } from "@dfinity/agent"
import { GovernanceCanister } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import { SnsWrapper, initSnsWrapper } from "@dfinity/sns"
import { copyToClipboard } from "quasar"
import { onMounted, ref } from "vue"

const userStore = useUserStore()

const addNNSVisible = ref(false)
const filter = ref("") //搜索框
const principal = ref(useUserStore().principal) //搜索框
const columns = [
  {
    name: "address",
    required: true,
    label: "Address",
    field: (row) => row.address,
  },
  { name: "id", label: "Neuron Id", field: "id" },
  { name: "maturity", label: "Maturity", field: "maturity" },
  { name: "stakedMaturity", label: "StakedMaturity", field: "stakedMaturity" },
]

const rows = ref<any[]>([])

onMounted(() => {
  getNNS()
  // getSNS()
})

const getNNS = async () => {
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    console.log("agent", agent)
    const neuron = GovernanceCanister.create({
      agent: agent,
    })

    neuron.listNeurons({ certified: false }).then((res) => {
      console.log("getListNeurons", res)
      if (res.length > 0) {
        for (const neuron of res) {
          if (neuron.fullNeuron) {
            const {
              id,
              accountIdentifier,
              maturityE8sEquivalent,
              stakedMaturityE8sEquivalent,
            } = neuron.fullNeuron
            const neuronData = {
              id: id,
              address: accountIdentifier,
              //1e8是10的八次方，除以1e8得到原数
              maturity: Number(maturityE8sEquivalent) / 1e8,
              stakedMaturity: Number(stakedMaturityE8sEquivalent) / 1e8,
            }
            console.log("neuronData", neuronData)
            rows.value.push(neuronData)
          }
        }
      }
    })
  }
}
const getSNS = async () => {
  const snsWrapper: SnsWrapper = await initSnsWrapper({
    rootOptions: {
      //根容器id在dashboard
      canisterId: Principal.fromText("3e3x2-xyaaa-aaaaq-aaalq-cai"),
    },
  })

  const { metadata: meta, swapState } = snsWrapper
  const [metadata, token] = await meta({})

  console.log("Summary data:", metadata, token)
}
const goHelp = () => {
  // To DOCS: NNS Help
  window.open(DOCS_URL + NNS_HELP)
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
</script>

<style lang="scss">
.nns-container {
  .head-icon {
    width: 32px !important;
  }
}
</style>

<template>
  <div class="nns-container">
    <div class="q-pa-md q-gutter-md">
      <q-btn label="Help" color="primary" @click="help = true" />
      <br />
      <span
        >Please enter the principal() of the current account of the neuron to be
        calculated into the neuron hotkey in nns.ic0.app
      </span>
      <br />
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
            <q-btn
              flat
              color="primary"
              label="Help"
              icon="lightbulb_outline"
              @click="goHelp()"
            />
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
  <q-dialog v-model="help">
    <q-card>
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Close icon</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup="true" />
      </q-card-section>

      <q-card-section>
        <q-stepper v-model="step" ref="stepper" color="primary" animated>
          <q-step
            :name="1"
            title="Select campaign settings"
            icon="settings"
            :done="step > 1"
          >
            Please enter the principal() of the current account of the neuron to
            be calculated into the neuron hotkey in nns.ic0.app
          </q-step>

          <q-step
            :name="2"
            title="Create an ad group"
            caption="Optional"
            icon="create_new_folder"
            :done="step > 2"
          >
            An ad group contains one or more ads which target a shared set of
            keywords.
          </q-step>

          <q-step :name="3" title="Ad template" icon="assignment">
            This step won't show up because it is disabled.
          </q-step>

          <q-step :name="4" title="Create an ad" icon="add_comment">
            Try out different ad text to see what brings in the most customers,
            and learn how to enhance your ads using features like ad extensions.
            If you run into any problems with your ads, find out how to tell if
            they're running and how to resolve approval issues.
          </q-step>

          <template v-slot:navigation>
            <q-stepper-navigation>
              <q-btn
                color="primary"
                :label="step === 4 ? 'Finish' : 'Continue'"
              />
              <q-btn
                v-if="step > 1"
                flat
                color="primary"
                label="Back"
                class="q-ml-sm"
              />
            </q-stepper-navigation>
          </template>
        </q-stepper>
      </q-card-section>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { initAuth } from "@/api/auth"
import { DOCS_URL, NNS_HELP } from "@/api/constants/docs"
import { HttpAgent } from "@dfinity/agent"
import { GovernanceCanister } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import { SnsWrapper, initSnsWrapper } from "@dfinity/sns"
import { onMounted, ref } from "vue"

const help = ref(false)
const addNNSVisible = ref(false)
const filter = ref("") //搜索框
const step = ref(1)
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
  // DOCS NNS Help
  window.open(DOCS_URL + NNS_HELP)
}
</script>

<style lang="scss">
.nns-container {
  .head-icon {
    width: 32px !important;
  }
}
</style>

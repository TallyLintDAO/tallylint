<template>
  <div class="settings-container">
    <div class="row">
      <div class="col-md-12 col-lg-8">
        <q-card class="my-card">
          <q-card-section>
            <div class="text-h6">Portfolio settings</div>
          </q-card-section>

          <q-card-section class="q-pt-none">
            <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
              <q-list>
                <q-item>
                  <q-item-section>
                    <q-item-label> Base currency </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="userConfig.base_currency"
                        filled
                        use-input
                        input-debounce="0"
                        emit-value
                        option-label="code"
                        option-value="code"
                        :options="currencies"
                        label="Select Base Currency *"
                        :rules="[
                          (val) => !!val || 'Please select base currency',
                        ]"
                        hint="Now only have data on the latest currency exchange rates, there is a bias for historical transaction records"
                        @filter="filterCurrencies"
                      >
                        <template v-slot:no-option>
                          <q-item>
                            <q-item-section class="text-grey">
                              No results
                            </q-item-section>
                          </q-item>
                        </template>
                        <template v-slot:option="scope">
                          <q-item v-bind="scope.itemProps">
                            <q-item-section>
                              <q-item-label>{{ scope.opt.code }}</q-item-label>
                              <q-item-label caption>{{
                                scope.opt.name
                              }}</q-item-label>
                            </q-item-section>
                          </q-item>
                        </template>
                      </q-select>
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item>
                  <q-item-section>
                    <q-item-label> time_zone for reports </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="userConfig.time_zone"
                        filled
                        use-input
                        input-debounce="0"
                        :options="time_zoneList"
                        label="Select time_zone *"
                        :rules="[(val) => !!val || 'Please select time_zone']"
                        @filter="filtertime_zone"
                      >
                        <template v-slot:no-option>
                          <q-item>
                            <q-item-section class="text-grey">
                              No results
                            </q-item-section>
                          </q-item>
                        </template>
                      </q-select>
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item>
                  <q-item-section>
                    <q-item-label> Cost basis method </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="userConfig.tax_method"
                        filled
                        :options="taxMethodOption"
                        label="Select cost basis method *"
                        :rules="[
                          (val) => !!val || 'Please select cost basis method',
                        ]"
                      ></q-select>
                    </q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>

              <div class="q-gutter-sm justify-end flex">
                <q-btn flat label="Cancel" v-close-popup="true" />
                <q-btn
                  :loading="loading"
                  label="Save"
                  type="submit"
                  color="primary"
                />
              </div>
            </q-form>
          </q-card-section>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getUserConfig, setUserConfig } from "@/api/user"
import type { UserConfig } from "@/types/user"
import baseCurrencies, { setCurrencyCode } from "@/utils/currencies"
import { showMessageSuccess } from "@/utils/message"
import { setStorage } from "@/utils/storage"
import moment from "moment-timezone"
import type { QForm } from "quasar"
import { onMounted, ref } from "vue"

const form = ref<QForm | null>(null)
const loading = ref(false)

const userConfig = ref<UserConfig>({
  base_currency: "USD",
  time_zone: moment.tz.guess(),
  tax_method: "FIFO",
})
const currencies = ref(baseCurrencies)
const time_zoneList = ref(moment.tz.names())
// const taxMethodOption = ["FIFO"]
const taxMethodOption = ["FIFO", "LIFO", "HIFO"]

onMounted(() => {
  initUserConfig()
})

const initUserConfig = async () => {
  const res = await getUserConfig()
  if (res) {
    userConfig.value = res
  }
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await form.value?.validate()
  try {
    if (validationSuccess) {
      //TODO 如果改了tax_method 应该重新加载一下transaction的缓存
      await setUserConfig(userConfig.value).then((res) => {
        console.log("setConfig", res)
        setStorage("USER_CONFIG", userConfig.value)
        setCurrencyCode(userConfig.value.base_currency)
        showMessageSuccess("Save settings success")
      })
    }
  } catch (error) {
  } finally {
    loading.value = false
  }
}

const filterCurrencies = (val, update) => {
  update(() => {
    const needle = val.toLowerCase()
    currencies.value = baseCurrencies.filter(({ code }) =>
      code.toLowerCase().includes(needle),
    )
  })
}
const filtertime_zone = (val, update) => {
  update(() => {
    const needle = val.toLowerCase()
    time_zoneList.value = moment.tz
      .names()
      .filter((v) => v.toLowerCase().includes(needle))
  })
}
</script>

<style lang="scss">
.settings-container {
}
</style>

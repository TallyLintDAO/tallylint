<template>
  <div class="predict-container q-pt-lg">
    <div class="container">
      <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
        <q-select
          filled
          v-model="predict.model"
          :options="models"
          label="Predict Model Version"
        />
        <q-select
          filled
          v-model="predict.model"
          :options="models"
          label="Predict Model Parameters"
        />
        <q-input
          filled
          v-model="predict.model"
          label="Wallet Address *"
          hint="Input Principal ID"
          lazy-rules
        />
        <q-input
          filled
          disable
          label="Wallet Account ID"
          v-model="predict.model"
        />
        <q-input
          filled
          v-model="predict.model"
          label="Wallet Name *"
          hint="Identify your wallet quickly"
          lazy-rules
          :rules="[
                (val: string) => (val && val.length > 0) || 'Please input wallet name',
              ]"
        />
        <div class="q-gutter-sm justify-end flex">
          <q-btn flat label="Cancel" v-close-popup="true" />
          <q-btn
            :loading="loading"
            label="Submit"
            type="submit"
            color="primary"
          />
        </div>
      </q-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { QForm } from "quasar"
import { ref } from "vue"
const form = ref<QForm | null>(null)
const loading = ref(false)
const predict = ref({ model: "LSTM" })
const models = ["LSTM"]

const onSubmit = async () => {
  loading.value = true

  loading.value = false
}
</script>

<style lang="scss" scoped></style>

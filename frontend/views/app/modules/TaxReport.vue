<template>
  <div class="taxreport-container">
    <div class="text-h4 row q-gutter-md items-center">
      <span> TaxReport For </span>
      <q-select
        outlined
        v-model="selectedYear"
        :options="dateOptions"
        style="max-width: 200px; font-size: 2.125rem"
      />
    </div>
    <div class="text-subtitle1 q-mb-md">
      {{ `Jan 1, ${selectedYear} to Dec 31, ${selectedYear}` }}
    </div>
    <div class="row">
      <div class="col-8 q-pa-md" style="padding-left: 0">
        <q-card class="my-card q-mb-md">
          <q-card-section class="bg-grey-1">
            <div class="text-h6">Summary</div>
          </q-card-section>
          <q-card-section>
            <div>
              TaxLint needs your full transaction history (fiat → crypto →
              crypto → fiat) in order to calculate your tax reports correctly.
              The transactions used in this report are summarized below.
              <br />
              <div class="q-gutter-sm">
                <q-badge color="blue"> 9 transactions </q-badge>
                <q-badge color="grey-6"> 9 despoit </q-badge>
                <q-badge color="grey-6"> 9 transfers </q-badge>
                <q-badge color="grey-6"> 9 withdraw </q-badge>
                <q-badge color="grey-6"> 9 trades </q-badge>
              </div>
            </div>
          </q-card-section>
          <q-separator />
          <q-list separator>
            <q-item clickable v-ripple>
              <q-item-section>Capital gains / P&L</q-item-section>
              <q-item-section side>$ 10</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                Other gains (futures, derivatives etc)
              </q-item-section>
              <q-item-section side>$ 10</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Income</q-item-section>
              <q-item-section side>$ 10</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Costs & expenses</q-item-label>
                <q-icon name="warning" />
              </q-item-section>
              <q-item-section side>$ 10</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Gifts, donations & lost coins</q-item-label>
              </q-item-section>
              <q-item-section side>$ 10</q-item-section>
            </q-item>
          </q-list>
          <q-separator />
          <q-card-section>
            Note: This is just an indication of your taxable gains. Download a
            Tax Report to see your short/long-term proceeds, cost-basis,
            disposals and detailed calculations that you can use in your
            official tax returns.
          </q-card-section>
        </q-card>
        <q-card>
          <q-card-section>
            <q-banner rounded class="bg-yellow-2">
              <template v-slot:avatar>
                <q-icon name="warning" />
              </template>
              You must have a tax plan for this year to download reports. View
              plans
            </q-banner>
            <br />
            <q-btn
              color="primary"
              icon="file_download"
              label="Export CSV"
              @click="exportToCSV"
            />
          </q-card-section>
        </q-card>
      </div>
      <div class="col-4 q-pa-md">
        <q-card class="my-card">
          <q-card-section>
            <div class="text-h6">Settings</div>
            <div class="text-caption text-grey">
              These settings are used to calculate your gains. To change any of
              these click here
            </div>
          </q-card-section>
          <q-separator />
          <q-list separator>
            <q-item clickable v-ripple>
              <q-item-section overline>Home Country</q-item-section>
              <q-item-section side>US</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Base Currency</q-item-section>
              <q-item-section side>USD</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Cost basis method</q-item-section>
              <q-item-section side>FIFO</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Cost tracking method</q-item-section>
              <q-item-section side>Universal</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>
                Gains on crypto → crypto trades?
              </q-item-section>
              <q-item-section side>Yes</q-item-section>
            </q-item>
          </q-list>
        </q-card>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { computed, ref } from "vue"

// 获取当前年份
const currentYear = new Date().getFullYear()
const selectedYear = ref<number>(currentYear)

// 初始化日期选项数组，包含当前年份以及前三年的年份
const dateOptions: string[] = [currentYear.toString()]
for (let i = 1; currentYear - i >= 2021; i++) {
  dateOptions.push((currentYear - i).toString())
}

const exportToCSV = () => {}
</script>

<style lang="scss" scoped>
.taxreport-container {
}
</style>

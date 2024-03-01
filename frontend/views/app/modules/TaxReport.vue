<template>
  <div class="taxreport-container">
    <div class="text-h4 q-mb-md">TaxReport For 2024</div>
    <div class="row">
      <div>
        <q-card class="my-card">
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
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>
                  Other gains (futures, derivatives etc)
                </q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label overline>Income</q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Costs & expenses</q-item-label>
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Gifts, donations & lost coins</q-item-label>
              </q-item-section>
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
      </div>

      <div>Set</div>
    </div>
  </div>
</template>
<script setup lang="ts"></script>

<style lang="scss" scoped>
.taxreport-container {
}
</style>

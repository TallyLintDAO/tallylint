<template>
  <div class="container footer-container q-mt-xl">
    <div class="header">
      <div class="text-h4">TaxLint</div>
    </div>
    <div class="row justify-between items-center q-mt-sm">
      <div class="footer-left col-auto">
        <nav class="footer-nav row items-center q-gutter-md">
          <router-link to="/" class="footer-link">Home</router-link>
          <router-link to="/values" class="footer-link">Values</router-link>
        </nav>
      </div>
      <div class="footer-right col-auto">
        <a :href="DOCS_URL" class="footer-link" target="_Blank">Gitbook</a>
      </div>
    </div>
    <div class="footer-bottom row justify-between items-center q-mt-md">
      <div class="footer-bottom-left col-auto">
        <p>© TaxLint 2023-2024. All Rights Reserved.</p>
      </div>
      <div class="footer-bottom-right col-auto row items-center">
        <a href="https://discord.gg/2q72rBYB5C" target="_Blank">
          <q-icon class="cursor-pointer" size="md" name="discord" />
        </a>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { DOCS_URL } from "@/api/constants/docs"
</script>

<style lang="scss" scoped>
.footer-container {
  width: 100%;
}

.footer-link {
  color: rgb(107 114 128);
  text-decoration: none;
}

.footer-bottom {
  width: 100%;
  border-top: 1px solid #e0e0e0;
  padding-top: 1rem;
  font-size: 0.875rem;
  color: #666;
}

.footer-bottom-right .footer-link {
  color: #666;
  text-decoration: none;
}

.footer-link:hover {
  text-decoration: underline;
}
</style>

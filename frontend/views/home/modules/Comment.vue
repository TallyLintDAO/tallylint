<template>
  <div class="comment-container bg-grey-2 q-py-xl">
    <div class="container">
      <div class="row items-start q-col-gutter-md">
        <div class="col-3" v-for="comment in comments">
          <q-card>
            <q-card-section>
              <div class="row items-center">
                <q-icon name="star" color="positive" class="green-star" />
                <div class="text-subtitle1">
                  {{ comment.from }}
                </div>
              </div>
              <div class="text-weight-medium text-grey-9">
                {{ comment.description }}
              </div>
            </q-card-section>
            <q-card-section class="row justify-between">
              <q-icon name="thumb_up" />
              <div class="q-ml-sm">{{ comment.author }}</div>
            </q-card-section>
          </q-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
const comments = ref([
  {
    author: "test",
    from: "Develop Forum",
    description: `"I switched to Koinly last month and really loving it so far. Much better than cointracking. Good job!"`,
  },
  {
    author: "test",
    from: "develop forum",
    description: `"I switched to Koinly last month and really loving it so far. Much better than cointracking. Good job!"`,
  },
  {
    author: "test",
    from: "develop forum",
    description: `"I switched to Koinly last month and really loving it so far. Much better than cointracking. Good job!"`,
  },
  {
    author: "test",
    from: "develop forum",
    description: `"I switched to Koinly last month and really loving it so far. Much better than cointracking. Good job!"`,
  },
])
</script>

<style lang="scss" scoped>
.comment-container {
  padding: 150px 0px;
}
</style>

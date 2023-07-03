<template>
  <q-scroll-area class="fit">
    <div v-if="!cipherStore.getFilteredLength" style="text-align: center; padding: 50px 0">
      <div>
        <q-icon name="las la-inbox" class="text-grey-8" size="5em" />
      </div>
      <div>
        <p class="text-caption text-grey-6">暂无数据</p>
      </div>
    </div>
    <q-list v-else>
      <q-item
        v-for="cipherItem in cipherStore.filteredCiphers"
        :key="cipherItem.nid"
        clickable
        v-ripple
        :to="{ name: 'CipherDetail', params: { cid: cipherItem.nid } }"
        active-class="active-item"
        class="q-mx-sm">
        <q-item-section avatar>
          <q-icon name="las la-globe" size="lg" />
        </q-item-section>
        <q-item-section>
          <q-item-label lines="1" class="text-subtitle1">{{ cipherItem.website }}</q-item-label>
          <q-item-label lines="1" caption>{{ cipherItem.username }}</q-item-label>
        </q-item-section>
      </q-item>

      <!-- 占位 -->
      <div style="height: 50px"></div>
    </q-list>
  </q-scroll-area>
</template>

<script setup lang="ts">
// import { ref } from "vue"
import { useCipherStore } from "stores/cipher"

const cipherStore = useCipherStore()
// let link = ref("inbox")
</script>

<style scoped lang="scss">
.q-item {
  color: $grey-8;
  border-top-right-radius: 16px;
  border-bottom-right-radius: 16px;
}

.active-item {
  color: $teal-8;
  background-color: $teal-1;

  &::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background-color: $teal-6;
  }
}
</style>

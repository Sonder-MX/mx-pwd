<template>
  <q-toolbar>
    <q-btn flat round dense icon="las la-bars" @click="showLeftDrawer()" />
    <q-toolbar-title>MX-PWD</q-toolbar-title>
    <q-space></q-space>
    <div>
      <q-input dense bg-color="teal-1" label="登录信息搜索" v-model="cipherStore.searchKey">
        <template v-slot:prepend>
          <span class="q-pl-sm"></span>
        </template>
        <template v-slot:append>
          <q-icon
            v-if="cipherStore.searchKey"
            name="las la-times-circle"
            class="cursor-pointer"
            @click="cipherStore.searchKey = ''" />
          <q-icon v-else name="las la-search" />
          <span class="q-pl-sm"></span>
        </template>
      </q-input>
    </div>
  </q-toolbar>
</template>

<script setup lang="ts">
import { useCipherStore } from "stores/cipher"

defineProps({
  showLeftDrawer: {
    type: Function,
    required: true,
  },
})

const cipherStore = useCipherStore()
</script>

<style scoped lang="scss"></style>

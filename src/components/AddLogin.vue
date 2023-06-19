<template>
  <div class="absolute-bottom bg-grey-2 text-black shadow-up-4">
    <q-toolbar class="flex-center">
      <q-btn
        rounded
        icon="las la-plus-circle"
        label="添加登录信息"
        color="light-blue-6"
        class="q-px-xl"
        @click="addLoginPrompt" />
    </q-toolbar>
  </div>
</template>

<script setup lang="ts">
import { useQuasar } from "quasar"
import CustomDialog from "components/CustomDialog.vue"

const $q = useQuasar()

const addLoginPrompt = () => {
  $q.dialog({
    component: CustomDialog,
    componentProps: {
      title: "添加登录信息",
      content: "添加登录信息",
      ok: "添加",
      cancel: "取消",
    },
  })
}
</script>

<style scoped></style>

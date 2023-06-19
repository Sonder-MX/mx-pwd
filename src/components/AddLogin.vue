<template>
  <div class="absolute-bottom add-btn-box">
    <q-btn
      rounded
      icon="las la-plus-circle"
      label="添加登录信息"
      color="light-blue-6"
      class="q-px-xl"
      @click="addLoginPrompt" />
  </div>
</template>

<script setup lang="ts">
import { useQuasar } from "quasar"
import CustomDialog from "components/CustomDialog.vue"

const $q = useQuasar()

const addLoginPrompt = () => {
  $q.dialog({
    component: CustomDialog,
    componentProps: {},
  })
}
</script>

<style scoped>
.add-btn-box {
  width: 100%;
  height: 60px;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

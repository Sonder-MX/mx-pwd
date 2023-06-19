<template>
  <div>
    <h4>{{ cipherInfo }}</h4>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router"
import { watch, reactive } from "vue"
import { CipherType } from "stores/cipher"
import { invoke } from "@tauri-apps/api"

const route = useRoute()
let cipherInfo = reactive<CipherType>({
  nid: "",
  website: "",
  username: "",
  password: "",
  remark: "",
  created: "",
  updated: "",
})

// get cipher detail
const getCipherDetail = (nid: string) => {
  invoke("get_cipher_detail", { dnid: nid }).then((res) => {
    const resp = res as CipherType
    cipherInfo.nid = resp.nid
    cipherInfo.website = resp.website
    cipherInfo.username = resp.username
    cipherInfo.password = resp.password
    cipherInfo.remark = resp.remark
    cipherInfo.created = resp.created
    cipherInfo.updated = resp.updated
  }).catch((err) => {
    console.log(err)
  })
}

watch(
  () => route.params.cid,
  (cid) => {
    getCipherDetail(cid as string)
  },
  { immediate: true } // 在组件挂载时立即执行
)
</script>

<style scoped></style>

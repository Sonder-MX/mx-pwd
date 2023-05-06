<template>
  <!-- <router-view /> -->
  <div>
    <button @click="getCipherList">get cipher list</button>
  </div>
  <div>
    <button @click="getCipherDetail">get cipher detail</button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api"

const getCipherList = () => {
  invoke("get_cipher_list")
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.log(err)
    })
}

const getCipherDetail = () => {
  invoke("get_cipher_detail", { dnid: "1" })
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.log(err)
    })
}
</script>

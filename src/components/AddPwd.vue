<template>
  <!-- <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div> -->
  <div>站点: <input type="text" v-model="pwdInfo.station" /></div>
  <div>用户名: <input type="text" v-model="pwdInfo.username" /></div>
  <div>密码: <input type="password" v-model="pwdInfo.password" /></div>
  <div>描述: <input type="text" v-model="pwdInfo.desc" /></div>
  <div>
    <button type="button" @click="addPwd">提交</button>
  </div>
</template>

<script setup>
import { reactive } from "vue"
import { invoke } from "@tauri-apps/api/tauri"

// const greetMsg = ref("")
// const name = ref("")

const pwdInfo = reactive({
  station: "",
  username: "",
  password: "",
  desc: null,
})

async function addPwd() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   greetMsg.value = await invoke("greet", { name: name.value })
  await invoke("add_pwd", {
    station: pwdInfo.station,
    username: pwdInfo.username,
    password: pwdInfo.password,
    desc: pwdInfo.desc,
  })
}
</script>

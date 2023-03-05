<template>
  <div>站点: <input type="text" v-model="pwdInfo.station" /></div>
  <div>用户名: <input type="text" v-model="pwdInfo.username" /></div>
  <div>密码: <input type="password" v-model="pwdInfo.pwd" /></div>
  <div>描述: <input type="text" v-model="pwdInfo.desc" /></div>
  <div>
    <button type="button" @click="addCipher">提交</button>
  </div>

  <div>
    <button type="button" @click="getAll">查看全部</button>
  </div>
</template>

<script setup>
import { reactive } from "vue"
import { invoke } from "@tauri-apps/api/tauri"

const pwdInfo = reactive({
  station: "",
  username: "",
  pwd: "",
  desc: "",
})

async function getAll() {
  await invoke("get_all")
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.log(err)
    })
}

async function addCipher() {
  await invoke("add_cipher", {
    station: pwdInfo.station,
    username: pwdInfo.username,
    pwd: pwdInfo.pwd,
    desc: pwdInfo.desc,
  })
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.log(err)
    })
}
</script>

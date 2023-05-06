<template>
  <!-- <router-view /> -->
  <div>
    <button @click="getCipherList">get cipher list</button>
  </div>
  <div>
    <button @click="getCipherDetail">get cipher detail</button>
  </div>
  <div>
    <label>
      website:
      <input type="text" v-model="cipherInfo.website" />
    </label>
    <br />
    <label>
      username:
      <input type="text" v-model="cipherInfo.username" />
    </label>
    <br />
    <label>
      password:
      <input type="text" v-model="cipherInfo.password" />
    </label>
    <br />
    <label>
      remark:
      <input type="text" v-model="cipherInfo.remark" />
    </label>
    <div>
      <button @click="submitData">submit</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from "vue"
import { invoke } from "@tauri-apps/api"

let cipherInfo = reactive({
  website: "",
  username: "",
  password: "",
  remark: "",
})

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

const submitData = () => {
  // // 验证字段 website username password 是否为空，为空则提示
  // if (cipherInfo.website === "" || cipherInfo.username === "" || cipherInfo.password === "") {
  //   console.log("website username password 不能为空")
  //   return
  // }
  invoke("add_cipher", {
    website: cipherInfo.website,
    username: cipherInfo.username,
    password: cipherInfo.password,
    remark: cipherInfo.remark,
  })
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.log(err)
    })
}
</script>

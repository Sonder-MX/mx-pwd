<template>
  <div class="top-bar">
    <div class="operation-box">
      <button class="btn-success" type="button" @click="addPwd">新建</button>
    </div>
    <div class="search-box">
      <label>
        <input type="text" placeholder="站点检索" v-model="search" />
      </label>
      <button type="button" @click="searchPwd">搜索</button>
    </div>
  </div>
  <ModalPage v-if="showModal" :changeShowModal="changeShowModal" title="新建" />
</template>

<script setup>
import { ref } from "vue"
import ModalPage from "./ModalPage.vue"

const search = ref("")
const showModal = ref(false)

function changeShowModal() {
  showModal.value = !showModal.value
}

function addPwd() {
  changeShowModal()
}

function searchPwd() {}
</script>

<style scoped>
.top-bar {
  margin-top: 46px;
  overflow: hidden;
}

.operation-box {
  margin-left: 5px;
  float: left;
}

.btn-success {
  width: 60px;
  height: 32px;
  background-color: #5cb85c;
  border-color: #4cae4c;
  color: #fff;
  border-radius: 5px;
  border: none;
  margin-bottom: 8px;
}

.btn-success:hover {
  background-color: #449d44;
  border-color: #398439;
}

.search-box {
  float: right;
}

.search-box input {
  width: 140px;
  height: 30px;
  font-size: 14px;
  border: 1px solid #585757;
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
  padding: 0 10px;
}

.search-box input:focus {
  outline: none;
  border: 1px solid #000;
}

.search-box button {
  width: 50px;
  height: 32px;
  border: 1px solid #585757;
  border-left: none;
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
  margin-right: 5px;
  background-color: #fff;
  cursor: pointer; /* 鼠标变成手指 */
}

.search-box button:hover {
  background-color: #585757;
  color: #fff;
}
</style>

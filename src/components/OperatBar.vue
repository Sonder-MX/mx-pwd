<template>
  <div class="top-bar">
    <div class="operation-box">
      <button class="btn-success" type="button" @click="addPwd">新建</button>
    </div>
    <div class="search-box">
      <label for="search-inp"> 搜索 </label>
      <input id="search-inp" type="text" placeholder="站点检索" v-model="search" />
    </div>
  </div>
  <ModalPage
    v-if="showModal"
    :changeShowModal="changeShowModal"
    title="新建"
    :getAll="props.getAll" />
</template>

<script setup>
import { ref, watch } from "vue"
import ModalPage from "./ModalPage.vue"

const props = defineProps({
  changeSearch: {
    type: Function,
    required: true,
  },
  getAll: {
    type: Function,
    required: true,
  },
})

let search = ref("")
let showModal = ref(false)

function changeShowModal() {
  showModal.value = !showModal.value
}

function addPwd() {
  changeShowModal()
}

watch(search, () => {
  props.changeSearch(search.value)
})
</script>

<style scoped>
.top-bar {
  margin-top: 46px;
  overflow: hidden;
}

.operation-box {
  margin-left: 10px;
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
  margin-right: 10px;
}

.search-box input {
  width: 140px;
  height: 30px;
  font-size: 14px;
  border: 1px solid #585757;
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
  padding: 0 10px;
}

.search-box input:focus {
  outline: none;
  border: 1px solid #000;
}

.search-box label {
  padding: 7px;
  font-size: smaller;
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
  background-color: #585757;
  color: #fff;
}
</style>

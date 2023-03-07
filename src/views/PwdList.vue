<template>
  <OperatBar />
  <div class="list-content">
    <table class="tab-pwd">
      <thead>
        <tr>
          <th>站点</th>
          <th>用户名</th>
          <th>密码</th>
          <th>描述</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody v-if="pList.length === 0">
        <tr>
          <td v-if="errMsg" colspan="5" style="color: red; text-align: center; padding: 30px 0">
            {{ errMsg }}
          </td>
          <td v-else colspan="5" style="text-align: center; padding: 30px 0">暂无数据！</td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr v-for="item in pList" :key="item.uid">
          <td>{{ item.station }}</td>
          <td>{{ item.username }}</td>
          <td>{{ item.password }}</td>
          <td>{{ item.desc }}</td>
          <td>
            <button type="button" class="btn btn-primary" @click="editPwd">编辑</button>&nbsp;
            <button type="button" class="btn btn-danger">删除</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <ModalPage v-show="showModal" :changeShowModal="changeShowModal" title="编辑" />
</template>

<script setup>
import OperatBar from "../components/OperatBar.vue"
import ModalPage from "../components/ModalPage.vue"

import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/tauri"

const pList = ref([])
const errMsg = ref("")
const showModal = ref(false)

function changeShowModal() {
  showModal.value = !showModal.value
}

function editPwd() {
  changeShowModal()
}

function getAll() {
  invoke("get_all")
    .then((res) => {
      pList.value = res
    })
    .catch(() => {
      errMsg.value = "获取数据失败！"
    })
}

onMounted(() => {
  getAll()
})
</script>

<style scoped>
.list-content {
  padding: 0 10px;
}

.tab-pwd {
  width: 100%;
  border-collapse: collapse;
  border: 1px solid #b3b2b2;
}

.tab-pwd th,
.tab-pwd td {
  border: 1px solid #b3b2b2;
  padding: 5px;
}

.tab-pwd th {
  background-color: #f2f2f2;
}

.tab-pwd td {
  text-align: center;
}

.tab-pwd tr:nth-child(odd) {
  background-color: #f2f2f2;
}

.tab-pwd tr:nth-child(even) {
  background-color: #fff;
}

.tab-pwd tr:hover {
  background-color: #e2e2e2;
}

.btn {
  padding: 3px 8px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

.btn-primary {
  font-size: smaller;
  background-color: #007bff;
  color: #fff;
}

.btn-danger {
  font-size: smaller;
  background-color: #dc3545;
  color: #fff;
}

.btn:hover {
  opacity: 0.8;
}

.btn:active {
  opacity: 0.6;
}
</style>

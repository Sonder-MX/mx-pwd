<template>
  <el-card>
    <template #header>
      <div class="card-header">
        <span>
          <el-icon><Promotion /></el-icon>
          {{ pwdInfo.station }}
        </span>
        <div>
          <el-button
            type="primary"
            icon="Edit"
            circle
            @click="dialogStore.changeVisible('编辑', pwdInfo)" />
          <el-popconfirm
            :hide-after="100"
            confirm-button-text="是"
            confirm-button-type="danger"
            cancel-button-text="否"
            cancel-button-type="info"
            icon="InfoFilled"
            title="是否删除？"
            @confirm="delete_pwd(pwdInfo.uid)">
            <template #reference>
              <el-button type="danger" icon="Delete" circle></el-button>
            </template>
          </el-popconfirm>
        </div>
      </div>
    </template>
    <div>
      <el-row :gutter="20" class="row-info">
        <el-col :span="8" class="col-label">
          <el-icon><Paperclip /></el-icon>&nbsp; 站点
        </el-col>
        <el-col :span="16" class="col-content">{{ pwdInfo.station }}</el-col>
      </el-row>
      <el-row :gutter="20" class="row-info">
        <el-col :span="8" class="col-label">
          <el-icon><User /></el-icon>&nbsp; 用户名
        </el-col>
        <el-col :span="16" class="col-content">{{ pwdInfo.username }}</el-col>
      </el-row>
      <el-row :gutter="20" class="row-info">
        <el-col :span="8" class="col-label">
          <el-icon><Lock /></el-icon>&nbsp; 密码
        </el-col>
        <el-col :span="16" class="col-content">{{ pwdInfo.password }}</el-col>
      </el-row>
      <el-row :gutter="20" class="row-info">
        <el-col :span="8" class="col-label">
          <el-icon><Document /></el-icon>&nbsp; 描述
        </el-col>
        <el-col :span="16" class="col-content">{{ pwdInfo.desc }}</el-col>
      </el-row>
    </div>
  </el-card>
</template>

<script setup>
import { ref, watch, onMounted } from "vue"
import { useRoute } from "vue-router"
import { invoke } from "@tauri-apps/api/tauri"
import { useDialogFormStore, successMsg, errorMsg } from "../stores/dialogForm"
import { useCipherListStore } from "../stores/cipherList"

const route = useRoute()
const pwdInfo = ref({})
const dialogStore = useDialogFormStore()

function get_pwd_detail(uuid) {
  invoke("pwd_detail", { uid: uuid }).then((res) => {
    pwdInfo.value = res
  })
}

function delete_pwd(uuid) {
  invoke("del_pwd", { uid: uuid })
    .then((res) => {
      if (res) {
        successMsg("删除成功")
        useCipherListStore().getCipherList()
      } else errorMsg("删除失败")
    })
    .catch(() => errorMsg("删除失败"))
}

watch(
  () => route.params.uid,
  (newVal) => get_pwd_detail(newVal)
)

onMounted(() => get_pwd_detail(route.params.uid))
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  user-select: none;
}

.card-header span {
  font-size: 24px;
}

.row-info {
  border: 1px solid #e1e1e3;
  font-size: 18px;
}

.col-label {
  display: flex;
  align-items: center;
  justify-content: left;
  user-select: none;
  padding: 8px 0;
  color: rgb(91, 91, 91);
  background-color: #f2f2f5;
}

.col-content {
  padding: 8px 0;
}
</style>

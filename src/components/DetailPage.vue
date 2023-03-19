<template>
  <el-card>
    <template #header>
      <div class="card-header">
        <span>
          <el-icon><Promotion /></el-icon>
          {{ pwdInfo.station }}
        </span>
        <div>
          <el-button type="primary" icon="Edit" circle />
          <el-button type="danger" icon="Delete" circle />
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
import { ref, watch } from "vue"
import { useRoute } from "vue-router"
import { invoke } from "@tauri-apps/api/tauri"

const route = useRoute()
const pwdInfo = ref({})

watch(
  () => route.params.uid,
  (newVal) => {
    invoke("pwd_detail", { uid: newVal }).then((res) => {
      pwdInfo.value = res
    })
    console.log(pwdInfo.value)
  }
)
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
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
  padding: 8px 0;
  color: rgb(91, 91, 91);
  background-color: #f2f2f5;
}

.col-content {
  padding: 8px 0;
}
</style>

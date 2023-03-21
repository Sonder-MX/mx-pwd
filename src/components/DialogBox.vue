<template>
  <el-dialog
    v-model="dialogStore.dialogFormVisible"
    :title="dialogStore.dialogFormTitle"
    :close-on-click-modal="false">
    <el-form
      label-position="right"
      label-width="70px"
      :model="formModel"
      ref="formRuleRef"
      :rules="formRules">
      <el-form-item label="站点" prop="station">
        <el-input placeholder="https://exp.com" v-model="formModel.station"></el-input>
      </el-form-item>
      <el-form-item label="用户名" prop="username">
        <el-input placeholder="请输入用户名" v-model="formModel.username"></el-input>
      </el-form-item>
      <el-form-item label="密码" prop="pwd">
        <el-input
          placeholder="请输入密码"
          type="password"
          v-model="formModel.pwd"
          show-password></el-input>
      </el-form-item>
      <el-form-item label="描述">
        <el-input placeholder="请输入描述" v-model="formModel.desc"></el-input>
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button type="info" plain @click="dialogStore.dialogFormVisible = false">
          取消
        </el-button>
        <el-button type="success" plain @click="dialogStore.submitFormData(formRuleRef)">
          保存
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref } from "vue"
import { useDialogFormStore } from "../stores/dialogForm"

const formRuleRef = ref()
const dialogStore = useDialogFormStore()
const formRules = dialogStore.getFormRules
const formModel = dialogStore.dialogFormModel
</script>

<style scoped></style>

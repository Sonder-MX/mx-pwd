<template>
  <div>
    <el-row>
      <el-col :span="12">
        <el-input v-model="inpSearch" placeholder="搜索登录信息" :prefix-icon="Search" clearable />
      </el-col>
      <el-col :span="12" class="my-dropdown-box">
        <el-dropdown>
          <span class="el-dropdown-link">
            Other
            <el-icon class="el-icon--right"><ArrowDownBold /></el-icon>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>
                <el-icon><Postcard /></el-icon> 关于
              </el-dropdown-item>
              <el-dropdown-item>
                <el-icon><Star /></el-icon> 支持
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref } from "vue"
import { Search } from "@element-plus/icons-vue"

let inpSearch = ref("")
</script>

<style scoped>
.my-dropdown-box {
  display: flex;
  justify-content: flex-end;
}

.el-dropdown-link {
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  user-select: none;
}
</style>

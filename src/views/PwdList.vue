<template>
  <el-container style="height: 97vh">
    <el-header>
      <h1 style="margin-top: 0; font-style: italic">MX-PWD</h1>
    </el-header>
    <el-divider />
    <el-main>
      <el-menu :default-active="cipherStore.activeId" @select="handleSelect">
        <el-menu-item v-for="pwd in cipherStore.filteredCipherList" :key="pwd.uid" :index="pwd.uid">
          <el-icon><Link /></el-icon>
          {{ pwd.station }}-{{ pwd.username }}
        </el-menu-item>
      </el-menu>
    </el-main>
    <el-divider />
    <el-footer>
      <el-button type="primary" style="width: 100%" @click="dialogStore.changeVisible('新建')">
        <el-icon><DocumentAdd /></el-icon>&nbsp; 新建信息
      </el-button>
    </el-footer>
  </el-container>

  <DialogBox />
</template>

<script setup>
import { useRouter } from "vue-router"
import { useCipherListStore } from "../stores/cipherList"
import { useDialogFormStore } from "../stores/dialogForm"
import DialogBox from "../components/DialogBox.vue"

const router = useRouter()
const cipherStore = useCipherListStore()
const dialogStore = useDialogFormStore()

cipherStore.getCipherList()

const handleSelect = (key) => {
  cipherStore.activeId = key
  router.push({ name: "Detail", params: { uid: key } })
}
</script>

<style scoped>
* {
  user-select: none;
}

.el-header {
  height: 0;
  padding-bottom: 8px;
}

.el-menu-item {
  font-size: 16px;
}

.el-menu {
  --el-menu-level: none !important;
}

.el-menu-item {
  height: 48px;
}

.el-footer {
  height: 40px;
}
</style>

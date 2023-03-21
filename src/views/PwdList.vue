<template>
  <el-container style="height: 97vh">
    <el-header>
      <h1 style="margin-top: 0; font-style: italic">MX-PWD</h1>
    </el-header>
    <el-divider />
    <el-main>
      <el-menu :default-active="defaultActive" @select="handleSelect">
        <el-menu-item v-for="pwd in pwdList" :key="pwd.uid" :index="pwd.uid">
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
import { invoke } from "@tauri-apps/api/tauri"
import { ref, onMounted } from "vue"
import { useRouter } from "vue-router"
import { useDialogFormStore } from "../stores/dialogForm"
import DialogBox from "../components/DialogBox.vue"

const router = useRouter()
const pwdList = ref([])
const defaultActive = ref("")
const dialogStore = useDialogFormStore()

onMounted(() => {
  invoke("pwd_list").then((res) => {
    pwdList.value = res
    defaultActive.value = res[0].uid
    router.push({ name: "Detail", params: { uid: res[0].uid } })
  })
})

const handleSelect = (key) => {
  defaultActive.value = key
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

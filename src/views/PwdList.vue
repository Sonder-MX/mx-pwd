<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <h1 style="margin-top: 0; font-style: italic">MX-PWD</h1>
      </el-header>
      <el-main>
        <el-menu :default-active="defaultActive" @select="handleSelect">
          <el-menu-item v-for="pwd in pwdList" :key="pwd.uid" :index="pwd.uid">
            <el-icon><Promotion /></el-icon>
            {{ pwd.station }}-{{ pwd.username }}
          </el-menu-item>
        </el-menu>
      </el-main>
    </el-container>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri"
import { ref, onMounted } from "vue"
import { useRouter } from "vue-router"

const router = useRouter()
const pwdList = ref([])
const defaultActive = ref("")

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
</style>

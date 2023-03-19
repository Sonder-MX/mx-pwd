<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="28vw">
        <PwdList></PwdList>
      </el-aside>
      <el-container>
        <el-header>
          <HeaderMx></HeaderMx>
        </el-header>
        <el-main>
          <router-view></router-view>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>
import PwdList from "./views/PwdList.vue"
import HeaderMx from "./components/HeaderMx.vue"
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
}
</style>

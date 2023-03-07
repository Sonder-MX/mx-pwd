<template>
  <HeaderMx />
  <div class="container">
    <PwdList />
  </div>
</template>

<script setup>
import HeaderMx from "./components/HeaderMx.vue"
import PwdList from "./views/PwdList.vue"
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

.container {
  margin-top: 40px;
}
</style>

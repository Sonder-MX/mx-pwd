<template>
  <HeaderMx />
  <div class="container">
    <PwdList />
  </div>
  <div>
    <TestApi />
  </div>
</template>

<script setup>
import HeaderMx from "./components/HeaderMx.vue"
import PwdList from "./views/PwdList.vue"
import TestApi from "./views/TestApi.vue"
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

.container {
  margin-top: 40px;
}

.btn {
  padding: 3px 8px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

.btn:hover {
  opacity: 0.8;
}

.btn:active {
  opacity: 0.6;
}
</style>

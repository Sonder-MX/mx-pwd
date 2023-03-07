<template>
  <ul class="ul-bar">
    <li class="logo">
      <a href="#">
        <h1>MX-PWD</h1>
      </a>
    </li>
    <li class="search-box">
      <!-- <label>
        <input type="text" v-model="search" />
      </label>
      <button type="button" @click="searchPwd">搜索</button> -->
    </li>
  </ul>
</template>

<script setup>
import { ref } from "vue"

const search = ref("")

function searchPwd() {
  if (search.value.trim()) {
    console.log(search.value)
  }
}
</script>

<style scoped>
h1 {
  font-style: italic;
}

h1:hover {
  text-shadow: 0 0 2px #242424;
}

.ul-bar {
  list-style-type: none;
  overflow: hidden;
  position: fixed;
  top: 0;
  width: 100%;
  background-color: #f5f0f0;
}

.ul-bar a {
  text-decoration: none;
  color: #000;
}

.logo {
  float: left;
  padding-left: 8px;
}

.search-box {
  float: right;
  margin-top: 8px;
}

.search-box input {
  width: 140px;
  height: 30px;
  font-size: 14px;
  border: 1px solid #585757;
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
  padding: 0 10px;
}

.search-box input:focus {
  outline: none;
  border: 1px solid #000;
}

.search-box button {
  width: 50px;
  height: 32px;
  border: 1px solid #585757;
  border-left: none;
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
  margin-right: 5px;
  background-color: #fff;
  cursor: pointer; /* 鼠标变成手指 */
}

.search-box button:hover {
  background-color: #585757;
  color: #fff;
}
</style>

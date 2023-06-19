<template>
  <q-layout view="hHh LpR lFf">
    <q-header elevated class="bg-grey-2 text-black">
      <nav-bar :showLeftDrawer="isShowLeftDrawer" />
    </q-header>

    <q-drawer v-model="leftDrawer" show-if-above :breakpoint="500" bordered>
      <cipher-list />
      <add-login />
    </q-drawer>

    <q-page-container>
      <q-page padding>
        <router-view></router-view>
      </q-page>
    </q-page-container>

    <q-footer elevated class="bg-grey-2 text-black">
      <foot-bar />
    </q-footer>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue"
import { useRouter } from "vue-router"
import { useCipherStore } from "stores/cipher"
import NavBar from "components/NavBar.vue"
import CipherSort from "components/CipherSort.vue"
import CipherList from "components/CipherList.vue"
import AddLogin from "components/AddLogin.vue"
import FootBar from "components/FootBar.vue"

const cipherStore = useCipherStore()
const router = useRouter()
let leftDrawer = ref(true)

const isShowLeftDrawer = () => {
  leftDrawer.value = !leftDrawer.value
}

onMounted(() => {
  if (cipherStore.ciphers.length === 0) {
    router.push({ name: "Welcome" })
  } else {
    router.push({ name: "CipherDetail", params: { cid: cipherStore.ciphers[0].nid } })
  }
})
</script>

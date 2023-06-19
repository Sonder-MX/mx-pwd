import { boot } from "quasar/wrappers"
import { useCipherStore } from "src/stores/cipher"

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(async (/* { app, router, ... } */) => {
  const cipherStore = useCipherStore()
  await cipherStore.fetchCiphers()
})

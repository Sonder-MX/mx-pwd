import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api"

export interface CipherType {
  nid?: string
  website: string
  username: string
  password: string
  remark?: string
  created?: string
  updated?: string
}

export const useCipherStore = defineStore("cipher-store", {
  state: () => ({
    ciphers: [] as CipherType[],
    searchKey: "",
  }),

  getters: {
    filteredCiphers: (state) => {
      return state.ciphers.filter((cipher) => {
        return (
          cipher.website.includes(state.searchKey) ||
          cipher.username.includes(state.searchKey) ||
          cipher.remark?.includes(state.searchKey)
        )
      })
    },
  },

  actions: {
    async fetchCiphers() {
      await invoke("get_cipher_list").then((ciphers) => {
        this.ciphers = ciphers as CipherType[]
      })
    },
  },
})

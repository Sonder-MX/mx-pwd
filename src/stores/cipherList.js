import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api"

export const useCipherListStore = defineStore({
  id: "cipherList",
  state: () => ({
    cipherList: [],
    searchText: "",
    activeId: "",
  }),
  getters: {
    filteredCipherList() {
      return this.cipherList.filter((cipherArr) => {
        return (
          cipherArr.station.includes(this.searchText) ||
          cipherArr.username.includes(this.searchText)
        )
      })
    },
    listLen() {
      return this.filteredCipherList.length
    },
  },
  actions: {
    getCipherList() {
      invoke("pwd_list")
        .then((res) => {
          this.cipherList = res
          if (res.length) {
            this.activeId = res[0].uid
          }
        })
        .catch(() => {
          this.cipherList = []
        })
    },
  },
})

import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api"
import { ElMessage } from "element-plus"

export const successMsg = (msg) => {
  ElMessage({
    message: msg,
    type: "success",
    duration: 2000,
  })
}

export const errorMsg = (msg) => {
  ElMessage({
    message: msg,
    type: "error",
    duration: 2000,
  })
}

function addPwd(formData) {
  const { station, username, pwd, desc } = formData
  invoke("add_pwd", {
    station,
    username,
    pwd,
    desc,
  })
    .then((res) => {
      if (res) successMsg("添加成功")
      else errorMsg("添加失败")
    })
    .catch(() => errorMsg("添加失败"))
}

function updatePwd(formData) {
  const { uid, station, username, pwd, desc } = formData
  invoke("upt_pwd", {
    uid,
    station,
    username,
    pwd,
    desc,
  })
    .then((res) => {
      if (res) successMsg("更新成功")
      else errorMsg("更新失败")
    })
    .catch(() => errorMsg("更新失败"))
}

export const useDialogFormStore = defineStore({
  id: "dialogForm",
  state: () => ({
    dialogFormVisible: false,
    dialogFormTitle: "",
    dialogFormModel: {
      uid: "",
      station: "",
      username: "",
      pwd: "",
      desc: "",
    },
    dialogFormRules: {
      station: [{ required: true, message: "请输入站点", trigger: "blur" }],
      username: [{ required: true, message: "请输入用户名", trigger: "blur" }],
      pwd: [{ required: true, message: "请输入密码", trigger: "blur" }],
    },
  }),
  getters: {
    getFormRules() {
      return this.dialogFormRules
    },
  },
  actions: {
    blankForm() {
      this.dialogFormModel.uid = ""
      this.dialogFormModel.station = ""
      this.dialogFormModel.username = ""
      this.dialogFormModel.pwd = ""
      this.dialogFormModel.desc = ""
    },
    changeVisible(formTitle, formData = null) {
      this.dialogFormVisible = !this.dialogFormVisible
      this.dialogFormTitle = formTitle
      this.blankForm()
      if (formData) {
        this.dialogFormModel.uid = formData.uid
        this.dialogFormModel.station = formData.station
        this.dialogFormModel.username = formData.username
        this.dialogFormModel.pwd = formData.password
        this.dialogFormModel.desc = formData.desc
        console.log("执行")
      }
    },
    submitFormData(formEl) {
      if (!formEl) return
      formEl.validate((valid) => {
        if (valid) {
          this.dialogFormVisible = false
          if (this.dialogFormModel.uid) {
            updatePwd(this.dialogFormModel)
          } else {
            addPwd(this.dialogFormModel)
          }
          this.blankForm()
        }
      })
    },
  },
})

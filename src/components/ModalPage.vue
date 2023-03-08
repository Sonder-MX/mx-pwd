<template>
  <div class="modal-backdrop">
    <div class="modal">
      <div class="modal-header">
        <h3>{{ props.title }}</h3>
      </div>
      <div class="modal-body">
        <div class="form-inp">
          <label>
            站点
            <input type="text" placeholder="输入站点" v-model="formData.station" />
          </label>
        </div>
        <div class="form-inp">
          <label>
            账号
            <input type="text" placeholder="输入账号" v-model="formData.username" />
          </label>
        </div>
        <div class="form-inp">
          <label>
            密码
            <input type="password" placeholder="输入密码" v-model="formData.pwd" />
          </label>
        </div>
        <div class="form-inp">
          <label>
            描述
            <input type="text" placeholder="说明(可选)" v-model="formData.desc" />
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-close" @click="closeModal">关闭</button>
        <button type="button" class="btn btn-primary" @click="submit">确认</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/tauri"

const props = defineProps({
  changeShowModal: {
    type: Function,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
  editInfo: {
    type: Object,
    required: false,
  },
  getAll: {
    type: Function,
    required: true,
  },
})

const closeModal = () => {
  props.changeShowModal()
}

let formData = reactive({
  uid: "",
  station: "",
  username: "",
  pwd: "",
  desc: "",
})

onMounted(() => {
  if (props.editInfo) {
    formData.uid = props.editInfo.uid
    formData.station = props.editInfo.station
    formData.username = props.editInfo.username
    formData.pwd = props.editInfo.password
    formData.desc = props.editInfo.desc
  }
})

function submit() {
  if (formData.uid) {
    invoke("upt_cipher", { ...formData })
      .then(() => {
        closeModal()
      })
      .catch(() => {
        console.log("更新失败！")
      })
  } else {
    invoke("add_cipher", {
      station: formData.station,
      username: formData.username,
      pwd: formData.pwd,
      desc: formData.desc,
    })
      .then(() => {
        closeModal()
      })
      .catch(() => {
        console.log("添加失败！")
      })
  }
  props.getAll()
}
</script>

<style>
.modal-backdrop {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background-color: rgba(0, 0, 0, 0.3);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal {
  background-color: #fff;
  box-shadow: 2px 2px 20px 1px;
  overflow-x: auto;
  display: flex;
  flex-direction: column;
  border-radius: 16px;
  width: 80%;
}

.modal-header {
  border-bottom: 1px solid #eee;
  color: #313131;
  justify-content: space-between;
  padding: 15px;
  display: flex;
}

.modal-footer {
  border-top: 1px solid #eee;
  justify-content: flex-end;
  padding: 15px;
  display: flex;
}

.modal-body {
  position: relative;
  padding: 20px 10px;
}

.btn-primary {
  margin-left: 5px;
  background-color: #007bff;
  color: #fff;
}

.btn-close {
  background-color: #878383;
  color: #fff;
}

.form-inp {
  width: 100%;
  text-align: center;
  margin-bottom: 16px;
}

.form-inp input {
  width: 50%;
  border: 2px solid #484747;
  border-radius: 8px;
  padding: 6px;
  font-size: 14px;
  color: #313131;
}

.form-inp input:focus {
  outline: none;
  border: 2px solid #000;
}
</style>

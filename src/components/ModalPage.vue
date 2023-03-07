<template>
  <div class="modal-backdrop">
    <div class="modal">
      <div class="modal-header">
        <h3>{{ props.title }}</h3>
      </div>
      <div class="modal-body">
        <p>我是一个Modal的内容</p>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn-close" @click="closeModal">关闭</button>
        <button type="button" class="btn-confirm">确认</button>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  changeShowModal: {
    type: Function,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
})

const closeModal = () => {
  props.changeShowModal()
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
.btn-close,
.btn-confirm {
  border-radius: 8px;
  margin-left: 16px;
  width: 56px;
  height: 36px;
  border: none;
  cursor: pointer;
}
.btn-close {
  color: #313131;
  background-color: transparent;
}
.btn-confirm {
  color: #fff;
  background-color: #2d8cf0;
}
</style>

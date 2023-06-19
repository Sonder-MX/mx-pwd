<template>
  <q-dialog ref="dialogRef" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section>
        <div class="text-h6">添加登录信息</div>
      </q-card-section>

      <q-card-section>
        <q-input
          rounded
          outlined
          dense
          label="网站"
          v-model="loginInfo.website"
          :rules="[(val) => !!val || '* 必填项', (val) => val.length > 3 || '长度不能小于3']"
          lazy-rules>
          <template v-slot:prepend>
            <q-icon name="las la-blog" />
          </template>
          <template v-slot:append>
            <q-icon name="las la-times" v-if="loginInfo.website" @click="loginInfo.website = ''" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-section>
        <q-input
          rounded
          outlined
          dense
          label="账号或用户名"
          v-model="loginInfo.username"
          :rules="[(val) => !!val || '* 必填项', (val) => val.length > 3 || '长度不能小于3']"
          lazy-rules>
          <template v-slot:prepend>
            <q-icon name="las la-user-circle" />
          </template>
          <template v-slot:append>
            <q-icon
              name="las la-times"
              v-if="loginInfo.username"
              @click="loginInfo.username = ''" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-section>
        <q-input
          rounded
          outlined
          dense
          label="密码"
          :type="isShowPwd ? 'text' : 'password'"
          v-model="loginInfo.password"
          :rules="[(val) => !!val || '* 必填项', (val) => val.length > 3 || '长度不能小于3']"
          lazy-rules>
          <template v-slot:prepend>
            <q-icon name="las la-lock" />
          </template>
          <template v-slot:append>
            <q-icon
              name="las la-times"
              v-if="loginInfo.password"
              @click="loginInfo.password = ''" />
            <q-icon
              :name="isShowPwd ? 'las la-eye-slash' : 'las la-eye'"
              v-if="loginInfo.password"
              @click="isShowPwd = !isShowPwd" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-section>
        <q-input rounded outlined dense label="备注" v-model="loginInfo.remark">
          <template v-slot:prepend>
            <q-icon name="las la-bookmark" />
          </template>
          <template v-slot:append>
            <q-icon name="las la-times" v-if="loginInfo.remark" @click="loginInfo.remark = ''" />
          </template>
        </q-input>
      </q-card-section>

      <q-separator inset />

      <!-- 按钮示例 -->
      <q-card-actions align="right">
        <q-btn color="grey" round dense icon="las la-window-close" @click="onDialogCancel" />
        <q-btn
          color="green"
          round
          dense
          icon="las la-plus-square"
          :disable="isOKBtnDisabled"
          :loading="submitLoading"
          @click="onAddClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup>
import { reactive, ref, computed } from "vue"
import { invoke } from "@tauri-apps/api"
import { useCipherStore } from "stores/cipher"
import { useDialogPluginComponent } from "quasar"

const props = defineProps({
  cid: {
    type: String,
    default: "",
  },
})
defineEmits([
  // REQUIRED; 需要明确指出
  // 组件通过 useDialogPluginComponent() 暴露哪些事件
  ...useDialogPluginComponent.emits,
])

const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } = useDialogPluginComponent()
// dialogRef      - 用在 QDialog 上的 Vue ref 模板引用
// onDialogHide   - 处理 QDialog 上 @hide 事件的函数
// onDialogOK     - 对话框结果为 ok 时会调用的函数
//                    示例: onDialogOK() - 不带参数
//                    示例: onDialogOK({ /*.../* }) - 带参数
// onDialogCancel - 对话框结果为 cancel 时调用的函数

const cipherStore = useCipherStore()
let loginInfo = reactive({
  website: "",
  username: "",
  password: "",
  remark: "",
})
let isShowPwd = ref(false)
let submitLoading = ref(false)

const isOKBtnDisabled = computed(() => {
  return (
    !loginInfo.website ||
    !loginInfo.username ||
    !loginInfo.password ||
    loginInfo.password.length <= 3 ||
    loginInfo.username.length <= 3 ||
    loginInfo.website.length <= 3
  )
})

// 这是示例的内容，不是必需的
const onAddClick = () => {
  // REQUIRED！ 对话框的结果为 ok 时，必须调用 onDialogOK()  (参数是可选的)
  onDialogOK()
  // 带参数的版本: onDialogOK({ ... })
  // ...会自动关闭对话框
}
</script>

<style scoped></style>

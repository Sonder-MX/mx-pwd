import { createApp } from "vue"
import App from "./App.vue"
import router from "./router"
import * as ElementPlusIconsVue from "@element-plus/icons-vue"
import "element-plus/theme-chalk/src/message-box.scss"
import "element-plus/theme-chalk/src/message.scss"

const app = createApp(App)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.use(router).mount("#app")

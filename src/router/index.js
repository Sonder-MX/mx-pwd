import { createRouter, createWebHistory } from "vue-router"

const routes = [
  {
    path: "/",
    name: "Welcome",
    component: () => import("../components/Welcome.vue"),
  },
  {
    path: "/detail/:uid",
    name: "Detail",
    component: () => import("../components/DetailPage.vue"),
  },
  {
    path: "/test-api",
    name: "TestApi",
    component: () => import("../views/TestApi.vue"),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router

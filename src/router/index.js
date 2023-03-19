import { createRouter, createWebHistory } from "vue-router"

const routes = [
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

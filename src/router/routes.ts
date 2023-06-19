import { RouteRecordRaw } from "vue-router"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "HomePage",
    component: () => import("pages/HomePage.vue"),
    children: [
      {
        path: ":cid",
        name: "CipherDetail",
        component: () => import("pages/CipherDetail.vue"),
      },
      {
        path: "/welcome",
        name: "Welcome",
        component: () => import("pages/Welcome.vue"),
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: "/:catchAll(.*)*",
    component: () => import("pages/ErrorNotFound.vue"),
  },
]

export default routes

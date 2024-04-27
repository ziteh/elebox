import { createRouter, createWebHistory } from "vue-router";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: () => import("./views/Home.vue"),
    },
    {
      path: "/part",
      component: () => import("./views/Part.vue"),
    },
    {
      path: "/category",
      component: () => import("./views/Category.vue"),
    },
    {
      path: "/settings",
      component: () => import("./views/Settings.vue"),
    },
  ],
});

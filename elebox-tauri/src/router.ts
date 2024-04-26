import { createRouter, createWebHistory } from "vue-router";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: () => import("./views/PartList.vue"),
    },
    {
      path: "/new-part",
      component: () => import("./views/NewPart.vue"),
    },
    {
      path: "/new-type",
      component: () => import("./views/NewType.vue"),
    },
  ],
});

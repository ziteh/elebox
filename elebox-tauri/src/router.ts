import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("@/views/Home.vue"),
    },
    {
      path: "/new_part",
      name: "new_part",
      component: () => import("@/views/NewPart.vue"),
    },
    {
      path: "/categories",
      name: "categories",
      component: () => import("@/views/Category.vue"),
    },
    {
      path: "/manufacturers",
      name: "mfrs",
      component: () => import("@/views/Manufacturer.vue"),
    },
    {
      path: "/packages",
      name: "packages",
      component: () => import("@/views/Package.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/Settings.vue"),
    },
    {
      path: "/part:name",
      name: "part_detail",
      component: () => import("@/views/PartDetail.vue"),
    },
    {
      path: "/edit:item:name",
      name: "edit",
      component: () => import("@/views/EditItem.vue"),
    },
  ],
});

export default router;

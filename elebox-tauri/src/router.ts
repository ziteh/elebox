import { createRouter, createWebHistory } from "vue-router";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("./views/Home.vue"),
    },
    {
      path: "/parts",
      name: "new_part",
      component: () => import("./views/Part.vue"),
    },
    {
      path: "/categories",
      name: "categories",
      component: () => import("./views/Category.vue"),
    },
    {
      path: "/manufacturers",
      name: "mfrs",
      component: () => import("./views/Manufacturer.vue"),
    },
    {
      path: "/packages",
      name: "packages",
      component: () => import("./views/Package.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("./views/Settings.vue"),
    },
    {
      path: "/part:name",
      name: "part_detail",
      component: () => import("./views/PartDetail.vue"),
    },
    {
      path: "/update-part:ori_name",
      name: "update_part",
      component: () => import("./views/Part.vue"),
    },
    {
      path: "/update-category:origin_name",
      name: "update_category",
      component: () => import("./views/CategoryEdit.vue"),
    },
    {
      path: "/tree",
      name: "tree",
      component: () => import("./views/CategoriesTree.vue"),
    },
  ],
});

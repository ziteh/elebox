import { createApp } from "vue";
import "./styles.css";
import router from './router.js';
import App from "./App.vue";

createApp(App).use(router).mount("#app");

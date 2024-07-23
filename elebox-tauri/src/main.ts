import { createApp } from "vue";
import App from "./App.vue";
import router from "./router.js";
import i18n from "./plugins/i18n.js";
import vuetify from "./plugins/vuetify.js";
import "./styles.css";

const app = createApp(App);
app.use(router);
app.use(i18n);
app.use(vuetify);
app.mount("#app");

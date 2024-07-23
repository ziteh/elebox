import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import i18n from "@/plugins/i18n";
import vuetify from "@/plugins/vuetify";
import "@/styles.css";

const app = createApp(App);
app.use(router);
app.use(i18n);
app.use(vuetify);
app.mount("#app");

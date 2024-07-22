import { createApp } from "vue";
import router from "./router.js";
import App from "./App.vue";
import { createI18n } from "vue-i18n";
import en from "./locales/en.js";
import zhHant from "./locales/zhHant.js";

// Vuetify
import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import "@mdi/font/css/materialdesignicons.css"; // icon

const vuetify = createVuetify({
  theme: {
    defaultTheme: "dark",
  },
  components,
  directives,
});

const messages = {
  en,
  "zh-Hant": zhHant,
};

const i18n = createI18n({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
  messages,
});

import "./styles.css";

createApp(App).use(i18n).use(vuetify).use(router).mount("#app");

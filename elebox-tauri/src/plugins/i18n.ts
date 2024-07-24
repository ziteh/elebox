import { createI18n } from "vue-i18n";
import en from "../locales/en.js";
import zhHant from "../locales/zhHant.js";

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

export default i18n;

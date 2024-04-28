import { createApp } from "vue";
// import "./styles.css";
import router from './router.js';
import App from "./App.vue";

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import '@mdi/font/css/materialdesignicons.css' // icon

const vuetify = createVuetify({
    theme: {
        defaultTheme: "dark"
    },
    components,
    directives,
})

createApp(App).use(vuetify).use(router).mount("#app");

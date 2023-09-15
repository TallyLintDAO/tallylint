import { createApp } from "vue"
import App from "./App.vue"
import router from './router';
import { Quasar,Notify } from 'quasar'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
import '@quasar/extras/roboto-font-latin-ext/roboto-font-latin-ext.css'
// Import Quasar css
import 'quasar/src/css/index.sass'

import { createPinia } from "pinia"; //类似于vuex的状态管理

const app = createApp(App);
const pinia = createPinia();

app.use(router).use(pinia).use(Quasar, {
    plugins: { Notify }, // import Quasar plugins and add here
});

app.mount('#app');

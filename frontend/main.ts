import 'element-plus/dist/index.css'; // element plus css

import { createApp } from "vue"
import App from "./App.vue"
import router from './router';


const app = createApp(App);
app.use(router);

app.mount('#app');

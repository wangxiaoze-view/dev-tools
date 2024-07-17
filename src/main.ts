import "./assets/styles/main.scss";
import "remixicon/fonts/remixicon.css";
import "element-plus/es/components/message/style/css";
import "element-plus/es/components/message-box/style/css";
import { createApp } from "vue";
import App from "./App.vue";
import { setupStore } from "./stores";

const app = createApp(App);
setupStore(app);
app.mount("#app");

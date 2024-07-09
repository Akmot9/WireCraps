import { createApp } from "vue";
import "./styles.css";
import router from './router';
import App from "./App.vue";

const eventBus = {
    emit(event, data) {
      document.dispatchEvent(new CustomEvent(event, { detail: data }));
    },
    on(event, callback) {
      document.addEventListener(event, (e) => callback(e.detail));
    },
    off(event, callback) {
      document.removeEventListener(event, callback);
    },
  };

let app = createApp(App)

app.config.globalProperties.$bus = eventBus;
app.use(router);

app.mount("#app");

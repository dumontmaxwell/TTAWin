import { createApp } from "vue";
import { Quasar, Notify } from 'quasar';
import App from "./App.vue";
import { createPinia } from 'pinia'

import '@quasar/extras/material-icons/material-icons.css';
import '@fortawesome/fontawesome-free/css/all.css';

// Import our global theme
import { darkTheme } from './theme';

const app = createApp(App);
app.use(createPinia())

app.use(Quasar, {
  plugins: {
    Notify
  },
  config: {
    brand: darkTheme
  }
});

app.mount("#app");

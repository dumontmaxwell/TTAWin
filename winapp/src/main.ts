import { createApp } from "vue";
import { Quasar, Notify } from 'quasar';
import App from "./App.vue";
import { createPinia } from 'pinia'

import '@quasar/extras/material-icons/material-icons.css';
import '@fortawesome/fontawesome-free/css/all.css';

const app = createApp(App);
app.use(createPinia())

app.use(Quasar, {
  plugins: {
    Notify
  },
  config: {
    brand: {
      primary: '#1976D2',
      secondary: '#26A69A',
      accent: '#9C27B0',
      dark: '#1D1D1D',
      darkPage: '#121212',
      positive: '#21BA45',
      negative: '#C10015',
      info: '#31CCEC',
      warning: '#F2C037'
    }
  }
});

app.mount("#app");

import { createApp } from "vue";
import { Quasar, Notify } from 'quasar';
import App from "./App.vue";
import { createPinia } from 'pinia'

import '@quasar/extras/material-icons/material-icons.css';
import '@fortawesome/fontawesome-free/css/all.css';

// Import Inter font from Google Fonts
import '@fontsource/inter/300.css';
import '@fontsource/inter/400.css';
import '@fontsource/inter/500.css';
import '@fontsource/inter/600.css';
import '@fontsource/inter/700.css';

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

import './assets/css/tailwind.scss'
import './assets/css/font.css'


// import {createApp} from 'vue'
// import {createPinia} from 'pinia'
// import ElementPlus from 'element-plus'
// import 'element-plus/dist/index.css'
// import App from './App.vue'
// import {configMainRouter, router} from "./routers";
//
// const app = createApp(App)
//
// await configMainRouter(app)
//
// app.use(createPinia())
// app.use(ElementPlus)
//
//
// app.mount('#app')


// main.ts
import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'

const app = createApp(App)

app.use(ElementPlus)
app.mount('#app')

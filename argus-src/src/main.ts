import './assets/css/tailwind.scss'
import "./assets/icon/ali/iconfont.css"

import 'element-plus/dist/index.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import { lazyLoadDirective } from '@/directives/lazyLoad'

const app = createApp(App)

app.use(createPinia())
app.use(router)

// 注册指令
app.directive('argus-lazy', lazyLoadDirective)

app.mount('#app')

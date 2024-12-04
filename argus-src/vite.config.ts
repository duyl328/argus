import { fileURLToPath, URL } from 'node:url'

import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

import vueJsx from '@vitejs/plugin-vue-jsx'
// 自动生成路由
import Pages from 'vite-plugin-pages'
import { prismjsPlugin } from 'vite-plugin-prismjs'

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(({ command, mode }) => {
  const config = loadEnv(mode, './')
  console.log(config)
  return {
    build: {
      rollupOptions: {
        input: './index.html', // 指定唯一的入口文件
      },
    },
    
    plugins: [
      vue(),
      vueJsx(),
      Pages(),
      // element 自动引入
      AutoImport({
        resolvers: [ElementPlusResolver()]
      }),
      Components({
        resolvers: [ElementPlusResolver()]
      })
    
    ],
    // 为 Tauri 开发量身定制的 Vite 选项，仅适用于 `tauri dev` 或 `tauri build`
    //
    // 1.防止vite掩盖rust错误
    clearScreen: false,
    // 2. tauri 需要一个固定端口，如果该端口不可用则失败
    // 默认启动端口
    server: {
      // host: '0.0.0.0',
      port: 3133,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
          protocol: 'ws',
          host,
          port: 1421
        }
        : undefined,
      watch: {
        // 3. 告诉 vite 忽略观看 `src-tauri`
        ignored: ['**/src-tauri/**','**/temp/**','**/reference/**']
      }
    },
    optimizeDeps: {
      exclude: ['./temp', './reference'], // 排除不需要扫描的目录
    },
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      }
    }
  }
})

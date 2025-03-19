import { fileURLToPath, URL } from 'node:url'
import { defineConfig, configDefaults } from 'vitest/config'

export default defineConfig({
  test: {
    environment: 'jsdom',
    exclude: [...configDefaults.exclude, 'e2e/**'],
    root: fileURLToPath(new URL('./', import.meta.url)),
    // 测试环境的设置文件
    // setupFiles: './src/setupTests.ts',

    // 配置根路径别名
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})

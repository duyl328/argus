import { fileURLToPath } from 'node:url'
import { mergeConfig, defineConfig, configDefaults } from 'vitest/config'
import viteConfig from './vite.config'

export default mergeConfig(
  // viteConfig,
  defineConfig({
    test: {
      environment: 'jsdom',
      exclude: [...configDefaults.exclude, 'e2e/**'],
      root: fileURLToPath(new URL('./', import.meta.url)),
      // 测试环境的设置文件
      // setupFiles: './src/setupTests.ts',
      
    }
  }),defineConfig({
    test: {
      environment: 'jsdom',
      exclude: [...configDefaults.exclude, 'e2e/**'],
      root: fileURLToPath(new URL('./', import.meta.url)),
      // 测试环境的设置文件
      // setupFiles: './src/setupTests.ts',

    }
  }),
)
// https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip
// set HTTP_PROXY=127.0.0.1:7890
// set HTTPS_PROXY=127.0.0.1:7890

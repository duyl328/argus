# utopia-front-vue
项目安装依赖出错请把node版本调至以下版本
``` json
"node": ">=20.14.0 <=18.8.2",  
"npm": ">=10.7.0 <=9.8.1"
```
This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Type-Check, Compile and Minify for Production

```sh
npm run build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
npm run test:unit
```

### Run End-to-End Tests with [Cypress](https://www.cypress.io/)

```sh
npm run test:e2e:dev
```

This runs the end-to-end tests against the Vite development server.
It is much faster than the production build.

But it's still recommended to test the production build with `test:e2e` before deploying (e.g. in CI environments):

```sh
npm run build
npm run test:e2e
```

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```
https://vuetifyjs.com/zh-Hans/

## **Git 贡献提交规范**
***
* feat 新功能  
* fix 修补 bug  
* docs 文档  
* style 格式、样式(不影响代码运行的变动)  
* refactor 重构(即不是新增功能，也不是修改 BUG 的代码)  
* perf 优化相关，比如提升性能、体验  
* test 添加测试  
* build 编译相关的修改，对项目构建或者依赖的改动  
* ci 持续集成修改  
* chore 构建过程或辅助工具的变动  
* revert 回滚到上一个版本  
* workflow 工作流改进  
* mod 不确定分类的修改  
* wip 开发中  
* types 类型  

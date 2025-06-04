# Vuetify (默认模板)

这是Vuetify的官方脚手架工具，旨在为您构建新的Vuetify应用程序提供良好的开端。它设置了一个包含所有必要配置和标准目录结构的基础模板，让您能够立即开始开发，而无需从头开始设置项目。

## ❗️ 重要链接

- 📄 [文档](https://vuetifyjs.com/)
- 🚨 [问题反馈](https://issues.vuetifyjs.com/)
- 🏬 [商店](https://store.vuetifyjs.com/)
- 🎮 [在线演示](https://play.vuetifyjs.com/)
- 💬 [Discord社区](https://community.vuetifyjs.com)

## 💿 安装

使用您喜欢的包管理器设置项目。使用相应的命令安装依赖项：

| 包管理器                                                        | 命令            |
|---------------------------------------------------------------|----------------|
| [yarn](https://yarnpkg.com/getting-started)                   | `yarn install` |
| [npm](https://docs.npmjs.com/cli/v7/commands/npm-install)     | `npm install`  |
| [pnpm](https://pnpm.io/installation)                          | `pnpm install` |
| [bun](https://bun.sh/#getting-started)                        | `bun install`  |

完成安装后，您的环境就已准备好进行Vuetify开发。

## ✨ 功能特性

- 🖼️ **优化的前端技术栈**: 利用最新的Vue 3和Vuetify 3获得现代化、响应式的UI开发体验。[Vue 3](https://v3.vuejs.org/) | [Vuetify 3](https://vuetifyjs.com/en/)
- 🗃️ **状态管理**: 集成了[Pinia](https://pinia.vuejs.org/)，这是一个直观的、模块化的Vue状态管理解决方案。
- 🚦 **路由和布局**: 使用Vue Router进行SPA导航，使用vite-plugin-vue-layouts-next组织Vue文件布局。[Vue Router](https://router.vuejs.org/) | [vite-plugin-vue-layouts-next](https://github.com/loicduong/vite-plugin-vue-layouts-next)
- 💻 **增强的开发体验**: 受益于TypeScript的静态类型检查和Vue的ESLint插件套件，确保代码质量和一致性。[TypeScript](https://www.typescriptlang.org/) | [ESLint Plugin Vue](https://eslint.vuejs.org/)
- ⚡ **下一代工具**: 由Vite驱动，体验快速冷启动和即时热模块替换(HMR)。[Vite](https://vitejs.dev/)
- 🧩 **自动组件导入**: 使用unplugin-vue-components简化您的工作流程，在使用组件时自动导入它们。[unplugin-vue-components](https://github.com/antfu/unplugin-vue-components)
- 🛠️ **强类型Vue**: 使用vue-tsc对您的Vue组件进行类型检查，享受稳健的开发体验。[vue-tsc](https://github.com/johnsoncodehk/volar/tree/master/packages/vue-tsc)

这些功能经过精心策划，提供从设置到部署的无缝开发体验，确保您的Vuetify应用程序既强大又易于维护。

## 💡 使用方法

本节介绍如何启动开发服务器并为生产环境构建项目。

### 启动开发服务器

要启动带有热重载的开发服务器，运行以下命令。服务器将在 [http://localhost:3000](http://localhost:3000) 可访问：

```bash
yarn dev
```

（对于npm、pnpm和bun，使用相应的命令重复此操作。）

> 添加 NODE_OPTIONS='--no-warnings' 来抑制作为Vuetify导入映射一部分的JSON导入警告。如果您使用的是Node [v21.3.0](https://nodejs.org/en/blog/release/v21.3.0) 或更高版本，可以将其更改为 NODE_OPTIONS='--disable-warning=5401'。如果您不介意这个警告，可以从package.json的dev脚本中删除此项。

### 生产环境构建

要为生产环境构建项目，使用：

```bash
yarn build
```

（对于npm、pnpm和bun，使用相应的命令重复此操作。）

构建过程完成后，您的应用程序将准备好在生产环境中部署。

## 💪 支持Vuetify开发

此项目使用[Vuetify](https://vuetifyjs.com/en/)构建，这是一个包含全面Vue组件集合的UI库。Vuetify是一个MIT许可的开源项目，由于我们的[赞助商和支持者](https://vuetifyjs.com/introduction/sponsors-and-backers/)的慷慨贡献而成为可能。如果您有兴趣支持此项目，请考虑：

- [申请企业支持](https://support.vuetifyjs.com/)
- [在Github上赞助John](https://github.com/users/johnleider/sponsorship)
- [在Github上赞助Kael](https://github.com/users/kaelwd/sponsorship)
- [在Open Collective上支持团队](https://opencollective.com/vuetify)
- [在Patreon上成为赞助商](https://www.patreon.com/vuetify)
- [在Tidelift上成为订阅者](https://tidelift.com/subscription/npm/vuetify)
- [通过Paypal进行一次性捐赠](https://paypal.me/vuetify)

## 📑 许可证
[MIT](http://opensource.org/licenses/MIT)

版权所有 (c) 2016至今 Vuetify, LLC

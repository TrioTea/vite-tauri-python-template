# 组件

此文件夹中的Vue模板文件会自动导入。

## 🚀 使用方法

导入由 [unplugin-vue-components](https://github.com/unplugin/unplugin-vue-components) 处理。此插件自动导入在 `src/components` 目录中创建的 `.vue` 文件，并将它们注册为全局组件。这意味着您可以在应用程序中使用任何组件，而无需手动导入它。

以下示例假设组件位于 `src/components/MyComponent.vue`：

```vue
<template>
  <div>
    <MyComponent />
  </div>
</template>

<script lang="ts" setup>
  //
</script>
```

当您的模板被渲染时，组件的导入将自动内联，渲染结果如下：

```vue
<template>
  <div>
    <MyComponent />
  </div>
</template>

<script lang="ts" setup>
  import MyComponent from '@/components/MyComponent.vue'
</script>
```

## capabilities 文件夹下权限无用

### 表现：

> 在项目引入dialog插件后，并且默认的`default.json`文件中定义了所需权限，但是无法直接使用相应功能，报错：dialog.confirm not allowed. Permissions associated with this command: dialog:allow-confirm, dialog:default



> 相关链接：
>
> [[错误\]无法在 v2 中使用对话框 ·问题 #9273 ·陶里应用程序/陶里](https://github.com/tauri-apps/tauri/issues/9273)



### 解决方式：

将权限字段定义在 `tauri.conf.json` 中，在 `>app >security >capabilities` 字段下，将 `default.json` 所有内容移动至该字段下，配置生效，增加和删除均有对应效果

### 参考链接：

[能力 |陶里](https://v2.tauri.app/security/capabilities/)



猜测原因：

配置文件未被引用或生效



时间：2024年12月1日 20点09分



### 解决

原因：

在`src-tauri\capabilities\default.json` 文件中定义的属性：
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": [
    "main"
  ],
  "permissions": [
    "core:default",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "core:window:allow-unmaximize",
    "core:window:allow-close",
    "core:window:allow-start-dragging",
    "dialog:allow-open",
    "dialog:allow-ask",
    "dialog:allow-save",
    "dialog:allow-confirm",
    "dialog:default",
    "dialog:allow-message",
    "shell:allow-open",
    "shell:allow-kill",
    "shell:allow-stdin-write",
    "fs:default",
    "fs:allow-open",
    "fs:allow-write",
    "fs:allow-read",
    "fs:allow-rename",
    "fs:allow-mkdir",
    "fs:allow-remove",
    "fs:allow-write-text-file",
    "fs:scope-download-recursive",
    "fs:scope-resource-recursive"
  ]
}
```

`permissions` 中的属性重复定义导致所有的权限失效





## tuari 引入 freezePrototype 导致前端报错

参数位置：
```json
{
  "app": {
    "security": {
      "freezePrototype": false,
    }
  }
}
```

> `freezePrototype` 是一个安全相关的配置选项，用于限制或防止 JavaScript 原型链污染攻击。

> [!Tip]
>
> 设置该参数会导致前端的部分库失效如:`element-plus`和`lodash`



### **`freezePrototype` 的作用**

1. **防止原型链污染攻击**：

   - JavaScript 原型链污染是一种攻击方式，攻击者通过修改全局对象（如 `Object.prototype`）上的属性，影响应用程序中使用该对象的所有实例。

   - 例如：

     ```javascript
     Object.prototype.isAdmin = true;
     console.log({}.isAdmin); // true
     ```

   - `freezePrototype` 会通过“冻结”基础对象（如 `Object.prototype`、`Array.prototype` 等），防止这些对象被篡改。

2. **增强安全性**：

   - 在 Tauri 应用中，JavaScript 的运行环境可能具有访问用户文件系统或调用系统 API 的能力。为了防止恶意代码通过污染原型链来破坏应用的逻辑，`freezePrototype` 可以保护这些全局对象的完整性。

3. **默认行为**：

   - 如果 `freezePrototype` 被启用，Tauri 会在应用启动时冻结关键的 JavaScript 原型，从而限制对其进行动态修改。



启用后，Tauri 会在应用启动时冻结以下原型对象：

- `Object.prototype`
- `Array.prototype`
- `Function.prototype`
- 其他常见内置对象的原型



### **推荐实践**

1. **开发环境中禁用**：

   - 在开发环境中，可以暂时关闭 `freezePrototype` 以便于调试和使用第三方工具：

     ```json
     {
       "tauri": {
         "security": {
           "freezePrototype": false
         }
       }
     }
     ```

2. **生产环境中启用**：

   - 在生产环境中，为了提升安全性，建议启用 `freezePrototype`。

3. **测试兼容性**：

   - 在引入新的依赖库时，测试它们是否与冻结原型功能兼容。如果某些关键库不兼容，可以考虑调整项目的安全策略。






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
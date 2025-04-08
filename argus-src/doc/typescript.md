# **为什么 `const enum` 在 `export` 后会报错？**

在 TypeScript 中：

1. **`const enum` 只在编译时生效**，它的值在编译时会被 **内联** 进代码中，不会生成 JavaScript 运行时代码。
2. **当 `const enum` 被 `export` 之后，TypeScript 无法在别的文件中内联它的值**，因为 `const enum` 本质上不会被编译为一个 JavaScript 对象，所以在运行时它是 **undefined**。

### **示例**

#### **错误代码**

**`enums.ts`**

```
ts复制编辑export const enum FILE_SIZE_UNIT_ENUM {
    B = "B",
    KB = "KB",
    MB = "MB",
    GB = "GB",
    TB = "TB"
}
```

**`main.ts`**

```
ts复制编辑import { FILE_SIZE_UNIT_ENUM } from "./enums";

const isValidUnit = (unit: string): boolean => {
    return unit in FILE_SIZE_UNIT_ENUM; // ❌ 运行时报错
}
```

**⚠ 运行时报错**

> `FILE_SIZE_UNIT_ENUM` 在编译后不会存在于 `main.js`，导致 `in` 操作符无法在 `undefined` 上使用。

------

## **✅ 解决方案**

### **方案 1：改用普通 `enum`（推荐）**

如果你需要 `FILE_SIZE_UNIT_ENUM` 在运行时可用，可以改用 **普通 `enum`**：

```
ts复制编辑// enums.ts
export enum FILE_SIZE_UNIT_ENUM {
    B = "B",
    KB = "KB",
    MB = "MB",
    GB = "GB",
    TB = "TB"
}
```

这样，`FILE_SIZE_UNIT_ENUM` 会在 JavaScript 运行时 **保留为一个对象**，`in` 操作符就可以正常工作。

------

### **方案 2：使用 `Object.values()`**

如果你仍然希望使用 `const enum`，可以创建一个数组来存储合法值：

```
ts复制编辑// enums.ts
export const enum FILE_SIZE_UNIT_ENUM {
    B = "B",
    KB = "KB",
    MB = "MB",
    GB = "GB",
    TB = "TB"
}

// 额外创建一个数组
export const FILE_SIZE_UNIT_VALUES = Object.values(FILE_SIZE_UNIT_ENUM);
```

**`main.ts`**

```
ts复制编辑import { FILE_SIZE_UNIT_VALUES } from "./enums";

const isValidUnit = (unit: string): boolean => {
    return FILE_SIZE_UNIT_VALUES.includes(unit);
}

console.log(isValidUnit("KB")); // ✅ true
console.log(isValidUnit("XX")); // ❌ false
```

✅ **这样 `const enum` 仍然可以在编译时优化，但 `FILE_SIZE_UNIT_VALUES` 在运行时是可用的**。





# TS中常见DOM泛型

| 元素       | 类型                |
| ---------- | ------------------- |
| `<div>`    | `HTMLDivElement`    |
| `<img>`    | `HTMLImageElement`  |
| `<canvas>` | `HTMLCanvasElement` |
| `<input>`  | `HTMLInputElement`  |
| `<video>`  | `HTMLVideoElement`  |
| `<audio>`  | `HTMLAudioElement`  |

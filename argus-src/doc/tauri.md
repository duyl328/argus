# `tauri://file` 的优势

`tauri://file` 是 Tauri 提供的一种协议

优势如下：

1. **安全性**
   - `tauri://file` 是受 Tauri 管理的协议，它对资源访问进行了控制，避免直接暴露本地文件路径给前端，从而减少被恶意攻击的风险。
   - 通过这种方式，前端无法直接访问本地文件系统的绝对路径，有助于保护用户隐私和应用的文件结构。
2. **跨平台一致性**
   - 由于不同操作系统的文件路径格式不同，直接使用路径可能导致跨平台问题。`tauri://file` 协议抽象化了这一层，使得路径在不同平台表现一致。
3. **打包支持**
   - 在生产环境中，Tauri 的应用通常是打包的。`tauri://file` 自动适配了生产和开发环境，无需担心路径是否有效或是否需要调整。
4. **热更新和资源管理**
   - 使用 `tauri://file`，Tauri 能够更高效地管理和加载静态资源，比如图片、字体等，而不用担心路径问题。
   - 对于开发环境中的热更新，协议处理通常比硬编码路径更优雅。







# 对于 tauri 的理解

他基于 rust 作为后端并套壳，并且完全不依赖前端，只要前端定义好指定的端口，后端在配置时使用同样的端口监听

```json
{
  "build": {
    "devUrl": "http://localhost:3128",
  }
}
```

后端启动时，匹配到前端端口，展示前端页面，整体来说，后端更像是一个浏览器，而后端将数据在浏览器中构建展示

并且 tauri 相对于真正的浏览器，提供了更多的可配置化选项，比如：指令的监听（并通过rust执行并返回）、更多的可配置项（如freezePrototype）



1. **前端与后端的关系**：

   - **前端**：Tauri 前端实际上可以是任何静态资源（HTML、CSS、JS），并不局限于网络地址（如 `http://localhost:3128`），它也可以直接从本地文件加载（通过 `dist` 文件夹的打包静态资源）。
   - **后端**：Tauri 后端由 Rust 实现，但并非传统意义的“监听前端端口”。后端是一个包含 WebView 的桌面应用程序，前端资源通过 WebView 加载到窗口中渲染显示。
   - **devUrl**：你提到的 `"devUrl"` 是开发模式下的一个配置项，它用于指向前端的开发服务器地址（如 Vite、Webpack DevServer 等）。在生产模式下，Tauri 会将前端资源打包为静态文件，而不是从网络加载。

2. **WebView 的角色**：

   - Tauri 的后端并不是一个完整的浏览器，而是基于 **WebView**（Windows 上使用 WebView2）实现的轻量级嵌入式浏览器。
   - WebView 提供了渲染前端页面的能力，并允许通过 API 与 Rust 后端进行双向通信。

3. **指令监听和事件通信**：

   - Tauri 提供了一个基于 Rust 和前端通信的 **Command 和 Event System**。
   - 指令（Command）：前端可以通过调用特定的 Rust 命令（通常是异步函数），让后端执行任务并返回结果。
   - 事件（Event）：前端或后端可以触发事件，并通过监听机制进行双向通信。

4. **更多的可配置选项**：

   - 安全性

     ：相比于真正的浏览器，Tauri 允许开发者严格控制 WebView 的行为，例如：

     - 禁用 JavaScript 原型污染（freezePrototype）。
     - 配置 CSP（内容安全策略）。
     - 限制文件访问路径等。

   - **文件管理**：Tauri 提供内置 API，用于访问文件系统，而不需要通过网络协议（如 HTTP）。

   - **多窗口支持**：Tauri 可以轻松创建和管理多个窗口，每个窗口可以加载不同的前端资源。



# tauri接受`&str`或`String`参数

### **1. 使用 `String` 接收**

- 推荐在以下场景使用：
  - 你需要修改或操作字符串。
  - 你希望在函数中获取字符串的所有权。
- 在这种情况下，`String` 类型会获取数据的所有权，适合需要对字符串进行复杂处理的场景。

### **2. 使用 `&str` 接收**

- 如果后端只需要对字符串进行只读操作而不需要修改，理论上可以使用 `&str`。
- **但在 Tauri 的命令函数中，直接使用 `&str` 不会编译通过**，因为从 JSON 数据中解析字符串时，Rust 无法保证引用的生命周期。

### **两种方式都可以用，但建议优先用 `String`**

由于 Tauri 的前端数据传递是以 JSON 格式传输，最终会转化为 Rust 的数据类型，`String` 是更直接且方便的选择。后端收到数据后，如果需要只读操作，可以再借用为 `&str`。





# tauri构建时下载wix报错 2024年12月6日 21点39分

报错详情：
```rust
warning: argus-src (lib) generated 37 warnings (run cargo fix --lib -p argus-src to apply 19 suggestions)                                                                                                                      
    Finished release profile [optimized] target(s) in 3m 07s                                                                                                                                                                     
    Built application at: D:\argus\argus-src\src-tauri\target\release\argus-src.exe
    Info Verifying wix package
    Downloading https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip
failed to bundle project: https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip: Connection Failed: Connect error: 由于目标计算机积极拒绝，无法连接。 (os error 10061)
    Error failed to bundle project: https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip: Connection Failed: Connect error: 由于目标计算机积极拒绝，无法连接。 (os error 10061)
```

该错误是因为在构建过程中 tauri 需要引用 `wix` 进行构建，但是下载一直无法完成导致出错。

解决过程：

1. 尝试通过浏览器直接下载对应链接文件，下载成功，猜测连接问题
2. 将node、npm、cargo都配置代理后尝试下载无法成功
3. 通过ping和curl尝试下载无法成功，通过设置代理后下载成功，但构建依然不成功
4. 在`stackoverflow`后，也有人遇到类似问题，链接：https://stackoverflow.com/questions/62567792/can-i-manually-download-tauri-buiding-dependencies

> # [Can I manually download Tauri buiding dependencies?](https://stackoverflow.com/questions/62567792/can-i-manually-download-tauri-buiding-dependencies)
>
> Due to some reasons, my host matchine can not get access to the Internet, the only way to build the app is to manually download all dependecies on another disk. However, I was stuck in the wix package, which is downloaded from github. I've successfully downloaded it from my browser, but I don't know where to place it.
>
> Is there any way to manually download all dependencies?
>
> I was stuck here (on another disk):
>
> ```rust
> info: Running Loopback command
>    Compiling app v0.1.0 (D:\frontend\src-tauri)
>    Compiling tauri v0.6.0
>     Finished release [optimized] target(s) in 19.28s
> info: Verifying wix package
> info: Downloading https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip
> ```
>
> Ans:
>
> On a windows machine, you can put the contents of https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip in `%LOCALAPPDATA%/tauri/WixTools`.

5. 根据回答尝试在根目录下创建对应的`WixTools`并重新构建，问题依然存在，开始尝试在`github`寻找答案
6. `github`有相关问题：https://github.com/tauri-apps/tauri/discussions/3770

> # Download wix package fail...help #3770
>
> ## [![img](../tauri.assets/59404696-1733491695592-1.png)peterroe](https://github.com/peterroe)[on Mar 24, 2022](https://github.com/tauri-apps/tauri/discussions/3770#discussion-3960065)
>
> When I run:
>
> `$ npx vue-cli-service tauri:build`
>
> The error happend:
>
> ```rust
> Compiling app v0.1.0 (D:\VscodeItems\tauri-app\src-tauri)
>     Finished release [optimized] target(s) in 22.91s
> info: Verifying wix package
> info: Downloading https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip
> (node:10460) UnhandledPromiseRejectionWarning: Error: failed to bundle project: `Tls Error: The remote host forcibly closed an existing connection. (os error 10054)`: Tls Error: An existing connection was forcibly closed by the remote host. (os error 10054)
> (Use `node --trace-warnings ...` to show where the warning was created)
> (node:10460) UnhandledPromiseRejectionWarning: Unhandled promise rejection. This error originated either by throwing inside of an async function without a catch block, or by rejecting a promise which was not handled with .catch(). To terminate the node process on unhandled promise rejection, use the CLI flag `--unhandled-rejections=strict` (see https://nodejs.org/api/cli.html#cli_unhandled_rejections_mode). (rejection id: 1)
> (node:10460) [DEP0018] DeprecationWarning: Unhandled promise rejections are deprecated. In the future, promise rejections that are not handled will terminate the Node.js process with a non-zero exit code.
> ```
>
> 
>
> I dont't know what should I do... help :(
>
> my node env:
>
> `$ node v14+`
>
> I try to upgrade my node version to `v16/v17`，it prompts another error:
>
> ```rust
> Compiling app v0.1.0 (D:\VscodeItems\tauri-app\src-tauri)
>      Finished release [optimized] target(s) in 21.85s
> info: Verifying wix package
> info: Downloading https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip
> node:internal/process/promises:279
>              triggerUncaughtException(err, true /* fromPromise */);
>              ^
> 
> [Error: failed to bundle project: `Io Error: No such host known. (os error 11001)`: Io Error: No such host known. (os error 11001)] {
>    code: 'GenericFailure'
> }
> ```
>
> 
>
> Answered by [FabianLars](https://github.com/FabianLars)[on Mar 24, 2022](https://github.com/tauri-apps/tauri/discussions/3770#discussioncomment-2430117)
>
> It's probably blocked by a firewall or something. Can you try downloading [it](https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip) manually and unpack it into the src-tauri dir so that the project structure looks similar to this:
>
> ```
> ./
>   src/
>   src-tauri/
>     WixTools/
>       many files, for example "candle.exe"
> ```
>
> ## Replies:1 comment · 6 replies
>
> ### [![img](../tauri.assets/30730186.jpeg)FabianLars](https://github.com/FabianLars)[on Mar 24, 2022](https://github.com/tauri-apps/tauri/discussions/3770#discussioncomment-2430117)Maintainer
>
> It's probably blocked by a firewall or something. Can you try downloading [it](https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip) manually and unpack it into the src-tauri dir so that the project structure looks similar to this:
>
> ```rust
> ./
>   src/
>   src-tauri/
>     WixTools/
>       many files, for example "candle.exe"
> ```
>
> [![@FabianLars](../tauri.assets/30730186-1733491695593-3.jpeg)](https://github.com/FabianLars)
>
> #### [FabianLars](https://github.com/FabianLars)[on Aug 28, 2022](https://github.com/tauri-apps/tauri/discussions/3770#discussioncomment-3490109)Maintainer
>
> Actually the location of the WixTools directory changed. It is now in 
>
> `C:\Users\your-username\AppData\Local\tauri\WixTools`
>
> 
>
> [![@liudonghua123](../tauri.assets/2276718.jpeg)](https://github.com/liudonghua123)
>
> 
>
> #### [liudonghua123](https://github.com/liudonghua123)[on Jul 24, 2023](https://github.com/tauri-apps/tauri/discussions/3770#discussioncomment-6526331)
>
> Yes, the location is changed, you should extract [`wix311-binaries.zip`](https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip) to `%LOCALAPPDATA%\tauri\WixTools`, not put `wix311-binaries.zip` in `%LOCALAPPDATA%\tauri\WixTools` or you will got `Warn WixTools directory is missing some files. Recreating it.` warnings.
>
> See https://github.com/tauri-apps/tauri/blob/b7277357b960521b04b4d9153bd402953cd483df/tooling/bundler/src/bundle/windows/msi.rs#L28C1-L44
>
> 
>
> [![@liudonghua123](../tauri.assets/2276718.jpeg)](https://github.com/liudonghua123)
>
> 
>
> #### [liudonghua123](https://github.com/liudonghua123)[on Jul 24, 2023](https://github.com/tauri-apps/tauri/discussions/3770#discussioncomment-6526383)
>
> And you should also do the similar things for nsis. Extract [nsis-3.zip](https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip) to `%LOCALAPPDATA%\tauri\NSIS`, [NSIS-ApplicationID.zip/ReleaseUnicode/ApplicationID.dll](https://github.com/tauri-apps/binary-releases/releases/download/nsis-plugins-v0/NSIS-ApplicationID.zip) to `%LOCALAPPDATA%\tauri\NSIS\Plugins\x86-unicode` and [nsis_tauri_utils.dll](https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.1.1/nsis_tauri_utils.dll) to `%LOCALAPPDATA%\tauri\NSIS\Plugins\x86-unicode`.
>
> See
>
> [tauri/tooling/bundler/src/bundle/windows/nsis.rs](https://github.com/tauri-apps/tauri/blob/b7277357b960521b04b4d9153bd402953cd483df/tooling/bundler/src/bundle/windows/nsis.rs#L68-L84)Lines 68 to 84 in [b727735](https://github.com/tauri-apps/tauri/commit/b7277357b960521b04b4d9153bd402953cd483df) 
>
> ```rust
>  pub fn bundle_project(settings: &Settings, updater: bool) -> crate::Result<Vec<PathBuf>> { 
>    let tauri_tools_path = dirs_next::cache_dir().unwrap().join("tauri"); 
>    let nsis_toolset_path = tauri_tools_path.join("NSIS"); 
>   
>    if !nsis_toolset_path.exists() { 
>      get_and_extract_nsis(&nsis_toolset_path, &tauri_tools_path)?; 
>    } else if NSIS_REQUIRED_FILES 
>      .iter() 
>      .any(|p| !nsis_toolset_path.join(p).exists()) 
>    { 
>      warn!("NSIS directory is missing some files. Recreating it."); 
>      std::fs::remove_dir_all(&nsis_toolset_path)?; 
>      get_and_extract_nsis(&nsis_toolset_path, &tauri_tools_path)?; 
>    } 
>   
>    build_nsis_app_installer(settings, &nsis_toolset_path, &tauri_tools_path, updater) 
>  } 
> ```
>
> 
>
> and[tauri/tooling/bundler/src/bundle/windows/nsis.rs](https://github.com/tauri-apps/tauri/blob/b7277357b960521b04b4d9153bd402953cd483df/tooling/bundler/src/bundle/windows/nsis.rs#L87-L122)Lines 87 to 122 in [b727735](https://github.com/tauri-apps/tauri/commit/b7277357b960521b04b4d9153bd402953cd483df) 
>
> ```rust
>  fn get_and_extract_nsis(nsis_toolset_path: &Path, _tauri_tools_path: &Path) -> crate::Result<()> { 
>    info!("Verifying NSIS package"); 
>   
>    #[cfg(target_os = "windows")] 
>    { 
>      let data = download_and_verify(NSIS_URL, NSIS_SHA1, HashAlgorithm::Sha1)?; 
>      info!("extracting NSIS"); 
>      extract_zip(&data, _tauri_tools_path)?; 
>      rename(_tauri_tools_path.join("nsis-3.08"), nsis_toolset_path)?; 
>    } 
>   
>    let nsis_plugins = nsis_toolset_path.join("Plugins"); 
>   
>    let data = download(NSIS_APPLICATIONID_URL)?; 
>    info!("extracting NSIS ApplicationID plugin"); 
>    extract_zip(&data, &nsis_plugins)?; 
>   
>    create_dir_all(nsis_plugins.join("x86-unicode"))?; 
>   
>    copy( 
>      nsis_plugins 
>        .join("ReleaseUnicode") 
>        .join("ApplicationID.dll"), 
>      nsis_plugins.join("x86-unicode").join("ApplicationID.dll"), 
>    )?; 
>   
>    let data = download_and_verify(NSIS_TAURI_UTILS, NSIS_TAURI_UTILS_SHA1, HashAlgorithm::Sha1)?; 
>    write( 
>      nsis_plugins 
>        .join("x86-unicode") 
>        .join("nsis_tauri_utils.dll"), 
>      data, 
>    )?; 
>   
>    Ok(()) 
>  } 
> ```

7. `github`的内容可描述为：维护者已经将`WixTools`的地址从根目录更改到了`$LOCALAPPDATA$`目录中，所以需要在`$LOCALAPPDATA\tauri\WixTools`目录解压相关文件，再次尝试，报错依然

8. 重新整理报错信息，报错信息前有`Info`输出：`Verifying wix package` ,直接在源代码中查找在https://github.com/tauri-apps/tauri/blob/b37741da6a2d3dad71490c910a64eeedda2ba9ca/crates/tauri-bundler/src/bundle/windows/msi/mod.rs#L261中有相关内容

   > ```rust
   > // Specifically goes and gets Wix and verifies the download via Sha256
   > pub fn get_and_extract_wix(path: &Path) -> crate::Result<()> {
   >   log::info!("Verifying wix package");
   > 
   >   let data = download_and_verify(WIX_URL, WIX_SHA256, HashAlgorithm::Sha256)?;
   > 
   >   log::info!("extracting WIX");
   > 
   >   extract_zip(&data, path)
   > }
   > ```
   >
   > 该信息在该地方报出，通过查询`get_and_extract_wix`的调用，在https://github.com/tauri-apps/tauri/blob/b37741da6a2d3dad71490c910a64eeedda2ba9ca/crates/tauri-bundler/src/bundle/windows/msi/mod.rs#L64 中查询到唯二的调用：
   >
   > ```rust
   > /// Runs all of the commands to build the MSI installer.
   > /// Returns a vector of PathBuf that shows where the MSI was created.
   > pub fn bundle_project(settings: &Settings, updater: bool) -> crate::Result<Vec<PathBuf>> {
   >   let tauri_tools_path = settings
   >     .local_tools_directory()
   >     .map(|d| d.join(".tauri"))
   >     .unwrap_or_else(|| dirs::cache_dir().unwrap().join("tauri"));
   > 
   >   let wix_path = tauri_tools_path.join("WixTools314");
   > 
   >   if !wix_path.exists() {
   >     get_and_extract_wix(&wix_path)?;
   >   } else if WIX_REQUIRED_FILES
   >     .iter()
   >     .any(|p| !wix_path.join(p).exists())
   >   {
   >     log::warn!("WixTools directory is missing some files. Recreating it.");
   >     std::fs::remove_dir_all(&wix_path)?;
   >     get_and_extract_wix(&wix_path)?;
   >   }
   > 
   >   build_wix_app_installer(settings, &wix_path, updater)
   > }
   > ```
   >
   > 通过阅读代码发现，作者已将`WixTools`名称更改为：`WixTools314`

9. 将`$LOCALAPPDATA$\tauri\WixTools`名称更改为`WixTools314`，Wix不再下载，正常执行。新的报错如下：

```rust
warning: `argus-src` (lib) generated 37 warnings (run `cargo fix --lib -p argus-src` to apply 19 suggestions)                                                                               
    Finished `release` profile [optimized] target(s) in 49.32s                                                                          
    Built application at: D:\argus\argus-src\src-tauri\target\release\argus-src.exe
    Info Target: x64
    Downloading https://go.microsoft.com/fwlink/?linkid=2124701
    Running candle for "main.wxs"
    Running light to produce D:\argus\argus-src\src-tauri\target\release\bundle\msi\argus-src_0.0.1_x64_en-US.msi
    Warn NSIS directory contains mis-hashed files. Redownloading them.
    Downloading https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll
failed to bundle project: `https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll: Connection Failed: Connect error: 由于目标计算机积极拒绝，无法连接。 (os error 10061)`
    Error failed to bundle project: `https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll: Connection Failed: Connect error: 由于目标计算机积极拒绝，无法连接。 (os error 10061)`
```

错误和刚刚类似，同样是查找数据不存在导致报错，采用同样的思路

查找代码如下：

在https://github.com/tauri-apps/tauri/blob/b37741da6a2d3dad71490c910a64eeedda2ba9ca/crates/tauri-bundler/src/bundle/windows/nsis/mod.rs#L108文件，有以下代码:

```rust
/// Runs all of the commands to build the NSIS installer.
/// Returns a vector of PathBuf that shows where the NSIS installer was created.
pub fn bundle_project(settings: &Settings, updater: bool) -> crate::Result<Vec<PathBuf>> {
  let tauri_tools_path = settings
    .local_tools_directory()
    .map(|d| d.join(".tauri"))
    .unwrap_or_else(|| dirs::cache_dir().unwrap().join("tauri"));

  let nsis_toolset_path = tauri_tools_path.join("NSIS");

  if !nsis_toolset_path.exists() {
    get_and_extract_nsis(&nsis_toolset_path, &tauri_tools_path)?;
  } else if NSIS_REQUIRED_FILES
    .iter()
    .any(|p| !nsis_toolset_path.join(p).exists())
  {
    log::warn!("NSIS directory is missing some files. Recreating it.");
    std::fs::remove_dir_all(&nsis_toolset_path)?;
    get_and_extract_nsis(&nsis_toolset_path, &tauri_tools_path)?;
  } else {
    let mismatched = NSIS_REQUIRED_FILES_HASH
      .iter()
      .filter(|(p, _, hash, hash_algorithm)| {
        verify_file_hash(nsis_toolset_path.join(p), hash, *hash_algorithm).is_err()
      })
      .collect::<Vec<_>>();

    if !mismatched.is_empty() {
      log::warn!("NSIS directory contains mis-hashed files. Redownloading them.");
      for (path, url, hash, hash_algorithm) in mismatched {
        let data = download_and_verify(url, hash, *hash_algorithm)?;
        fs::write(nsis_toolset_path.join(path), data)?;
      }
    }
  }

  build_nsis_app_installer(settings, &nsis_toolset_path, &tauri_tools_path, updater)
}

// Gets NSIS and verifies the download via Sha1
fn get_and_extract_nsis(nsis_toolset_path: &Path, _tauri_tools_path: &Path) -> crate::Result<()> {
  log::info!("Verifying NSIS package");

  #[cfg(target_os = "windows")]
  {
    let data = download_and_verify(NSIS_URL, NSIS_SHA1, HashAlgorithm::Sha1)?;
    log::info!("extracting NSIS");
    crate::utils::http_utils::extract_zip(&data, _tauri_tools_path)?;
    fs::rename(_tauri_tools_path.join("nsis-3.08"), nsis_toolset_path)?;
  }

  let nsis_plugins = nsis_toolset_path.join("Plugins");

  let data = download_and_verify(
    NSIS_TAURI_UTILS_URL,
    NSIS_TAURI_UTILS_SHA1,
    HashAlgorithm::Sha1,
  )?;

  let target_folder = nsis_plugins.join("x86-unicode");
  fs::create_dir_all(&target_folder)?;
  fs::write(target_folder.join("nsis_tauri_utils.dll"), data)?;

  Ok(())
}
```

这里包含了该报错信息，将指定文件手动下载并移动至`$LOCALAPPDATA$\tauri\NSIS`文件夹中，尝试重新运行

同样的报错(经过验证目标文件和使用文件SHA1不同)；将文件移动至`$LOCALAPPDATA$\tauri\NSIS\Plugins\x86-unicode`再次尝试，构建成功。

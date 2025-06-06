// 对于Rust来说，他有推荐的日志框架吗？
// 需求如下：
// 多级别日志（info/debug/warn/error）
// 支持控制台输出
// 支持输出到文件
// 日志文件大小超限时自动切割
// 日志压缩（如每周压缩归档）
// 日志定期清理（如保留一个月）
// 日志记录中包含源码位置（文件名、行号）
// 性能不应显著影响主程序运行
// ✅ 异步日志写入（避免阻塞主线程，尤其是高并发场景）
// ✅ 可配置性强（配置文件支持）
// ✅ JSON 或结构化日志支持（方便机器分析，如 ELK、Loki 等）
// ✅ 过滤不同模块/目标的日志等级（如只打某个模块的 debug）
// 
// 基于此完成日志工具类的封装！要求使用简单优雅、可拓展性强
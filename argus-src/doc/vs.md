2024年12月31日 21点57分42秒
- 图像信息读取
  - 尝试内容包括：kamadak-exif，exif,exif-rc 等，但是这些库都是只能读数据，不支持写入或者编辑，故放弃
  - 目前的解决方案：使用 exiftool 完成内容的读取（待尝试）和写入
  - 未来的解决方案：尝试构建一个关于rust的exif信息读取和写入（可使用nom等库）

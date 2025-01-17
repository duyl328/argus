/**
 * 获取图片绝对路径
 */
export const getImageAbsolutePathCommand = 'get_image_absolute_path'

/**
 * 问候
 */
export const greetCommand = 'greet'
/**
 * 全局报错 emit
 */
export const emitGlobalMsgCommand = 'emit_global_msg'

/**
 * 检查指定路径是否有权限
 */
export const checkDirectoryAccessCommand = 'check_directory_access'

/**
 * 读取照片为 Base64
 */
export const readImageAsBase64Command = 'read_image_as_base64'
export const getAllPostCommand = 'get_all_post'
export const insertPostCommand = 'insert_post'
export const logLogsCommand = 'log_logs'
/**
 * 获取所有图像
 */
export const getPhotoStorageCommand = 'get_photo_storage'
/**
 * 插入图像地址
 */
export const addPhotoStorageCommand = 'add_photo_storage'
export const updatePhotoStorageCommand = 'update_photo_storage'
export const deletePhotoStorageCommand = 'delete_photo_storage'
/**
 * 获取指定路径所有子路径的第一张图片，没有图片则不返回内容
 */
export const getDirAllSubfoldersFirstImgCommand = 'get_dir_all_subfolders_first_img'

/**
 * 生成保存压缩缩略图
 */
export const generateSaveThumbnailCommand = 'generate_save_thumbnail'
/**
 * 获取指定的压缩图片地址
 */
export const getImageThumbnailPathCommand = 'get_image_thumbnail_path'
/**
 * 获取指定图片的缩略图【如果不存在，直接创建】
 */
export const getImageThumbnailCommand = 'get_image_thumbnail'
/**
 * 获取所有照片路径
 */
export const getAllImgsCommand = 'get_all_imgs'

using Common.ExpandException;
using Common.Utils.Log;
using Repository.Contexts;
using Repository.entity;

namespace Repository.Servers
{
    public class PhotoStoragePathServer
    {
        private static readonly Lazy<PhotoStoragePathServer> Instance = new(() => new PhotoStoragePathServer());

        public static PhotoStoragePathServer GetPhotoStoragePathServer()
        {
            return Instance.Value;
        }

        private PhotoStoragePathServer()
        {
        }

        /// <summary>
        ///     将存储路径添加到数据库
        /// </summary>
        /// <param name="storagePath">需要添加的路径</param>
        public void AddPhotoStoragePath(PhotoStoragePath storagePath)
        {
            var storagePathStoragePath = storagePath.StoragePath.Trim();
            if (string.IsNullOrWhiteSpace(storagePathStoragePath))
            {
                throw ExceptionEnum.PhotoStoragePathCannotEmpty;
            }

            using var context = new PhotoStoragePathHelper();
            context.Database.EnsureCreated();

            using var transaction = context.Database.BeginTransaction(); // 开始事务

            try
            {
                var existingPath = context.PhotoStoragePathTable
                    .FirstOrDefault(p => p.StoragePath == storagePathStoragePath);
                if (existingPath != null)
                {
                    throw ExceptionEnum.PathsCannotRepeatedly;
                }

                context.PhotoStoragePathTable.Add(storagePath);
                context.SaveChanges();

                transaction.Commit(); // 提交事务
                LoggerFacade.Logger.Info($"路径存储成功 存储内容:{storagePath}");
                

            }
            catch (Exception ex)
            {
                transaction.Rollback(); // 如果出错，回滚事务
                throw;
            }
        }


        /// <summary>
        ///     删除一个存储内容
        /// </summary>
        /// <param name="storagePath">删除指定的路径</param>
        public void RemovePhotoStoragePath(string storagePath)
        {
            if (storagePath == null)
            {
                throw ExceptionEnum.PhotoStoragePathCannotIsNull;
            }

            using var context = new PhotoStoragePathHelper();
            context.Database.EnsureCreated();

            using var transaction = context.Database.BeginTransaction(); // 开始事务
            try
            {
                var photoStoragePath = context.PhotoStoragePathTable
                    .FirstOrDefault(p => p.StoragePath == storagePath);

                if (photoStoragePath != null)
                {
                    context.PhotoStoragePathTable.Remove(photoStoragePath);
                    context.SaveChanges();
                }

                transaction.Commit(); // 提交事务
                LoggerFacade.Logger.Info($"路径删除成功 删除内容:{storagePath}");
            }
            catch (Exception ex)
            {
                transaction.Rollback(); // 回滚事务
                throw; // 保持原始异常抛出
            }
        }

        /// <summary>
        ///     指定一个路径是否启用
        /// </summary>
        /// <param name="storagePath">要操作的路径</param>
        /// <param name="isEnable">设置指定状态</param>
        public void UpdatePhotoStorageState(string storagePath, bool isEnable)
        {
            var storagePathStoragePath = storagePath.Trim();
            if (string.IsNullOrWhiteSpace(storagePathStoragePath))
            {
                throw ExceptionEnum.PhotoStoragePathCannotEmpty;
            }

            using var context = new PhotoStoragePathHelper();
            context.Database.EnsureCreated();

            using var transaction = context.Database.BeginTransaction();

            try
            {
                var existingPath = context.PhotoStoragePathTable
                    .FirstOrDefault(p => p.StoragePath == storagePathStoragePath);

                if (existingPath == null)
                {
                    throw ExceptionEnum.PhotoStoragePathNotFound; // 如果不存在，抛出一个路径未找到的异常
                }

                existingPath.IsEnable = isEnable; // 更新状态
                context.PhotoStoragePathTable.Update(existingPath);
                context.SaveChanges();

                transaction.Commit();
                LoggerFacade.Logger.Info($"路径更新成功 更新内容:{existingPath}, 更新前:{storagePath}, {isEnable}");
            }
            catch (Exception ex)
            {
                transaction.Rollback();
                throw;
            }
        }


        /// <summary>
        ///     获取所有存储数据
        /// </summary>
        /// <returns>将查询的值全部返回</returns>
        public List<PhotoStoragePath> GetPhotoStoragePaths()
        {
            using var context = new PhotoStoragePathHelper();
            context.Database.EnsureCreated(); // 确保数据库已创建
            var photoStoragePaths = context.PhotoStoragePathTable;
            return photoStoragePaths.ToList();
        }
    }

}

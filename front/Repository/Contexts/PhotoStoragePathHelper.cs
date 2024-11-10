using Common.Utils.Log;
using Microsoft.EntityFrameworkCore;
using Repository.entity;

namespace Repository.Contexts
{
    public class PhotoStoragePathHelper : DbContext
    {
        /// <summary>
        ///     照片存储路径
        /// </summary>
        public DbSet<PhotoStoragePath> PhotoStoragePathTable { get; set; }

        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
        {
            optionsBuilder.UseSqlite("Data Source=photos.db");
        }


        public override int SaveChanges()
        {
            // 这里可以添加额外的逻辑
            return base.SaveChanges();
        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            modelBuilder.Entity<PhotoStoragePath>()
                .ToTable("PhotoStoragePathTable"); // 自定义表名
        }
        /// <summary>
        ///     使用 Migrations 进行数据库结构的自动更新
        ///
        /// 每次当你更改模型时，你可以通过以下命令生成新的迁移：
        ///     dotnet ef migrations add MigrationName
        /// </summary>
        public void InitializeDatabase()
        {
            // 自动应用所有挂起的迁移
            Database.Migrate();
        }
    }
}

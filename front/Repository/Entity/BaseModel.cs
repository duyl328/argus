namespace Repository.entity
{
    public class BaseModel
    {
        public long Id { get; set; }

        public long CreateTime { get; set; }
        public long UpdateTime { get; set; }
        public bool IsDelete { get; set; }

    }
}

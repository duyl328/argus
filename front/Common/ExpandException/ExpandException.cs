namespace Common.ExpandException
{
    public class ExpandException : Exception
    {
        /// <summary>
        /// 默认异常编码
        /// </summary>
        public const long DEFAULT_CODE = -1;

        /// <summary>
        /// 异常编码
        /// </summary>
        public long ErrorCode { get; private set; } = DEFAULT_CODE;

        /// <summary>
        ///     建议
        /// </summary>
        public string Solution { get; private set; }

        private ExpandException()
        {
        }

        public ExpandException(long errorCode, string massage, string solution) : base(
            massage)
        {
            ErrorCode = errorCode;
            Solution = solution;
        }

        public ExpandException(long errorCode, string massage, string solution, Exception? innerException) : base(
            massage, innerException)
        {
            ErrorCode = errorCode;
            Solution = solution;
        }

        public override string ToString()
        {
            return $"Error Code: {ErrorCode}\nMessage: {Message}\nSolution: {Solution}\n{base.ToString()}";
        }
    }
}

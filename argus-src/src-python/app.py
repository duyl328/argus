import socket

from waitress import serve
from app import create_app

app = create_app()


# 使用 socket 获取一个可用的随机端口
def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('127.0.0.1', 0))  # 绑定端口为 0，表示自动选择一个空闲端口
        return s.getsockname()[1]  # 返回自动选择的端口


# 打包
# pyinstaller --onefile app.py

if __name__ == "__main__":
    print("======================================= 程序启动 =======================================")
    # 获取一个空闲端口
    port = find_free_port()

    print(f"Flask 将运行在端口 {port}")
    # 使用 Waitress 启动 Flask 应用并指定动态获取的端口
    serve(app, host='127.0.0.1', port=5000)


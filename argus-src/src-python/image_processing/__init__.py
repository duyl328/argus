import cv2
import sys

if __name__ == "__main__":
    sys.path.append(r"D:\argus\env\opencv\build\python\cv2\python - 3.10")


    # 打印 OpenCV 构建信息
    print(cv2.getBuildInformation())
    print("CUDA enabled devices:", cv2.cuda.getCudaEnabledDeviceCount())  # 检查可用设备

    print(sys.version)

    print("======================================= 程序启动 =======================================")


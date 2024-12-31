import os

import cv2
import torch
from ultralytics import YOLO
import numpy as np


def object_detection():
    # 加载预训练的 YOLOv8 模型
    model = YOLO('./models/yolo11l.pt').to(device)  # 使用 'yolov8s.pt'，你也可以选择其他模型

    # 读取图像
    image = cv2.imread('./img/bus.jpg')

    # 使用模型进行检测
    results = model(image)

    # 获取检测结果并在图像上绘制边框
    annotated_image = results[0].plot()

    # 显示带有检测框的图像
    # cv2.imshow('Detected Image', annotated_image)
    cv2.waitKey(0)
    cv2.destroyAllWindows()

    # 保存带有检测框的图像
    cv2.imwrite('detected_bus.jpg', annotated_image)

    # 获取类别名称的字典（可选）
    class_names = model.names

    # 遍历检测结果
    for box in results[0].boxes:
        # 获取类别 ID
        class_id = int(box.cls)
        # 获取类别名称
        class_name = class_names[class_id] if class_names else f'Class {class_id}'
        # 获取置信度
        confidence = box.conf.item()
        # 获取位置信息（x1, y1, x2, y2）
        x1, y1, x2, y2 = map(int, box.xyxy[0])
        # 输出检测信息
        print(f'检测到 {class_name}，置信度：{confidence:.2f}，位置：({x1}, {y1}), ({x2}, {y2})')
        # print(f'检测到 {class_name}，置信度：{confidence:.2f}')


if __name__ == "__main__":
    device = 'cuda' if torch.cuda.is_available() else 'cpu'
    print(f'Using device: {device}')
    object_detection()
    # face_detection()

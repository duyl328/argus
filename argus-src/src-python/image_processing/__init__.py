import os

import cv2
import torch
from ultralytics import YOLO
import numpy as np
from insightface.app import FaceAnalysis


def face_detection():
    model = YOLO('yolov8x-face-lindevs.pt')  # 请确保使用适用于人脸检测的模型权重

    # 读取图像
    image = cv2.imread('bus.jpg')
    # 使用模型进行检测
    results = model(image)

    # 创建保存裁剪人脸的目录
    output_dir = 'cropped_faces'
    os.makedirs(output_dir, exist_ok=True)
    # 遍历每个检测到的人脸
    for i, box in enumerate(results[0].boxes):
        print(box.xyxy[0])
        # 获取边界框坐标
        x1, y1, x2, y2 = map(int, box.xyxy[0])

        # 裁剪人脸区域
        face = image[y1:y2, x1:x2]

        # 保存裁剪的人脸图像
        face_filename = os.path.join(output_dir, f'face_{i}.jpg')
        cv2.imwrite(face_filename, face)

        print(f'人脸已保存至: {face_filename}')


def object_detection():
    # 加载预训练的 YOLOv8 模型
    model = YOLO('yolov8m.pt').to(device)  # 使用 'yolov8s.pt'，你也可以选择其他模型

    # 读取图像
    image = cv2.imread('bus.jpg')

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



def insightface_use():
    # 初始化
    app = FaceAnalysis(providers=['CPUExecutionProvider'])  # 如果有 GPU，可使用 'CUDAExecutionProvider'
    # app = FaceAnalysis(providers=['CUDAExecutionProvider'])  # 如果有 GPU，可使用 'CUDAExecutionProvider'
    app.prepare(ctx_id=0, det_size=(640, 640))  # 检测模型的输入大小

    # 加载图像
    image = cv2.imread("512.jpg")
    faces = app.get(image)

    # 遍历检测到的人脸
    for face in faces:
        bbox = face.bbox.astype(int)  # 人脸边界框
        feature_vector = face.normed_embedding  # 512 维特征向量
        cropped_face = image[bbox[1]:bbox[3], bbox[0]:bbox[2]]  # 截取人脸区域

        print("Bounding Box:", bbox)
        print("Feature Vector:", feature_vector)
        cv2.imshow("Cropped Face", cropped_face)
        cv2.waitKey(0)


if __name__ == "__main__":
    if torch.cuda.is_available():
        print("CUDA is available!")
    else:
        print("CUDA is not available.")
    device = 'cuda' if torch.cuda.is_available() else 'cpu'
    print(f'Using device: {device}')
    # object_detection()
    # face_detection()
    insightface_use()
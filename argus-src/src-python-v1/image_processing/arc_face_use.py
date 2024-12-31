import os

import numpy as np
import onnxruntime
from insightface.app import FaceAnalysis
import cv2
import faiss
from matplotlib import pyplot as plt
from sklearn.cluster import DBSCAN
from sklearn.decomposition import PCA

model_pack_name = 'buffalo_l'
# 初始化人脸分析器（默认为 ArcFace）
app = FaceAnalysis(name=model_pack_name)
app.prepare(ctx_id=0, det_size=(640, 640))  # ctx_id=0 表示使用 GPU，如果没有 GPU 可以设置为 -1


# arcFace 使用测试
def arc_face_use(path):
    # 读取图片
    img = cv2.imread(path)

    # 进行人脸检测和特征提取
    faces = app.get(img)

    # 输出检测到的所有人脸特征
    # for face in faces:
    #     print("Feature vector:", face.embedding)  # 输出 512 维特征向量
    return faces
    # 设置扩展的边界框比例（可以调整此比例）
    expand_ratio = 0.4  # 扩展20%

    # 保存人脸到本地
    for i, face in enumerate(faces):
        # 获取人脸的边界框
        bbox = face.bbox  # 左上角(x1, y1), 右下角(x2, y2)
        x1, y1, x2, y2 = map(int, bbox)  # 转换为整数

        # 截取人脸区域
        # 打印边界框坐标
        print(f"Face {i} bounding box: ({x1}, {y1}, {x2}, {y2})")
        # 计算扩展的边界框
        width = x2 - x1
        height = y2 - y1

        # 扩展边界框的大小，增加上下文
        x1 = int(x1 - width * expand_ratio)  # 向左扩展
        y1 = int(y1 - height * expand_ratio)  # 向上扩展
        x2 = int(x2 + width * expand_ratio)  # 向右扩展
        y2 = int(y2 + height * expand_ratio)  # 向下扩展

        # 确保裁剪的坐标在有效范围内
        x1 = max(0, x1)
        y1 = max(0, y1)
        x2 = min(img.shape[1], x2)  # x2 不能超出图像宽度
        y2 = min(img.shape[0], y2)  # y2 不能超出图像高度

        # 打印修正后的坐标
        print(f"Corrected bounding box: ({x1}, {y1}, {x2}, {y2})")
        cropped_face = img[y1:y2, x1:x2]
        # 保存裁剪后的图片
        cv2.imwrite(f"cropped_face_{i}.jpg", cropped_face)


# 人脸识别
def custom_model_use():
    session = onnxruntime.InferenceSession("D:/argus/new/pythonProject8/Face Recognition models/model.onnx")

    # 加载图片
    img = cv2.imread('D:/argus/new/pythonProject8/selfie-many-people.jpg')

    # 将图像从 BGR 转为 RGB 格式
    img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

    # 假设模型需要 112x112 尺寸的输入图像
    img_resized = cv2.resize(img_rgb, dsize=(112, 112))

    # 对图像进行归一化
    img_normalized = (img_resized / 255.0).astype(np.float32)

    # 转换为 (C, H, W) 格式，并增加批次维度
    img_input = np.transpose(img_normalized, (2, 0, 1))  # HWC -> CHW
    img_input = np.expand_dims(img_input, axis=0)  # 增加批次维度 (1, C, H, W)

    # 推理输入
    inputs = {session.get_inputs()[0].name: img_input}
    outputs = session.run(None, inputs)

    # 输出人脸特征向量
    feature_vector = outputs[0]
    print("Feature Vector:", feature_vector)


# 人脸检测
def face_detect_use():
    # 设置模型路径
    model_path = 'path_to_your_model/retinaface-R50'

    # 加载FaceAnalysis应用
    app = FaceAnalysis()
    app.prepare(ctx_id=0, det_size=(640, 640), model_path=model_path)  # 指定模型路径

    # 加载图片
    img = cv2.imread('your_image.jpg')

    # 检测人脸
    faces = app.get(img)

    # 遍历检测到的人脸
    for face in faces:
        # 获取人脸的坐标
        bbox = face.bbox.astype(int)

        # 抠出人脸区域
        face_img = img[bbox[1]:bbox[3], bbox[0]:bbox[2]]

        # 保存抠出来的人脸
        cv2.imwrite('extracted_face.jpg', face_img)

        # 也可以画框标出人脸位置
        cv2.rectangle(img, (bbox[0], bbox[1]), (bbox[2], bbox[3]), (0, 255, 0), 2)

    # 保存带框的人脸图像
    cv2.imwrite('image_with_faces.jpg', img)


# 初始化 FAISS 索引，假设人脸特征是 512 维的
d = 512  # 特征向量的维度
index = faiss.IndexFlatL2(d)  # 创建一个基于L2距离的索引


# 向量存储
def faiss_use(facess):
    i1 = 0

    feature_vectors = []
    ids = []

    for face in facess:
        face_np = np.array(face, dtype=np.float32)
        dimension = face_np.shape[0]
        # xb = np.random.random((nb, d)).astype('float32')
        # xb[:, 0] += np.arange(nb) / 1000.

        feature_vectors.append(face_np)
        ids.append(i1)
        i1 += 1

    feature_vectors = np.array(feature_vectors)  # 转换为 NumPy 数组，形状为 (n, 512)
    print(ids)
    # ids = np.arange(d)
    ids = np.array(ids, dtype=np.int64)  # IDs 必须是 np.int64 类型
    print(ids)
    # 将向量添加到索引中
    # index.add(feature_vectors)
    index2 = faiss.IndexIDMap(index)
    index2.add_with_ids(feature_vectors, ids)

    fc = arc_face_use("D:/argus/argus-src/src-python-v1/image_processing/img/img1.jpg")[0].embedding
    # 查询最近的邻居
    k = 4  # we want to see 4 nearest neighbors
    fc = np.expand_dims(fc, axis=0)  # 或者 fc.reshape(1, -1)

    D, I = index2.search(fc, k)  # actual search
    print(I)
    print(D)
    print("============================================")
    # 构建邻接矩阵（这里使用距离矩阵来构造邻接矩阵）
    # 你可以通过调整阈值来选择相似度较高的点作为邻居
    # 假设我们选择距离小于一定阈值的点作为邻居
    threshold = 0.5  # 距离阈值，决定哪些点可以作为邻居
    # 从 FAISS 索引中提取出所有向量
    vectors = index.reconstruct_n(0, index.ntotal)  # 提取从索引 0 到 index.ntotal 的所有向量
    print(vectors.shape)
    # 初始化 PCA，指定降到 50 维
    pca = PCA(n_components=5)  # 你可以根据需要选择降到 50 维或 100 维

    # 使用 PCA 对数据进行降维
    reduced_features = pca.fit_transform(vectors)

    # print(f"原始数据形状: {face_features.shape}")
    # print(f"降维后的数据形状: {reduced_features.shape}")
    # 计算累积方差解释比例
    explained_variance_ratio = pca.explained_variance_ratio_
    cumulative_explained_variance = np.cumsum(explained_variance_ratio)
    print(cumulative_explained_variance)

    # 可视化方差
    plt.plot(np.arange(1, len(cumulative_explained_variance) + 1), cumulative_explained_variance)
    plt.xlabel('Number of Components')
    plt.ylabel('Cumulative Explained Variance')
    plt.title('Explained Variance by Number of Components')
    plt.show()

    # 使用 DBSCAN 聚类
    # eps 控制了两点之间的最大距离（即最大邻域范围）
    # min_samples 控制着一个簇中至少要有多少个点才能被认为是一个有效的簇。
    db = DBSCAN(eps=27, min_samples=1, metric='euclidean')  # DBSCAN 参数，eps 是距离阈值，min_samples 是最小样本数
    labels = db.fit_predict(reduced_features)  # 进行聚类，返回每个向量的簇标签

    # 输出聚类结果
    print("聚类标签:", labels)
    print("聚类数目:", len(set(labels)) - (1 if -1 in labels else 0))  # -1 是噪声点


def get_files_by_extension(directory, extension):
    # 遍历指定目录
    result = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(extension):
                result.append(os.path.join(root, file))  # 拼接完整路径
    return result


if __name__ == '__main__':
    paths = get_files_by_extension("D:/argus/argus-src/src-python-v1/image_processing/img", ".jpg")
    ans = []

    for path in paths:
        facess1 = arc_face_use(path)
        print(facess1.__len__())
        ans.append(facess1[0].embedding)
    faiss_use(ans)
    print('PyCharm')

# pip install insightface opencv-python
# pip install torch torchvision torchaudio
# pip install onnxruntime

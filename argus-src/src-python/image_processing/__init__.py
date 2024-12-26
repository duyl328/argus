from functools import partial
from multiprocessing import Pool
import os

import torch


def image_process(args):
    path, of = args  # 解包参数
    from PIL import Image
    import torch
    import torchvision.transforms as t

    try:
        img = Image.open(path)
        transform = t.ToTensor()
        image_tensor = transform(img).unsqueeze(0).to('cuda')

        original_height, original_width = image_tensor.shape[2], image_tensor.shape[3]
        target_width = 512
        aspect_ratio = original_width / original_height
        target_height = int(target_width * aspect_ratio)

        resized_tensor = torch.nn.functional.interpolate(image_tensor, size=(target_width, target_height),
                                                         mode='bilinear')
        to_pil = t.ToPILImage()
        resized_image = to_pil(resized_tensor.squeeze(0).cpu())
        output_path = os.path.join(of, os.path.basename(path))
        # output_path = f"output/{os.path.basename(path)}"
        print(output_path)
        resized_image.save(output_path, format="JPEG", quality=100)
        return f"{path} processed successfully."
    except Exception as e:
        return f"Error processing {path}: {e}"


def get_all_images(folder, extensions=(".jpg", ".png")):
    """递归获取文件夹中的所有图片路径"""
    image_paths = []
    for root, _, files in os.walk(folder):
        for file in files:
            if file.lower().endswith(extensions):
                image_paths.append(os.path.join(root, file))
    return image_paths



if __name__ == "__main__":
    if torch.cuda.is_available():
        print("CUDA is available!")
    else:
        print("CUDA is not available.")
    # print(torch.version.cuda)  # PyTorch 支持的 CUDA 版本
    # print(torch.cuda.get_arch_list())  # 支持的 GPU 架构
    # 增大像素限制（设置为 1 亿像素或更多）
    # Image.MAX_IMAGE_PIXELS = None  # 取消限制

    output_folder = "D:/argus/img/output"
    os.makedirs(output_folder, exist_ok=True)

    image_paths = get_all_images("D:/argus/img/jpg")
    args_list = [(path, output_folder) for path in image_paths]

    with Pool(processes=2) as pool:  # 使用 4 个子进程
        results = pool.map(image_process, args_list)

    for res in results:
        print(res)

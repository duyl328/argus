import type { Directive } from 'vue'
import { getImageThumbnail } from '@/services/imageService'
import { convertFileSrc } from '@tauri-apps/api/core'

/**
 * Time:2024/12/22 15:57 22
 * Name:lazyLoad.ts
 * Path:src/directives
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */

export const lazyLoadDirective: Directive = {
  mounted(el, binding) {
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) {
        let imageThumbnailPath = getImageThumbnail(binding.value)
        console.log(el.childNodes);
        // el.childNodes.forEach((ele) => {
        //   ele.class
        // })
        imageThumbnailPath
          .then((res) => {
            el.src = convertFileSrc(res)
            el.classList.remove('img-load')
          })
          .catch((err) => {
            console.error("获取失败!",err);
            el.src = '/src/assets/images/img_example_not_exist.png'
            el.classList.add('img-load')
            el.alt = '图像获取失败: ' + err.toString()
          })
          .finally(() => {
            observer.unobserve(el) // 停止观察
          })
      }
    })
    observer.observe(el)
  }
}
// 懒加载

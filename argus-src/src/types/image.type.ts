/**
 * Time:2025/2/16 13:21 22
 * Name:image.type.ts
 * Path:src/types
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
/**
 * 基础图片信息
 */
export type ImageInfo = {
  imgPath: string
  imgName: string
  hash: string
  width: number
  height: number
  aspectRatio: number
  fileSize: number
  format: string
  notes: string | null
  isAlgorithm: boolean | null
  algorithmScore: number | null
  lastViewedTime: number | null
  offsetTime: number | null
  rating: number | null
  make: string | null
  model: string | null
  software: string | null
  exposureTime: number | null
  flash: string | null
  fNumber: number | null
  iso: number | null
  dateTimeOriginal: number | null
  maxApertureValue: string | null
  focalLength: number | null
  imageWidth: number | null
  imageHeight: number | null
  gpsInfo: string | null
  exposureProgram: string | null
  meteringMode: string | null
  artist: string | null
  isDelete: boolean
  createTime: number
  updateTime: number
}

/**
 * 图 exif 信息
 */

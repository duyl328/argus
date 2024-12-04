import {invoke} from "@tauri-apps/api/core";
import {getImageAbsolutePathCommand, readImageAsBase64Command} from '@/command';

/**
 * 获取图片路径
 */
export function getImg(){
    return invoke<string>(getImageAbsolutePathCommand)
}


export function readImageAsBase64(file_path:string):Promise<string> {
    return invoke<string>(readImageAsBase64Command,{directory:file_path});
}

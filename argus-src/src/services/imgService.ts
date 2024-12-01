import {invoke} from "@tauri-apps/api/core";
import {getImageAbsolutePathCommand} from "../command.ts";

/**
 * 获取图片路径
 */
export function getImg(){
    return invoke<string>(getImageAbsolutePathCommand)
}

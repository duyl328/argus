import {invoke} from "@tauri-apps/api/core";
import {checkDirectoryAccessCommand} from '@/constants/command';

type ans = {
    ans: boolean
    msg: string
}

/**
 * 检查软件是否有对应权限
 * @param filePath
 */
export function checkDirectoryAccess(filePath: string): Promise<ans> {
    return invoke<ans>(checkDirectoryAccessCommand, {directory: filePath});
}

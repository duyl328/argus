import {invoke} from "@tauri-apps/api/core";

import {greetCommand} from '@/command';

/**
 * 问候
 * @param name
 */
export function greet(name: string) {
    return invoke<string>(greetCommand, {name});
}

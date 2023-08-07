import type { Plugin } from 'vite';

import type { ViteEnv } from '../../../types/model';

import vue from '@vitejs/plugin-vue';
import { viteHtmlPlugins } from './html';
import { viteCompressionPlugin } from './compression';
import { legacyPlugin } from "./legacy";

export function createVitePlugins(viteEnv: ViteEnv, isBuild: boolean) {
    const {
        VITE_BUILD_COMPRESS: compressType,
        VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: shouldBuildCompressDeleteFile,
    } = viteEnv;

    const vitePlugins: (Plugin | Plugin[])[] = [];

    vitePlugins.push(vue());
    vitePlugins.push(...viteHtmlPlugins(viteEnv, isBuild)); // 注入配置字符串
    vitePlugins.push(legacyPlugin()); // 低级浏览器支持

    if (isBuild) {
        // 生成模式执行
        vitePlugins.push(
            viteCompressionPlugin(compressType, shouldBuildCompressDeleteFile), // 压缩
        );
    }

    return vitePlugins;
}

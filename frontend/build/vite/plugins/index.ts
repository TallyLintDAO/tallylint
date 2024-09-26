import type { PluginOption } from "vite"
import type { ViteEnv } from "../../../types/model"

import { quasar, transformAssetUrls } from "@quasar/vite-plugin"
import vue from "@vitejs/plugin-vue"
import { createHtmlPlugin } from "vite-plugin-html"
import { viteCompressionPlugin } from "./compression"

export function createVitePlugins(viteEnv: ViteEnv, isBuild: boolean) {
  const {
    VITE_BUILD_COMPRESS: compressType,
    VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: shouldBuildCompressDeleteFile,
  } = viteEnv

  const vitePlugins: (PluginOption | PluginOption[])[] = [
    vue({
      template: { transformAssetUrls },
    }),
    quasar({
      sassVariables: "@/assets/css/quasar-variables.sass",
    }),
    createHtmlPlugin({
      minify: true,
      inject: {
        data: {
          keywords: viteEnv.VITE_KEYWORDS,
          author: viteEnv.VITE_AUTHOR,
          description: viteEnv.VITE_DESCRIPTION,
          noScriptTitle: viteEnv.VITE_TITLE,
          title: viteEnv.VITE_TITLE,
        },
      },
    }),
  ]

  if (isBuild) {
    // 生成模式执行
    vitePlugins.push(
      viteCompressionPlugin(compressType, shouldBuildCompressDeleteFile), // 压缩
    )
  }

  return vitePlugins
}

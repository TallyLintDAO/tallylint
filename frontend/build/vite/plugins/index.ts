import type { Plugin } from "vite"

import type { ViteEnv } from "../../../types/model"

import { quasar, transformAssetUrls } from "@quasar/vite-plugin"
import vue from "@vitejs/plugin-vue"
import { viteCompressionPlugin } from "./compression"
import { viteHtmlPlugins } from "./html"

export function createVitePlugins(viteEnv: ViteEnv, isBuild: boolean) {
  const {
    VITE_BUILD_COMPRESS: compressType,
    VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: shouldBuildCompressDeleteFile,
  } = viteEnv

  const vitePlugins: (Plugin | Plugin[])[] = []

  vitePlugins.push(
    vue({
      template: { transformAssetUrls },
    }),
  )
  vitePlugins.push(
    quasar({
      sassVariables: "frontend/assets/css/quasar-variables.sass",
    }),
  )
  vitePlugins.push(...viteHtmlPlugins(viteEnv, isBuild)) // 注入配置字符串

  if (isBuild) {
    // 生成模式执行
    vitePlugins.push(
      viteCompressionPlugin(compressType, shouldBuildCompressDeleteFile), // 压缩
    )
  }

  return vitePlugins
}

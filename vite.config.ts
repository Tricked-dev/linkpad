import UnoCSS from "unocss/vite";
import { defineConfig } from "vite";
import extractorSvelte from "@unocss/extractor-svelte";
import { presetAtoUI } from "./src/lib/ui/preset/index";
import presetIcons from "@unocss/preset-icons";
import { presetUno } from "unocss";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import transformerDirectives from "@unocss/transformer-directives";
import transformerVariantGroup from "@unocss/transformer-variant-group";
// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    svelte(),
    UnoCSS({
      presets: [
        presetIcons({
          extraProperties: {
            display: "inline-block",
            "vertical-align": "middle",
          },
        }),
        presetUno(),
        presetAtoUI(),
      ],
      extractors: [extractorSvelte()],
      transformers: [transformerVariantGroup(), transformerDirectives()],
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));

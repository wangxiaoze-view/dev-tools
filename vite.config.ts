import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve, basename } from "path";

import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [
		vue(),
		AutoImport({
			resolvers: [ElementPlusResolver()],
		}),
		Components({
			resolvers: [ElementPlusResolver()],
		}),
	],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	// 别名
	resolve: {
		alias: {
			// "@/": "/",
			"@": "/src",
		},
	},

	css: {
		devSourcemap: false,
		preprocessorOptions: {
			scss: {
				sassOptions: { outputStyle: "expanded" },
				additionalData(content: string, loaderContext: string) {
					// @ts-ignore
					return ["variables.scss"].includes(basename(loaderContext))
						? content
						: `@use "@/assets/styles/theme/variables.scss" as *;${content}`;
				},
			},
		},
	},
	server: {
		port: 1420,
		strictPort: true,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));

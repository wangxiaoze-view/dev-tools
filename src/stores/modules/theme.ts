import { defineStore } from "pinia";
import { ref } from "vue";
import { useCssVar } from "@vueuse/core";

export type ThemeColor = "blue" | "green" | "red" | "purple";
export type Size = "large" | "default" | "small";

export const useThemeStore = defineStore("theme", {
	state: () => ({
		// blue, green, red, purple
		theme: localStorage.getItem("theme") || "blue",
		// large, default, small
		size: localStorage.getItem("size") || "default",
	}),

	getters: {
		getTheme: state => state.theme,
		getSize: state => state.size,
	},

	actions: {
		setTheme(theme: ThemeColor) {
			this.theme = theme;
			localStorage.setItem("theme", theme);
			this.updateTheme();
		},
		setSize(size: Size) {
			this.size = size;
			localStorage.setItem("size", size);
		},

		updateTheme() {
			const themeColor = this.theme || "blue";
			const variables = import.meta.glob(
				`../../assets/styles/theme/*.module.scss`,
				{
					eager: true,
				}
			);

			for (const [key, value] of Object.entries(variables)) {
				if (key.indexOf(themeColor) !== -1) {
					// eslint-disable-next-line @typescript-eslint/no-explicit-any
					const theme = (value as any).default;
					Object.keys(theme).forEach(k => {
						if (k.startsWith("sim-")) {
							const prop = k.replace("sim-", "--el-");
							const target = ref(null);
							useCssVar(prop, target).value = theme[k];
						}
					});
				}
			}
			return Promise.resolve(true);
		},
	},
});

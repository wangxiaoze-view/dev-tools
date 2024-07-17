import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface IFnmStatus {
	status: boolean;
	stdout: string;
	stderr: string;
	statusText: string;
}
export const useAppStore = defineStore("app", {
	state: () => ({}),

	getters: {},

	actions: {
		async openLink(link: string) {
			await invoke("open_link", {
				url: link,
			});
		},
	},
});

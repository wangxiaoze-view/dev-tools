import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface IFnmStatus {
	status: boolean;
	stdout: string;
	stderr: string;
	statusText: string;
}
export const useGitStore = defineStore("git", {
	state: () => ({}),

	getters: {},

	actions: {
		async getGitConfig() {
			const result = await invoke("get_git_config");
			return result as IFnmStatus;
		},
		async setGitConfig(name: string, email: string, autocrlf: string) {
			const result = await invoke("set_git_config", {
				name,
				email,
				autocrlf,
			});
			return result as IFnmStatus;
		},
	},
});

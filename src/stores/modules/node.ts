import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface IFnmStatus {
	status: boolean;
	stdout: string;
	stderr: string;
	statusText: string;
}

export const useNodeStore = defineStore("node", {
	state: () => ({
		isInstallFnm: false,
		allNodeVersion: [],
	}),

	getters: {
		getIsInstallFnm: state => state.isInstallFnm,
		getterAllNodeVersion: state => state.allNodeVersion,
	},

	actions: {
		async checkIsInstallFnm() {
			const isInstall: boolean = await invoke("is_installed", {
				program: "fnm",
			});
			this.isInstallFnm = isInstall;
			return Promise.resolve(isInstall);
		},

		async installFnm(): Promise<IFnmStatus> {
			const result = await invoke("install_fnm");
			return result as IFnmStatus;
		},

		async getNodeVersion(): Promise<string> {
			const result = await invoke("get_node_version");
			return result as string;
		},

		async getAllNodeVersion(): Promise<any> {
			const result = await (
				await fetch("https://nodejs.org/dist/index.json")
			).json();

			let filterData = result.filter((i: any) => {
				const v = i.version.replace(/v/, "").split(".")[0];
				return v >= 16;
			});
			this.allNodeVersion = filterData;
			return filterData;
		},

		async installNode(version: string) {
			const result = await invoke("install_node", {
				version: version.replace("v", ""),
			});
			return result as IFnmStatus;
		},

		async getInstalledNode() {
			const result: any = await invoke("get_list_node");
			return result.versions
				.map((i: string) => {
					i = i
						.replace(/\*/g, "")
						.replace("default", "")
						.replace(/\s|\r|\n|\t/g, "");
					return i;
				})
				.filter((i: string) => !["", "system"].includes(i));
		},

		async changeNode(version: string) {
			const result = await invoke("change_node", {
				version: version.replace("v", ""),
			});
			return result as IFnmStatus;
		},

		async deleteNode(version: string) {
			const result = await invoke("delete_node", {
				version: version.replace("v", ""),
			});
			return result as IFnmStatus;
		},

		async getRegistry() {
			const result = await invoke("get_npm_registry");
			return result as string;
		},
		async setRegistry(registry: string) {
			const result = await invoke("set_npm_registry", {
				registry: registry,
			});
			return result as IFnmStatus;
		},
		async getNpmView(packageName: string) {
			const result = await invoke("get_npm_package", { package: packageName });
			return result as IFnmStatus;
		},
		async getInstallNpmView() {
			const result = await invoke("get_list_npm_package");
			const { status, stdout } = result as IFnmStatus;
			if (!status) return {};
			return (stdout as any).dependencies;
		},
		async installNpmView(packageName: string) {
			const result = await invoke("install_npm_package", {
				package: packageName,
			});
			return result as IFnmStatus;
		},

		// 检测包更新
		async checkNpmViewVersion(name: string): Promise<any> {
			const result = await (
				await fetch("https://mirrors.cloud.tencent.com/npm/" + name)
			).json();
			return result["dist-tags"]["latest"];
		},

		async deleteNpmView(packageName: string) {
			const result = await invoke("delete_npm_package", {
				package: packageName,
			});
			return result as IFnmStatus;
		},

		async updateNpmView(packageName: string) {
			const result = await invoke("update_npm_package", {
				package: packageName,
			});
			return result as IFnmStatus;
		},
	},
});

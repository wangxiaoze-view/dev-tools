<script setup lang="ts">
import { ref } from "vue";
import { IFnmStatus, useNodeStore } from "@/stores/modules/node";
import { home_config } from "@/config";

const nodeStore = useNodeStore();
const visible = ref(false);
const result = ref<Partial<IFnmStatus>>({});

const isExit = home_config.find(i => i.name === "fnm");
const url = ref(isExit ? isExit.web : "");

const setVisible = async (show: boolean) => {
	visible.value = show;
	result.value = {};

	show && (await getInstallResult());
};

const getInstallResult = async () => {
	const { status, stderr, stdout } = await nodeStore.installFnm();
	const isInstall = await nodeStore.checkIsInstallFnm();
	setTimeout(() => {
		result.value = {
			status,
			stderr,
			stdout,
			statusText: isInstall ? "安装成功" : "安装失败",
		};
	}, 700);
};

defineExpose({
	setVisible,
});
</script>

<template>
	<el-dialog v-model="visible" title="安装提示" width="80%" :show-close="false">
		<el-alert type="error" show-icon :closable="false">
			<template #title>
				如果出现安装失败的情况, 可以手动进行安装
				<a
					style="
						display: inline-block;
						text-decoration: underline;
						padding-left: 10px;
					"
					:href="url"
					target="_blank"
					>fnm下载</a
				>
			</template>
		</el-alert>

		<div class="code">
			<el-scrollbar ref="scrollRef" style="height: 300px">
				<code class="code-content">
					{{ result.statusText || "正在安装, 请稍等..." }}
					<br />
					{{ result.status ? result.stdout : result.stderr }}
				</code>
			</el-scrollbar>
		</div>
		<template #footer>
			<div class="dialog-footer">
				<el-button @click="setVisible(false)">取消</el-button>
				<!-- <el-button type="primary"> 下一步 </el-button> -->
			</div>
		</template>
	</el-dialog>
</template>

<style lang="scss" scoped>
.code {
	background-color: black;
	color: #fff;
	padding: 14px;
	height: 250px;
	border-radius: 4px;
	margin-top: 10px;
	code {
		word-break: keep-all;
	}
	.code-content {
		font-size: 12px;
		line-height: 30px;
		color: #fff;
		opacity: 0.7;
	}
}
</style>

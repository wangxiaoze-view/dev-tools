<script setup lang="ts">
import { ref } from "vue";
import { home_config } from "@/config";
import SvgIcon from "@/components/SvgIcon.vue";
import { useAppStore } from "@/stores/modules/app";
import installFnmDIalog from "./installFnmDIalog.vue";

const isExit = home_config.find(i => i.name === "fnm");
const url = ref(isExit ? isExit.web : "");
const installFnmDIalogRef = ref<InstanceType<typeof installFnmDIalog> | null>(
	null
);

const installFnmDialog = () => {
	installFnmDIalogRef.value?.setVisible(true);
};
const appStore = useAppStore();
const openLink = (link: string) => {
	appStore.openLink(link);
};
</script>

<template>
	<div class="app-container">
		<div class="icon">
			<SvgIcon iconName="empty" />
		</div>
		<p class="tip">
			检测到您还未安装
			<el-button type="primary" link @click="openLink(url)"> Fnm </el-button>,
			可点击下方按钮进行安装
		</p>
		<div class="handler">
			<el-button type="primary" @click="installFnmDialog">安装</el-button>
		</div>

		<installFnmDIalog ref="installFnmDIalogRef" />
	</div>
</template>

<style lang="scss" scoped>
.app-container {
	display: flex;
	gap: 8px;
	flex-direction: column;
	.icon {
		width: 80%;
		margin: 0 auto;
	}

	.tip {
		margin-top: -2em;
		text-align: center;
		color: var(--el-color-info-light-3);
	}
	a {
		display: inline-block;
	}

	.handler {
		text-align: center;
	}
}
</style>

<script setup lang="ts">
import { computed, markRaw } from "vue";
import { useNodeStore } from "@/stores/modules/node";
import { storeToRefs } from "pinia";
import isInstall from "./components/isInstall.vue";
import notInstall from "./components/notInstall.vue";

defineOptions({
	name: "NodePage",
});

const nodeStore = useNodeStore();
const { getIsInstallFnm } = storeToRefs(nodeStore);

const getComponent = computed(() => {
	return markRaw(getIsInstallFnm.value ? isInstall : notInstall);
});
</script>

<template>
	<div class="home-container">
		<div class="header">
			<Tag
				title="Node版本管理器"
				background-color="#e1f2f5"
				color="#004d5b"
				icon="ri-nodejs-line"
			/>

			<el-alert
				class="alert"
				title="注意：下面的配置设置的是全局环境！"
				type="warning"
				show-icon
				:closable="false"
			/>
		</div>

		<component :is="getComponent" />
	</div>
</template>
<style scoped lang="scss">
.header {
	margin-bottom: 30px;
	display: flex;
	align-items: center;
	justify-content: space-between;
	.alert {
		margin-left: 50px;
	}
}
</style>

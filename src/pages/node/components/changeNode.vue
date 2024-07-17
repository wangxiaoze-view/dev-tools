<script lang="ts" setup>
import { ref } from "vue";
import { IFnmStatus, useNodeStore } from "@/stores/modules/node";
import { ElMessageBox } from "element-plus";

const props = defineProps<{
	version: string;
}>();

// 状态
const nodeStore = useNodeStore();

const scrollRef = ref();

// 当前的版本
const version = ref(props.version);
const allVersion = ref([]);

// 初始化步骤条
const stepActive = ref(1);

const result = ref<Partial<IFnmStatus>>({});

// 弹窗显示
const visible = ref(false);
const setVisible = (show: boolean) => {
	visible.value = show;
	if (!show) {
		stepActive.value = 1;
	} else {
		initVersion();
	}
};

// 正在安装中, 禁止关闭弹窗
const handlerClose = (done: () => void) => {
	if (stepActive.value === 2) {
		ElMessageBox.confirm("Node还未安装成功, 请勿关闭?", "温馨提示", {
			confirmButtonText: "知道了",
			showClose: false,
			showCancelButton: false,
			type: "warning",
		})
			.then(() => {})
			.catch(() => {});
	} else {
		done();
		setVisible(false);
	}
};

// 下一步的操作
const nextStep = async () => {
	stepActive.value++;

	const { status, stderr, stdout } = await nodeStore.installNode(version.value);

	result.value = {
		status,
		stderr,
		stdout,
	};

	if (status) {
		stepActive.value++;
	}
};

// 初始化所有node版本
const initVersion = async () => {
	const result = await nodeStore.getAllNodeVersion();
	const installed = await nodeStore.getInstalledNode();
	allVersion.value = result.map((i: any) => ({
		label: i.version,
		value: i.version,
		disabled: installed.includes(i.version),
	}));
};

defineExpose({
	setVisible,
});
</script>

<template>
	<Teleport to="body">
		<el-drawer
			v-model="visible"
			class="drawer-header-0"
			title="切换Node版本"
			size="50%"
			:before-close="handlerClose"
		>
			<el-steps direction="horizontal" :active="stepActive">
				<el-step title="选择版本" />
				<el-step title="安装" />
				<el-step title="完成" />
			</el-steps>

			<div class="container">
				<template v-if="stepActive === 1">
					<div class="row">
						<span>版本列表</span>
						<div class="row-right">
							<!-- 这里使用虚拟化 -->
							<el-select-v2
								v-model="version"
								filterable
								:options="allVersion"
								placeholder="请先选择Node版本"
							>
								<template #default="{ item }">
									<div class="select-item">
										<span class="version">{{ item.label }}</span>
										<el-tag type="primary" v-if="item.disabled">已安装</el-tag>
									</div>
								</template>
							</el-select-v2>
						</div>
					</div>

					<div class="handler">
						<el-button type="primary" :disabled="!version" @click="nextStep"
							>下一步</el-button
						>
					</div>
				</template>

				<div class="code" v-if="stepActive !== 1">
					<el-scrollbar ref="scrollRef">
						<code class="code-content">
							<template v-if="stepActive === 2"> 正在安装, 请稍等... </template>
							<template v-if="stepActive === 3">
								<template v-if="result.status"
									>安装成功：{{ result.stdout }}</template
								>
								<template v-if="!result.status"
									>安装失敗：{{ result.stderr }}</template
								>
							</template>
						</code>
					</el-scrollbar>
				</div>
			</div>
		</el-drawer>
	</Teleport>
</template>

<style lang="scss" scoped>
.container {
	margin-top: 50px;

	.row {
		display: flex;
		align-items: center;
		justify-content: space-between;

		&-right {
			margin-left: 20px;
			flex: 1;
		}
	}

	.handler {
		margin-top: 30px;
		text-align: right;
	}

	.code {
		background-color: black;
		color: #fff;
		padding: 14px;
		height: 300px;
		border-radius: 4px;
		code {
			word-break: keep-all;
		}

		.title {
			margin-bottom: 14px;
			font-weight: bold;
		}

		.tip {
			color: #fff;
			opacity: 0.5;
			font-size: 12px;
			margin-bottom: 10px;
			font-weight: 600;
		}

		.content {
			font-size: 12px;
			color: var(--el-color-primary);
		}

		.code-content {
			font-size: 12px;
			line-height: 30px;
			color: #fff;
			opacity: 0.7;
		}
	}
}

.select-item {
	display: flex;
	align-items: center;
	justify-content: space-between;
}
</style>

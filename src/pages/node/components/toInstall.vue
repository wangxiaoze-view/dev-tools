<script lang="ts" setup>
import { ref, computed } from "vue";
import { useNodeStore } from "@/stores/modules/node";
import useLoading from "@/hooks/loading";
import { ElMessage } from "element-plus";

const emits = defineEmits(["emit-refresh"]);

// 状态
const nodeStore = useNodeStore();

const { loading, setLoading } = useLoading(false);

const dependencies = ref({});
// 弹窗显示
const visible = ref(false);
const setVisible = (show: boolean) => {
	visible.value = show;
	show && getInstallNpm();
};
// 查询某个包是否存在
const tableData = ref<{ name: string; version: string }[]>([]);
const npmStr = ref("");
const isDisabled = computed(() => !npmStr.value);
const findNpmIsExit = async () => {
	if (!npmStr.value) return;

	tableData.value = [];
	setLoading(true);

	const { status, stdout, stderr } = await nodeStore.getNpmView(npmStr.value);

	if (status) {
		const data = JSON.parse(stdout);
		if (Array.isArray(data)) {
			tableData.value = data.map(i => {
				return {
					name: npmStr.value,
					version: i,
				};
			});
		} else {
			tableData.value.push({
				name: npmStr.value,
				version: stdout.replace(/\s|\r|\n|\t|\"/g, ""),
			});
		}
	} else {
		ElMessage({
			message: stderr || "操作失败",
			type: "error",
		});
	}
	setLoading(false);
};

const findExitNpm = (row: { name: string; version: string }) => {
	const key = Object.keys(dependencies.value).find(
		i => row.name.indexOf(i) !== -1
	);
	const exit: { version: string } =
		dependencies.value[key as keyof typeof dependencies.value];
	if (exit) {
		return exit.version === row.version;
	}
	return false;
};

const installNpm = async (name: string) => {
	setLoading(true);
	const { status, stderr, stdout } = await nodeStore.installNpmView(name);
	ElMessage({
		message: status ? stdout || "安装成功" : stderr || "安装失败",
		type: status ? "success" : "error",
	});
	await getInstallNpm();
	setLoading(false);
	emits("emit-refresh");
};

const getInstallNpm = async () => {
	dependencies.value = await nodeStore.getInstallNpmView();
};

defineExpose({
	setVisible,
});
</script>

<template>
	<div
		v-loading.fullscreen="loading"
		element-loading-text="正在配置中, 请稍等..."
	>
		<el-drawer
			v-model="visible"
			class="drawer-header-0"
			title="安装Npm依赖"
			size="50%"
		>
			<el-alert
				title="查询对应的包名, 输入包名, 点击查询, 获取包名, 再点击安装"
				type="info"
				:closable="false"
				style="margin-bottom: 14px"
			/>

			<div class="row-search" style="margin-bottom: 14px">
				<el-input
					v-model="npmStr"
					style="width: 240px"
					placeholder="请输入包名称"
				/>
				<el-button type="primary" :disabled="isDisabled" @click="findNpmIsExit"
					>查询</el-button
				>
			</div>

			<el-table :data="tableData" border style="width: 100%" :resizable="false">
				<el-table-column prop="version" label="版本" />
				<el-table-column prop="address" label="操作" width="60">
					<template #default="{ row }">
						<el-tooltip
							effect="dark"
							:content="findExitNpm(row) ? '已安装' : '安装该依赖'"
							placement="top-start"
						>
							<el-button
								type="primary"
								circle
								:disabled="findExitNpm(row)"
								@click="installNpm(row.name)"
							>
								<template #icon><i class="ri-box-2-line"></i></template>
							</el-button>
						</el-tooltip>
					</template>
				</el-table-column>
			</el-table>
		</el-drawer>
	</div>
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

:deep(.now-version--row) {
	color: var(--el-color-primary);
	font-weight: 700;
	background-color: var(--el-color-primary-light-9);
}

.row-search {
	width: 100%;
	display: flex;
	align-items: center;
	justify-content: space-between;
}
</style>

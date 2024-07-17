<script lang="ts" setup>
import { ref } from "vue";
import { useNodeStore } from "@/stores/modules/node";
import useLoading from "@/hooks/loading";
import { ElMessage } from "element-plus";

const emits = defineEmits<{
	(e: "emit-refresh"): void;
}>();

// 状态
const nodeStore = useNodeStore();

const { loading, setLoading } = useLoading(false);

const data = ref([]);
const curtrentVersion = ref("");

// 弹窗显示
const visible = ref(false);
const setVisible = (show: boolean) => {
	visible.value = show;
	show && getInstalledNode();
};

// 是否为当前的node版本
// 是当前的行背景色, 操作按钮禁止操作
const tableRowClassName = ({ row }: any): string => {
	return row.version.indexOf(curtrentVersion.value) !== -1
		? "now-version--row"
		: "";
};

// 操作按钮
const useNode = async (version: string) => {
	setLoading(true);
	const { status, stdout, stderr } = await nodeStore.changeNode(version);
	getInstalledNode();
	setLoading(false);
	emits("emit-refresh");
	ElMessage({
		message: status ? stdout || "操作成功" : stderr || "操作失败",
		type: status ? "success" : "error",
	});
};
const deleteNode = async (version: string) => {
	setLoading(true);
	const { status, stdout, stderr } = await nodeStore.deleteNode(version);
	getInstalledNode();
	setLoading(false);
	emits("emit-refresh");
	ElMessage({
		message: status ? stdout || "删除成功" : stderr || "删除失败",
		type: status ? "success" : "error",
	});
};

const getInstalledNode = async () => {
	const result = await nodeStore.getAllNodeVersion();
	const installed = await nodeStore.getInstalledNode();
	curtrentVersion.value = await nodeStore.getNodeVersion();

	data.value = result.filter((i: any) => {
		return installed.includes(i.version);
	});
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
			title="管理Node版本"
			size="55%"
		>
			<el-alert
				title="带有背景色的为您当前的Node版本; 如果没有删除成功, 您可以手动再文件目录删除对应的目录文件!"
				type="info"
				:closable="false"
				style="margin-bottom: 14px"
			/>

			<el-table
				:data="data"
				border
				style="width: 100%"
				:resizable="false"
				:row-class-name="tableRowClassName"
			>
				<el-table-column prop="version" label="node" width="80">
				</el-table-column>
				<el-table-column prop="npm" label="npm" width="90" />
				<el-table-column prop="date" label="更新时间" />
				<el-table-column prop="address" label="操作" width="110">
					<template #default="scope">
						<el-tooltip
							effect="dark"
							:content="
								tableRowClassName(scope)
									? '已在使用该版本'
									: '切换使用该版本Node'
							"
							placement="top-start"
						>
							<el-button
								type="primary"
								circle
								:disabled="tableRowClassName(scope) ? true : false"
								@click="useNode(scope.row.version)"
							>
								<template #icon><i class="ri-mouse-line"></i></template>
							</el-button>
						</el-tooltip>

						<el-tooltip
							effect="dark"
							:content="
								tableRowClassName(scope) ? '当前版本禁止删除' : '删除该版本Node'
							"
							placement="top-start"
						>
							<el-button
								type="danger"
								circle
								:disabled="tableRowClassName(scope) ? true : false"
								@click="deleteNode(scope.row.version)"
							>
								<template #icon><i class="ri-delete-bin-line"></i></template>
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
</style>

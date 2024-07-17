<script setup lang="ts">
import { registries } from "@/config";
import { ref } from "vue";
import { useNodeStore } from "@/stores/modules/node";
import changeNode from "./changeNode.vue";
import installVsersions from "./installVersions.vue";
import toInstall from "./toInstall.vue";
import { ElMessage } from "element-plus";
import useLoading from "@/hooks/loading";

defineOptions({
	name: "NodePage",
});

const nodeStore = useNodeStore();

const { loading, setLoading } = useLoading(false);

const initRegistry = ref("");
const dependencies = ref<any[]>([]);

const version = ref("");

const drawerRef = ref<InstanceType<typeof changeNode> | null>(null);
const instllRef = ref<InstanceType<typeof installVsersions> | null>(null);
const toRef = ref<InstanceType<typeof toInstall> | null>(null);

const getNodeVersion = async () => {
	setLoading(true);

	const _version = await nodeStore.getNodeVersion();
	version.value = _version;
	initRegistry.value = await nodeStore.getRegistry();
	const npmView = await nodeStore.getInstallNpmView();

	dependencies.value = [];
	Object.keys(npmView).forEach(async item => {
		const version = await getNewVersion(item);

		dependencies.value.push({
			name: item,
			...(npmView[item] || {}),
			newVersion: version,
			isUpdate: isUpdate({
				...(npmView[item] || {}),
				newVersion: version,
			}),
		});
	});

	setLoading(false);
};
getNodeVersion();

const changeNodeVersion = () => {
	drawerRef.value?.setVisible(true);
};

const reviewInstallVersion = () => {
	instllRef.value?.setVisible(true);
};

const changeRegistry = async (value: string) => {
	const { status, stderr, stdout } = await nodeStore.setRegistry(value);
	ElMessage({
		message: status ? stdout || "操作成功" : stderr || "操作失败",
		type: status ? "success" : "error",
	});
};

const installDependencies = () => {
	toRef.value?.setVisible(true);
};

const getNewVersion = async (name: string) => {
	const version = await nodeStore.checkNpmViewVersion(name);
	return await version;
};

const isUpdate = (row: any) => {
	let newVersion = row.newVersion.replace(/\.|\n/g, "");
	let nowVersion = row.version.replace(/\.|\n/g, "");
	const max = Math.max(newVersion.length, nowVersion.length);

	if (newVersion.length < max) {
		newVersion = newVersion.padEnd(max, "0");
	}
	if (nowVersion.length < max) {
		nowVersion = nowVersion.padEnd(max, "0");
	}
	return Number(newVersion) > Number(nowVersion);
};

const reinstallNpm = async (row: any) => {
	setLoading(true);
	const { status, stderr, stdout } = await nodeStore.installNpmView(row.name);
	ElMessage({
		message: status ? stdout || "重装成功" : stderr || "重装失败",
		type: status ? "success" : "error",
	});
	getNodeVersion();
};

const deleteNpm = async (row: any) => {
	setLoading(true);
	const { status, stderr, stdout } = await nodeStore.deleteNpmView(row.name);
	ElMessage({
		message: status ? stdout || "删除成功" : stderr || "删除失败",
		type: status ? "success" : "error",
	});
	getNodeVersion();
};

const updateNpm = async (row: any) => {
	setLoading(true);
	const { status, stderr, stdout } = await nodeStore.updateNpmView(row.name);
	ElMessage({
		message: status ? stdout || "更换成功" : stderr || "更换失败",
		type: status ? "success" : "error",
	});
	getNodeVersion();
};
</script>

<template>
	<div
		class="container"
		v-loading.fullscreen="loading"
		element-loading-text="正在配置中, 请稍等..."
	>
		<div class="row">
			<div class="row-title">Node版本</div>

			<div class="row-right">
				<span :class="['exit-version', 'version']"> {{ version }} </span>
				<el-button type="primary" text @click="changeNodeVersion"
					>选择版本
				</el-button>
				<el-button type="info" text @click="reviewInstallVersion">
					查看安装的版本
				</el-button>
			</div>
		</div>

		<div class="row">
			<div class="row-title">切换镜像源</div>

			<div class="row-right">
				<el-select
					v-model="initRegistry"
					placeholder="请选择镜像源"
					@change="changeRegistry"
				>
					<el-option
						v-for="(item, key) in registries"
						:key="item.registry"
						:label="key + ' : ' + item.registry"
						:value="item.registry"
					/>
				</el-select>
			</div>
		</div>

		<div class="row">
			<div class="row-title">依赖管理</div>

			<div class="row-right">
				<el-button type="primary" @click="installDependencies"
					>安装依赖</el-button
				>
			</div>
		</div>

		<el-table :data="dependencies" border style="width: 100%">
			<el-table-column prop="name" label="依赖名称" />
			<el-table-column prop="nowVersion" label="依赖版本" width="120">
				<template #default="{ row }">
					{{ row.version || "-" }}
				</template>
			</el-table-column>
			<el-table-column prop="version" label="最新版本" width="120">
				<template #default="{ row }">
					<span :class="[row.isUpdate ? 'new_update' : '']">
						{{ row.newVersion || "-" }}
					</span>
					<i
						v-if="row.isUpdate"
						:class="[row.isUpdate ? 'new_update' : '', 'ri-arrow-up-line']"
					></i>
				</template>
			</el-table-column>
			<el-table-column prop="address" label="操作">
				<template #default="{ row }">
					<el-tooltip effect="dark" placement="top-start" content="重装依赖">
						<el-button type="warning" circle @click="reinstallNpm(row)">
							<template #icon><i class="ri-refresh-line"></i></template>
						</el-button>
					</el-tooltip>

					<el-tooltip effect="dark" placement="top-start" content="删除依赖">
						<el-button type="danger" circle @click="deleteNpm(row)">
							<template #icon><i class="ri-delete-bin-3-line"></i></template>
						</el-button>
					</el-tooltip>

					<template v-if="row.isUpdate">
						<el-tooltip
							effect="dark"
							placement="top-start"
							content="更新最新版本"
						>
							<el-button type="primary" circle @click="updateNpm(row)">
								<template #icon><i class="ri-arrow-up-fill"></i></template>
							</el-button>
						</el-tooltip>
					</template>
				</template>
			</el-table-column>
		</el-table>

		<changeNode ref="drawerRef" :version="version"></changeNode>
		<installVsersions ref="instllRef" @emit-refresh="getNodeVersion" />
		<toInstall ref="toRef" @emit-refresh="getNodeVersion"> </toInstall>
	</div>
</template>
<style scoped lang="scss">
.container {
	.row {
		margin-bottom: 20px;
		display: flex;
		align-items: center;
		&-title {
			width: 140px;
			font-weight: 600;
		}
		&-right {
			flex: 1;
			display: flex;
			align-items: center;
			justify-content: flex-start;

			.version {
				margin-right: 20px;
				font-weight: 700;
				color: var(--el-color-info-dark-2);

				&.exit-version {
					color: var(--el-color-primary);
				}
			}
		}
	}

	.new_update {
		font-size: 14px;
		font-weight: 600;
		color: var(--el-color-primary);
	}
}
</style>

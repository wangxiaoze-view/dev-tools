<script setup lang="ts">
import Tag from "@/components/Tag.vue";
import { ref, computed } from "vue";
import { useGitStore } from "@/stores/modules/git";
import useLoading from "@/hooks/loading";
import { ElMessage } from "element-plus";

defineOptions({
	name: "GitPage",
});

interface IResult {
	key: string;
	value: string;
}

const gitStore = useGitStore();

const { loading, setLoading } = useLoading(false);

const userName = ref("");
const userEmail = ref("");
const autocrlf = ref(false);

const isEmpty = computed(() => {
	return !userName.value || !userEmail.value;
});

const setGitConfig = async () => {
	setLoading(true);
	const { status, stderr, stdout } = await gitStore.setGitConfig(
		userName.value,
		userEmail.value,
		autocrlf.value + ""
	);
	console.log(status, stderr, stdout);
	setLoading(false);

	// useGitStore().setGitGlobalConfig({
	//   userName: userName.value,
	//   userEmail: userEmail.value,
	//   autoCrlf: autocrlf.value
	// })
};

const getFilterData = (data: IResult[], key: string) => {
	const exit: IResult | null = data.find(i => i.key === key) || null;
	return exit ? exit.value : "";
};

const getGitConfig = async () => {
	setLoading(true);
	const { status, stderr, stdout } = await gitStore.getGitConfig();
	if (status) {
		userName.value = getFilterData(stdout as unknown as IResult[], "user.name");
		userEmail.value = getFilterData(
			stdout as unknown as IResult[],
			"user.email"
		);
		autocrlf.value =
			getFilterData(stdout as unknown as IResult[], "core.autocrlf") === "true"
				? true
				: false;
	} else {
		ElMessage({
			message: stderr || "操作失败",
			type: "error",
		});
	}
	setLoading(false);
};
getGitConfig();
</script>

<template>
	<div
		v-loading.fullscreen="loading"
		class="home-container"
		element-loading-text="正在配置中，请稍等..."
	>
		<div class="header">
			<Tag
				title="Git用户配置"
				background-color="#ffe3df"
				color="#8d0905"
				icon="ri-git-pull-request-line"
			/>

			<el-alert
				class="alert"
				title="注意：下面的配置设置的是全局环境！"
				type="warning"
				show-icon
				:closable="false"
			/>

			<el-button type="primary" :disabled="isEmpty" @click="setGitConfig"
				>保存</el-button
			>
		</div>

		<div class="container">
			<div class="row">
				<div class="row-title">用户名</div>

				<div class="row-right">
					<el-input
						v-model.tirm="userName"
						style="width: 50%"
						maxlength="30"
						placeholder="请输入您的用户名"
					/>
				</div>
			</div>

			<div class="row">
				<div class="row-title">邮箱</div>

				<div class="row-right">
					<el-input
						v-model.tirm="userEmail"
						style="width: 50%"
						maxlength="30"
						placeholder="请输入您的邮箱"
					/>
				</div>
			</div>
			<div class="row">
				<div class="row-title">行尾字符差异</div>
				<div class="row-right">
					<el-switch v-model="autocrlf" />
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped lang="scss">
.header {
	margin-bottom: 30px;
	display: flex;
	align-items: center;
	justify-content: space-between;
	.alert {
		margin: 0 20px;
	}
}

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
			}
		}
	}
}
</style>

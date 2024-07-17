<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useThemeStore } from "@/stores/modules/theme";
import { useNodeStore } from "@/stores/modules/node";
import { markRaw, onMounted, ref } from "vue";
import Welcome from "@/pages/welcome/index.vue";
import Home from "@/pages/home/index.vue";
import Node from "@/pages/node/index.vue";
import Git from "@/pages/git/index.vue";
import Setting from "@/pages/setting/index.vue";
import useLoading from "@/hooks/loading";

defineOptions({ name: "AppPage" });

// 主题
const nodeStore = useNodeStore();
const themeStore = useThemeStore();
const { getSize } = storeToRefs(themeStore);
const { loading, setLoading } = useLoading(true);

// 组件配置
const navListConfig = [
	// 暂不开发
	{
		title: "欢迎使用",
		icon: "ri-hand-heart-line",
		component: markRaw(Welcome),
	},
	{ title: "首页", icon: "ri-home-6-line", component: markRaw(Home) },
	{ title: "Node管理", icon: "ri-nodejs-fill", component: markRaw(Node) },
	{ title: "Git配置", icon: "ri-git-merge-line", component: markRaw(Git) },
];

// 可收缩
const isShrink = ref(false);
// 初始化tab
const initTab = ref(0);
// 初始化组件
const componentName = ref(navListConfig[initTab.value].component);
// 动态切换tab
const changeTab = (index: number) => {
	initTab.value = index;
	componentName.value = navListConfig[index].component;
};
// 切换收缩
const changeShrink = () => (isShrink.value = !isShrink.value);
// 切换设置
const changeSetting = () => {
	initTab.value = -1;
	componentName.value = markRaw(Setting);
};

onMounted(() => {
	const quests = [themeStore.updateTheme(), nodeStore.checkIsInstallFnm()];
	Promise.all(quests)
		.then(() => {})
		.finally(() => {
			setTimeout(() => {
				setLoading(false);
			}, 1000);
		});
});
</script>

<template>
	<el-config-provider :button="{ autoInsertSpace: true }" :size="getSize">
		<div
			v-loading.fullscreen="loading"
			:class="['app-container', isShrink ? 'shrink-app--container' : '']"
			element-loading-text="正在配置中，请稍等..."
		>
			<div class="app-container--left">
				<!-- 左侧菜单 -->
				<div
					v-for="(item, index) in navListConfig"
					:key="index"
					:class="[index === initTab ? 'active' : '', 'menu-item']"
					@click="changeTab(index)"
				>
					<i :class="[item.icon, 'tab-icon']"></i>
					<span class="tab-title">{{ item.title }}</span>
				</div>

				<!-- 底部操作 -->
				<div class="bottom-handler">
					<div class="handler-menu" @click="changeShrink">
						<i
							:class="[
								!isShrink ? 'ri-menu-fold-fill' : 'ri-menu-unfold-fill',
								'shrink-icon',
							]"
						></i>
						<span class="shrink-title">收起侧边栏</span>
					</div>
					<i
						:class="[
							initTab === -1 ? 'set-active' : '',
							'set-icon ri-list-settings-line',
						]"
						@click="changeSetting"
					></i>
				</div>
			</div>

			<!-- 右侧内容 -->
			<div class="app-container--right">
				<el-scrollbar height="100%">
					<div class="pad-container">
						<KeepAlive>
							<component :is="componentName"></component>
						</KeepAlive>
					</div>
				</el-scrollbar>
			</div>
		</div>
	</el-config-provider>
</template>

<style scoped></style>

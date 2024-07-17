<script setup lang="ts">
import { useAppStore } from "@/stores/modules/app";
defineOptions({
	name: "HomePage",
});

const appStore = useAppStore();
const config = [
	{ link: "https://github.com/wangxiaoze-view", name: "Github" },
	{ link: "https://www.wangzevw.com/", name: "知识库" },
	{ link: "https://docs.wangzevw.com/", name: "项目文档" },
];

const openLink = (link: string) => {
	appStore.openLink(link);
};
</script>

<template>
	<div class="container">
		<h1>欢迎使用</h1>
		<p class="desc">
			该软件是对于前端工具的可视化面板, 包含Node,
			Git等工具的可视化操作，仅需安装此软件就可以快速使用。再也不用担心因为环境配置的烦恼了!
		</p>
		<ul>
			<li>😃 <b>首页:</b>涵盖前端常见的开发工具</li>
			<li>😍 <b>Node管理:</b>全局可视化操作Node, 更换源，版本，更新包等</li>
			<li>🫡 <b>Git配置</b>全局的Git配置</li>
		</ul>
		<p class="author">
			作者： 王小泽
			<el-button
				type="primary"
				link
				v-for="(item, index) in config"
				:key="index"
				@click="openLink(item.link)"
			>
				{{ item.name }}
			</el-button>
		</p>

		<p class="pic">
			<img src="https://file.wangzevw.com/images/wx_pay.png" alt="" />
			<img src="https://file.wangzevw.com/images/zfb_pay.png" alt="" />
		</p>
	</div>
</template>

<style scoped lang="scss">
.container {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	padding: 4em;
	height: calc(100vh - var(--base-header-top) * 2);
	h1 {
		font-weight: bold;
		letter-spacing: 2px;
	}

	.desc {
		margin: var(--base-header-top) 0;
		letter-spacing: 1px;
	}

	ul {
		margin: 0;
		padding: 0;
		display: flex;
		width: 100%;
		gap: 6px;
		flex-direction: column;
		justify-content: flex-start;
		text-align: left;
		b {
			font-weight: 600;
			color: #000;
			padding-right: 6px;
		}
	}

	.author {
		margin: 14px 0;
		text-align: left;
		width: 100%;
	}

	a {
		padding: 0 6px;
		display: inline-block;
	}

	.pic {
		width: 100%;
		display: flex;
		justify-content: space-around;
		img {
			width: 140px;
			height: 140px;
		}
	}
}
</style>

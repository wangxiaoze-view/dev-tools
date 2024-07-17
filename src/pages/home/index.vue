<script setup lang="ts">
import Tag from "@/components/Tag.vue";
import SvgIcon from "@/components/SvgIcon.vue";
import { useAppStore } from "@/stores/modules/app";
import { home_config } from "@/config";
defineOptions({
	name: "HomePage",
});
const appStore = useAppStore();
const openLink = (link: string) => {
	appStore.openLink(link);
};
</script>

<template>
	<div class="home-container">
		<div class="header">
			<Tag
				title="前端开发必备工具"
				background-color="#d2e0ed"
				color="#004b83"
				icon="ri-hammer-line"
			/>
		</div>

		<el-row :gutter="20">
			<el-col v-for="(item, index) in home_config" :key="index" :span="8">
				<div class="card">
					<div class="card-header">
						<SvgIcon :icon-name="item.name" />

						<el-button type="primary" link @click="openLink(item.web)">
							官网
						</el-button>
					</div>
					<span class="card-name">{{ item.name }}</span>
					<div class="card-desc">{{ item.desc }}</div>
					<div class="card-handerl"></div>
				</div>
			</el-col>
		</el-row>
	</div>
</template>

<style scoped lang="scss">
.header {
	margin-bottom: 30px;
}

.card {
	&-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	&-name {
		font-weight: 700;
		margin: 10px 0;
	}
	&-desc {
		margin-bottom: 20px;
		font-size: 14px;
		color: var(--color-text-light);
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}
}
</style>

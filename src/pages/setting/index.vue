<script setup lang="ts">
import Tag from "@/components/Tag.vue";
import { ref } from "vue";
import { useThemeStore, ThemeColor, Size } from "@/stores/modules/theme";
import { storeToRefs } from "pinia";

const themeStore = useThemeStore();

const { getTheme, getSize } = storeToRefs(themeStore);

const colors: { key: ThemeColor; color: string }[] = [
	{ key: "blue", color: "#409eff" },
	{ key: "green", color: "#41b584" },
	{ key: "purple", color: "#8b77ea" },
	{ key: "red", color: "#cd4432" },
];

const sizeList = [
	{ label: "大", size: "large" },
	{ label: "中", size: "default" },
	{ label: "小", size: "small" },
];

const size = ref(getSize.value);

const changeTheme = (themeKey: ThemeColor) => {
	themeStore.setTheme(themeKey);
};
const changeSize = (size: Size) => {
	themeStore.setSize(size);
};
</script>

<template>
	<div class="home-container">
		<div class="header">
			<Tag
				title="可以根据以下配置定制您的主题化设置！"
				background-color="#e9e2f8"
				color="#361c8a"
				icon="ri-t-shirt-line"
			/>

			<!-- <el-button type="primary">保存</el-button> -->
		</div>

		<div class="container">
			<div class="row">
				<div class="row-title">颜色</div>

				<div class="row-right">
					<div
						v-for="(item, index) in colors"
						:key="index"
						:class="[getTheme === item.key ? 'active' : '', 'color']"
						:style="{ backgroundColor: item.color }"
						@click="changeTheme(item.key)"
					></div>
				</div>
			</div>

			<div class="row">
				<div class="row-title">尺寸</div>

				<div class="row-right">
					<el-select
						v-model="size"
						placeholder="请选择UI尺寸"
						style="width: 240px"
						@change="changeSize"
					>
						<el-option
							v-for="item in sizeList"
							:key="item.size"
							:label="item.label"
							:value="item.size"
						/>
					</el-select>
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

			.color {
				width: 20px;
				height: 20px;
				border-radius: 4px;
				margin-right: 10px;
				cursor: pointer;
				border: 2px solid transparent;
				transition: all cubic-bezier(0.075, 0.82, 0.165, 1) 0.2s;

				&.active {
					border-color: pink;
				}
			}
		}
	}
}
</style>

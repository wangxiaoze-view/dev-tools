<script setup lang="ts">
// eslint-disable-next-line vue/valid-define-props
defineProps<{
	backgroundColor?: string;
	color?: string;
	title?: string;
	icon?: string;
}>();
</script>

<template>
	<el-tag
		key="primary"
		type="primary"
		effect="dark"
		round
		class="tag"
		:style="{ backgroundColor: backgroundColor, color }"
	>
		{{ title }}

		<i v-if="icon" :class="[icon, 'icon']"></i>
	</el-tag>
</template>

<style scoped lang="scss">
.tag {
	border: none;
	.icon {
		margin-left: 4px;
	}
}
</style>

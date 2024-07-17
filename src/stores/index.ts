import type { App } from "vue";
import { createPinia } from "pinia";

const pinia = createPinia();

export const setupStore = (app: App<Element>) => {
	app.use(pinia);
};

export default pinia;

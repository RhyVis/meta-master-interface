import { createRouter, createWebHistory } from 'vue-router';

import BaseLayout from '@/layout/BaseLayout.vue';
import routes from '@/router/routes.ts';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'root',
      component: BaseLayout,
      children: routes,
    },
  ],
});

export default router;

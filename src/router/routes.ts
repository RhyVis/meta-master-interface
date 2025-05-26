import type { RouteRecordRaw } from 'vue-router';

type RouteRecordCompose = RouteRecordRaw & {
  meta: {
    title: string;
    icon: string;
  };
};

const routes: RouteRecordCompose[] = [
  {
    path: '',
    name: 'dashboard',
    component: () => import('@/pages/dashboard/index.vue'),
    meta: {
      title: 'Dashboard',
      icon: 'home',
    },
  },
];

export default routes;

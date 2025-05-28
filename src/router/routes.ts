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
      title: '管理',
      icon: 'home',
    },
  },
  {
    path: 'settings',
    name: 'settings',
    component: () => import('@/pages/settings/index.vue'),
    meta: {
      title: '设置',
      icon: 'settings',
    },
  },
];

export default routes;

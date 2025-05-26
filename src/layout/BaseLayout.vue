<script lang="ts" setup>
import { useToggle } from '@vueuse/core';
import BaseSideDrawer from '@/layout/comp/BaseSideDrawer.vue';

const [leftDrawerOpen, setLeftDrawerOpen] = useToggle(false);
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <q-header class="bg-white text-subtitle2 text-black" elevated>
      <q-toolbar>
        <q-btn dense flat icon="menu" round @click="setLeftDrawerOpen()" />

        <q-toolbar-title class="r-no-sel text-center">
          <span class="text-h6 text-bold q-mr-xl">Metadata Master Interface</span>
        </q-toolbar-title>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" bordered overlay side="left">
      <BaseSideDrawer />
    </q-drawer>

    <q-page-container>
      <RouterView />
    </q-page-container>
  </q-layout>
</template>

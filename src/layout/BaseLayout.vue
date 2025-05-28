<script lang="ts" setup>
import { useToggle } from '@vueuse/core';
import BaseSideDrawer from '@/layout/comp/BaseSideDrawer.vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const [menuOpen, toggleMenuOpen] = useToggle(false);

const appWindow = getCurrentWindow();
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <q-header class="bg-white text-subtitle2 text-black r-drag-region" id="base-header">
      <q-toolbar class="flex-center">
        <q-btn
          class="r-drag-region-disabled"
          :icon="menuOpen ? 'menu_open' : 'menu'"
          dense
          flat
          round
          @click="toggleMenuOpen()"
        />

        <q-toolbar-title class="r-no-sel flex">
          <span class="text-h6 text-bold">Metadata Master Interface</span>
        </q-toolbar-title>

        <q-btn-group class="r-drag-region-disabled" flat>
          <q-btn
            dense
            flat
            icon="fa-solid fa-window-minimize"
            round
            @click="appWindow.minimize()"
          />
          <q-btn
            dense
            flat
            icon="fa-solid fa-window-maximize"
            round
            @click="appWindow.toggleMaximize()"
          />
          <q-btn dense flat icon="fa-solid fa-xmark" round @click="appWindow.close()" />
        </q-btn-group>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="menuOpen" bordered overlay side="left">
      <BaseSideDrawer />
    </q-drawer>

    <q-page-container>
      <RouterView />
    </q-page-container>
  </q-layout>
</template>

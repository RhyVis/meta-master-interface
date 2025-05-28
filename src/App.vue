<script lang="ts" setup>
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { onMounted } from 'vue';

onMounted(() => useLibraryStore().reload());
</script>

<template>
  <RouterView />
</template>

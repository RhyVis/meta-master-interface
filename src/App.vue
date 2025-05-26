<script lang="ts" setup>
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { onMounted } from 'vue';

const library = useLibraryStore();

onMounted(() => library.reload());
</script>

<template>
  <RouterView />
</template>

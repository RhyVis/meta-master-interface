<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { onMounted } from 'vue';
import { useQuasar } from 'quasar';

const store = useGlobalStore();
const { develop } = storeToRefs(store);
const { notify } = useQuasar();
const { clear: libClear, export: libExport, import: libImport } = useLibraryStore();

onMounted(() => {
  store.$tauri.start().catch((e) => {
    console.error('Failed to start Tauri:', e);
    notify({
      type: 'negative',
      message: '无法启动同步',
      color: 'negative',
      position: 'top',
    });
  });
});
</script>

<template>
  <q-page class="r-no-sel" padding>
    <q-list bordered padding>
      <q-item-label header>数据库</q-item-label>

      <q-item v-ripple clickable @click="libExport">
        <q-item-section side>
          <q-icon name="output" />
        </q-item-section>
        <q-item-section>
          <q-item-label>导出数据库</q-item-label>
          <q-item-label caption>导出数据库为 library.json 文件</q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="libImport">
        <q-item-section side>
          <q-icon name="exit_to_app" />
        </q-item-section>
        <q-item-section>
          <q-item-label>导入数据库</q-item-label>
          <q-item-label caption>从 library.json 文件导入数据库</q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="libClear">
        <q-item-section side>
          <q-icon name="clear_all" />
        </q-item-section>
        <q-item-section>
          <q-item-label>清空数据库</q-item-label>
          <q-item-label caption>清空数据库中所有记录，会自动执行一次导出</q-item-label>
        </q-item-section>
      </q-item>

      <q-separator spaced />
      <q-item-label header>开发设置</q-item-label>
      <q-item v-ripple>
        <q-item-section side>
          <q-icon name="developer_mode" />
        </q-item-section>
        <q-item-section>
          <q-item-label>开发模式</q-item-label>
          <q-item-label caption>启用后将显示开发相关的功能和设置</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-toggle v-model="develop" />
        </q-item-section>
      </q-item>
    </q-list>
  </q-page>
</template>

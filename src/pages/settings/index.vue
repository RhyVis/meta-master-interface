<script lang="ts" setup>
import { command_library_export, command_library_import } from '@/api/command.ts';
import { useQuasar } from 'quasar';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';

const {
  notify,
  loading: { show, hide },
} = useQuasar();
const { develop } = storeToRefs(useGlobalStore());

const handleExport = async () => {
  try {
    show();
    await command_library_export();
    notify({
      message: '导出成功',
      color: 'positive',
      position: 'top',
      icon: 'check_circle',
    });
  } catch (e) {
    console.error(e);
    notify({
      message: '导出失败',
      color: 'warning',
      position: 'top',
      icon: 'warning',
    });
  } finally {
    hide();
  }
};

const handleImport = async () => {
  try {
    show();
    await command_library_import();
    notify({
      message: '导入成功',
      color: 'positive',
      position: 'top',
      icon: 'check_circle',
    });
  } catch (e) {
    console.error(e);
    notify({
      message: '导入失败',
      color: 'warning',
      position: 'top',
      icon: 'warning',
    });
  } finally {
    hide();
  }
};
</script>

<template>
  <q-page class="r-no-sel" padding>
    <q-list bordered padding>
      <q-item-label header>导入/导出数据库</q-item-label>

      <q-item v-ripple clickable @click="handleExport">
        <q-item-section side>
          <q-icon name="output" />
        </q-item-section>
        <q-item-section>
          <q-item-label>导出数据库</q-item-label>
          <q-item-label caption>导出数据库为 library.json 文件</q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="handleImport">
        <q-item-section side>
          <q-icon name="exit_to_app" />
        </q-item-section>
        <q-item-section>
          <q-item-label>导入数据库</q-item-label>
          <q-item-label caption>从 library.json 文件导入数据库</q-item-label>
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

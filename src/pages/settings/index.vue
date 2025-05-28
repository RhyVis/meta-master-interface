<script lang="ts" setup>
import { command_library_export, command_library_import } from '@/api/command.ts';
import { useQuasar } from 'quasar';

const {
  notify,
  loading: { show, hide },
} = useQuasar();

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
  <div class="q-pa-xl flex flex-center">
    <div class="q-pa-lg q-mx-auto">
      <div class="text-h5 text-center q-mb-md r-no-sel">导入/导出数据库</div>
      <q-separator spaced />
      <div class="row justify-center q-gutter-md q-mb-md">
        <q-btn-group push>
          <q-btn icon="output" label="导出数据库" push @click="handleExport" />
          <q-btn icon="exit_to_app" label="导入数据库" push @click="handleImport" />
        </q-btn-group>
      </div>
      <div class="text-caption text-grey text-center r-no-sel">
        导出当前数据库进行备份，或导入数据库进行恢复。
      </div>
    </div>
  </div>
</template>

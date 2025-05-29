import type { Metadata, MetadataOptional } from '@/api/types.ts';

import { defineStore } from 'pinia';
import { Loading, Notify } from 'quasar';

import {
  command_library_clear,
  command_library_export,
  command_library_import,
  command_metadata_deploy,
  command_metadata_deploy_off,
  command_metadata_get_all,
  command_metadata_remove,
  command_metadata_update,
} from '@/api/command.ts';

interface LibraryState {
  data: Metadata[];
}

export const useLibraryStore = defineStore('library', {
  state: (): LibraryState => ({
    data: [],
  }),
  actions: {
    async reload() {
      try {
        Loading.show({
          message: '加载数据中……',
        });
        this.data = await command_metadata_get_all();
        Notify.create({
          message: '加载数据成功',
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        console.error(e);
        this.data = [];
        Notify.create({
          message: '加载数据失败',
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async update(opt: MetadataOptional) {
      try {
        if (!!opt.id) {
          Loading.show({
            message: `更新条目 ${opt.id}...`,
          });
        } else {
          Loading.show({
            message: '创建新条目...',
          });
        }
        await command_metadata_update(opt);
        try {
          this.data = await command_metadata_get_all();
          Notify.create({
            message: `更新成功`,
            color: 'positive',
            position: 'top',
            icon: 'cloud_done',
          });
        } catch (e) {
          console.error(e);
          Notify.create({
            message: '更新成功，但获取数据失败',
            caption: e as string,
            color: 'negative',
            position: 'top',
            icon: 'error',
          });
        }
      } catch (e) {
        console.error(e);
        Notify.create({
          message: `更新失败`,
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async remove(key: string) {
      try {
        Loading.show({
          message: '正在删除...',
        });
        await command_metadata_remove(key);
        this.data = await command_metadata_get_all();
      } catch (e) {
        console.error(e);
        Notify.create({
          message: `删除失败: ${key}`,
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async deploy(key: string, target: string) {
      if (!target) {
        Notify.create({
          message: '部署目标不能为空',
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
        return;
      }

      try {
        Loading.show({
          message: `正在部署 ${key} 到 ${target}...`,
        });
        await command_metadata_deploy(key, target);
        this.data = await command_metadata_get_all();
        Notify.create({
          message: `部署成功`,
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        Notify.create({
          message: `部署失败: ${key} 到 ${target}`,
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async deployOff(key: string) {
      try {
        Loading.show({
          message: `正在取消部署 ${key}...`,
        });
        await command_metadata_deploy_off(key);
        this.data = await command_metadata_get_all();
        Notify.create({
          message: `取消部署成功`,
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        console.error(e);
        Notify.create({
          message: `取消部署失败: ${key}`,
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async clear() {
      try {
        Loading.show({
          message: '正在自动导出库...',
        });
        await command_library_export();
        Loading.hide();
        Loading.show({
          message: '正在清空库...',
        });
        await command_library_clear();
        this.data = [];
        Notify.create({
          message: '清空库成功',
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        console.error(e);
        Notify.create({
          message: '清空库失败',
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async export() {
      try {
        Loading.show();
        await command_library_export();
        Notify.create({
          message: '导出库成功',
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        console.error(e);
        Notify.create({
          message: '导出库失败',
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
    async import() {
      try {
        Loading.show({
          message: '正在导入库...',
        });
        await command_library_import();
        this.data = await command_metadata_get_all();
        Notify.create({
          message: '导入库成功',
          color: 'positive',
          position: 'top',
          icon: 'cloud_done',
        });
      } catch (e) {
        console.error(e);
        Notify.create({
          message: '导入库失败',
          caption: e as string,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
      } finally {
        Loading.hide();
      }
    },
  },
});

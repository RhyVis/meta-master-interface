import { Notify } from 'quasar';

import { command_util_resolve_root } from '@/api/command.ts';
import { open } from '@tauri-apps/plugin-dialog';
import { openPath } from '@tauri-apps/plugin-opener';

export async function openSelectFile(): Promise<string | null> {
  try {
    return await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: '所有文件',
          extensions: ['*'],
        },
      ],
    });
  } catch (e) {
    console.error(e);
    Notify.create({
      type: 'negative',
      message: '选择文件失败',
      caption: e as string,
    });
    return Promise.reject(e);
  }
}

export async function openSelectFolder(): Promise<string | null> {
  try {
    return await open({
      multiple: false,
      directory: true,
    });
  } catch (e) {
    console.error(e);
    Notify.create({
      type: 'negative',
      message: '选择文件夹失败',
      caption: e as string,
    });
    return Promise.reject(e);
  }
}

export async function openPathTo(path?: string) {
  if (!path) {
    Notify.create({
      type: 'negative',
      message: '路径不能为空',
      position: 'top',
      icon: 'warning',
    });
    return;
  } else {
    try {
      await openPath(await command_util_resolve_root(path));
    } catch (e) {
      console.error(e);
      Notify.create({
        type: 'negative',
        message: '打开路径失败',
        caption: e as string,
        position: 'top',
        icon: 'warning',
      });
    }
  }
}

import type { QForm } from 'quasar';
import type { Ref } from 'vue';
import type { ArchiveInfo, DistributionPlatform, MetadataOptional } from '@/api/types.ts';

import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref, watch } from 'vue';

import { ArchiveType, ContentType, PlatformType } from '@/api/types.ts';
import { removeEmptyStrings } from '@/api/util.ts';
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { get, set } from '@vueuse/core';

export const useUpdate = (id: Ref<string>, formRef: Ref<QForm>) => {
  const { notify } = useQuasar();
  const library = useLibraryStore();
  const current = computed<MetadataOptional | undefined>(() =>
    library.data.find((val) => val.id === id.value),
  );

  // true if editing an existing item, false if creating a new one
  const mode = computed(() => !!current.value);
  const edit = ref<MetadataOptional>({
    title: '',
    alias: [],
    tags: [],
    content_type: ContentType.Other,
    platform: PlatformType.Unknown,
    archive_info: ArchiveType.Unset,
  });

  const reset = () => {
    set(edit, {
      title: '',
      alias: [],
      tags: [],
      content_type: ContentType.Other,
      platform: PlatformType.Unknown,
      archive_info: ArchiveType.Unset,
    });
    set(cAlias, '');
    set(cTag, '');
    set(cPlatformType, PlatformType.Unknown);
    set(cPlatformID, '');
    set(cPlatformName, '');
  };

  const cAlias = ref('');
  const addAlias = () => {
    if (!edit.value.alias) {
      edit.value.alias = [];
    }

    const trimInput = cAlias.value.trim();
    if (!trimInput) {
      notify({
        message: '添加别名不能为空',
        color: 'warning',
        position: 'top',
        icon: 'warning',
      });
      return;
    } else if (edit.value.alias.includes(trimInput)) {
      if (edit.value.alias.includes(trimInput)) {
        notify({
          message: `别名 '${trimInput}' 已存在`,
          color: 'warning',
          position: 'top',
          icon: 'warning',
        });
        return;
      }
    } else {
      edit.value.alias.push(trimInput);
      cAlias.value = '';
    }
  };
  const delAlias = (idx: number) => {
    edit.value.alias?.splice(idx, 1);
  };

  const cTag = ref('');
  const addTag = () => {
    if (!edit.value.tags) {
      edit.value.tags = [];
    }

    const trimInput = cTag.value.trim();
    if (!trimInput) {
      notify({
        message: '添加标签不能为空',
        color: 'warning',
        position: 'top',
        icon: 'warning',
      });
      return;
    }

    // Split by whitespace, commas and seps filter out empty strings
    const tags = trimInput.split(/[\s，,；;|]+/).filter(Boolean);
    const duplicatedTags = [];

    let added = false;
    for (const tag of tags) {
      if (!edit.value.tags.includes(tag)) {
        edit.value.tags.push(tag);
        added = true;
      } else {
        duplicatedTags.push(tag);
      }
    }

    if (added) {
      cTag.value = '';
    }
    if (duplicatedTags.length > 0) {
      notify({
        message: `标签 '${duplicatedTags.join(', ')}' 已存在`,
        color: 'warning',
        position: 'top',
        icon: 'warning',
      });
    }
  };
  const delTag = (idx: number) => {
    edit.value.tags?.splice(idx, 1);
  };

  const cPlatformType = ref<PlatformType>(PlatformType.Unknown);
  const cPlatformID = ref('');
  const cPlatformName = ref('');
  const mapPlatform = (): DistributionPlatform => {
    switch (cPlatformType.value) {
      case PlatformType.Unknown: {
        return PlatformType.Unknown;
      }
      case PlatformType.Steam: {
        return { Steam: { id: cPlatformID.value } };
      }
      case PlatformType.DLSite: {
        return { DLSite: { id: cPlatformID.value } };
      }
      case PlatformType.Other: {
        return { Other: { name: cPlatformName.value, id: cPlatformID.value ?? undefined } };
      }
    }
  };

  const cArchiveType = ref<ArchiveType>(ArchiveType.Unset);
  const cArchivePath = ref('');
  const cArchivePassword = ref('');
  const mapArchiveInfo = (): ArchiveInfo => {
    switch (cArchiveType.value) {
      case ArchiveType.Unset:
        return ArchiveType.Unset;
      case ArchiveType.ArchiveFile:
        return cArchivePassword.value
          ? {
              ArchiveFile: {
                path: cArchivePath.value,
                password: cArchivePassword.value,
              },
            }
          : {
              ArchiveFile: {
                path: cArchivePath.value,
              },
            };
      case ArchiveType.CommonFile:
        return { CommonFile: { path: cArchivePath.value } };
      case ArchiveType.Directory:
        return { Directory: { path: cArchivePath.value } };
    }
  };

  const doUpdate = async (): Promise<boolean> => {
    if (await formRef.value.validate()) {
      edit.value.platform = mapPlatform();
      edit.value.archive_info = mapArchiveInfo();

      const editCopy = removeEmptyStrings(cloneDeep(get(edit)));

      if (!mode.value && cArchiveType.value != ArchiveType.Unset) {
        // If we are creating a new item, we need to set the creation flag
        editCopy['flag_create_archive'] = true;
      }

      await library.update(editCopy);
      reset();
      return true;
    } else {
      notify({
        message: '请检查输入',
        color: 'warning',
        position: 'top',
        icon: 'warning',
      });
      return false;
    }
  };

  watch(
    () => id.value,
    () => {
      if (current.value) {
        console.log('Cloning current metadata for editing:', current.value);
        const clone = cloneDeep(current.value);
        console.log(clone);

        // Reset the form
        console.log('Resetting form for editing');
        reset();

        console.log('Mapping current metadata to edit form: platform', clone.platform);
        if (clone.platform === PlatformType.Unknown) {
          set(cPlatformType, PlatformType.Unknown);
          set(cPlatformName, '');
          set(cPlatformID, '');
        } else if (PlatformType.Steam in clone.platform) {
          set(cPlatformType, PlatformType.Steam);
          set(cPlatformID, clone.platform.Steam.id);
          set(cPlatformName, '');
        } else if (PlatformType.DLSite in clone.platform) {
          set(cPlatformType, PlatformType.DLSite);
          set(cPlatformID, clone.platform.DLSite.id);
          set(cPlatformName, '');
        } else if (PlatformType.Other in clone.platform) {
          set(cPlatformType, PlatformType.Other);
          set(cPlatformID, clone.platform.Other.id ?? '');
          set(cPlatformName, clone.platform.Other.name ?? '');
        }

        console.log('Mapping current metadata to edit form: archive_type', clone.archive_info);
        if (clone.archive_info === ArchiveType.Unset) {
          console.log(0);
          set(cArchiveType, ArchiveType.Unset);
          set(cArchivePath, '');
          set(cArchivePassword, '');
        } else if (ArchiveType.ArchiveFile in clone.archive_info) {
          console.log(1);
          set(cArchiveType, ArchiveType.ArchiveFile);
          set(cArchivePath, clone.archive_info.ArchiveFile.path);
          set(cArchivePassword, clone.archive_info.ArchiveFile.password ?? '');
        } else if (ArchiveType.CommonFile in clone.archive_info) {
          console.log(2);
          set(cArchiveType, ArchiveType.CommonFile);
          set(cArchivePath, clone.archive_info.CommonFile.path);
          set(cArchivePassword, '');
        } else if (ArchiveType.Directory in clone.archive_info) {
          console.log(3);
          set(cArchiveType, ArchiveType.Directory);
          set(cArchivePath, clone.archive_info.Directory.path);
          set(cArchivePassword, '');
        }

        Object.assign(edit.value, clone);
      } else {
        reset();
      }
    },
    { immediate: true },
  );

  return {
    // Universal methods
    edit,
    mode,
    doUpdate,
    // Helper ALIAS methods
    cAlias,
    addAlias,
    delAlias,
    // Helper TAG methods
    cTag,
    addTag,
    delTag,
    // Helper PLATFORM methods
    cPlatformType,
    cPlatformID,
    cPlatformName,
    // Helper ARCHIVE methods
    cArchiveType,
    cArchivePath,
    cArchivePassword,
  };
};

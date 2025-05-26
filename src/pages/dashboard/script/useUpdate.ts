import type { QForm } from 'quasar';
import type { Ref } from 'vue';
import type { DistributionPlatform, MetadataOptional } from '@/api/types.ts';

import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref, watch } from 'vue';

import { PlatformType } from '@/api/types.ts';
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { get, set } from '@vueuse/core';

export const useUpdate = (index: Ref<number>, formRef: Ref<QForm>) => {
  const { notify } = useQuasar();
  const library = useLibraryStore();
  const current = computed<MetadataOptional | undefined>(() => {
    const exist = library.data[index.value];
    if (exist) {
      const clone = cloneDeep(exist);
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
      return clone;
    } else {
      return undefined;
    }
  });

  // true if editing an existing item, false if creating a new one
  const mode = computed(() => !!current.value);
  const edit = ref<MetadataOptional>({
    title: '',
    alias: [],
    tags: [],
    platform: PlatformType.Unknown,
  });

  const reset = () => {
    set(edit, { title: '', alias: [], tags: [], platform: PlatformType.Unknown });
    set(cAlias, '');
    set(cTag, '');
    set(cPlatformType, PlatformType.Unknown);
    set(cPlatformID, '');
    set(cPlatformName, '');
  };

  const cAlias = ref('');
  const addAlias = () => {
    const trimInput = cAlias.value.trim();
    if (!edit.value.alias) {
      edit.value.alias = [];
    }
    if (!trimInput) {
      notify({
        message: '别名不能为空',
        color: 'negative',
        position: 'top',
        icon: 'error',
      });
      return;
    } else if (edit.value.alias.includes(trimInput)) {
      if (edit.value.alias.includes(trimInput)) {
        notify({
          message: `别名 '${trimInput}' 已存在`,
          color: 'negative',
          position: 'top',
          icon: 'error',
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
    const trimInput = cTag.value.trim();
    if (!edit.value.tags) {
      edit.value.tags = [];
    }
    if (!trimInput) {
      notify({
        message: '别名不能为空',
        color: 'negative',
        position: 'top',
        icon: 'error',
      });
      return;
    } else if (edit.value.tags.includes(trimInput)) {
      if (edit.value.tags.includes(trimInput)) {
        notify({
          message: `别名 '${trimInput}' 已存在`,
          color: 'negative',
          position: 'top',
          icon: 'error',
        });
        return;
      }
    } else {
      edit.value.tags.push(trimInput);
      cTag.value = '';
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

  const doUpdate = async (): Promise<boolean> => {
    if (await formRef.value.validate()) {
      edit.value.platform = mapPlatform();
      await library.update(get(edit));
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
    () => index.value,
    () => {
      if (current.value) {
        Object.assign(edit.value, current.value);
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
  };
};

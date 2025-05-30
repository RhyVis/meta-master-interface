import type { Metadata } from '@/api/types.ts';

import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';

import { PlatformType } from '@/api/types.ts';
import { useLibraryStore } from '@/pages/dashboard/store.ts';

export const useTable = () => {
  const { data } = storeToRefs(useLibraryStore());

  const searchTag = ref('');
  const searchByRegex = ref(false);

  const searchFunc = computed(() =>
    searchByRegex.value
      ? (a: string, b: string) => a.match(b)
      : (a: string, b: string) => a.includes(b),
  );
  const filterFunc = computed(
    () => (rows: Metadata[]) =>
      rows.filter(
        (row) =>
          searchFunc.value(row.title, searchTag.value) ||
          row.alias.some((alias) => searchFunc.value(alias, searchTag.value)) ||
          row.tags.some((tag) => searchFunc.value(tag, searchTag.value)) ||
          (row.developer && searchFunc.value(row.developer, searchTag.value)) ||
          (row.publisher && searchFunc.value(row.publisher, searchTag.value)) ||
          (row.platform != PlatformType.Unknown &&
            PlatformType.Steam in row.platform &&
            searchFunc.value(row.platform.Steam.id, searchTag.value)) ||
          (row.platform != PlatformType.Unknown &&
            PlatformType.DLSite in row.platform &&
            searchFunc.value(row.platform.DLSite.id, searchTag.value)) ||
          (row.platform != PlatformType.Unknown &&
            PlatformType.Other in row.platform &&
            (searchFunc.value(row.platform.Other.name, searchTag.value) ||
              (row.platform.Other.id && searchFunc.value(row.platform.Other.id, searchTag.value)))),
      ),
  );
  const filteredRows = computed(() =>
    searchTag.value ? filterFunc.value(data.value) : data.value,
  );

  return { searchTag, searchByRegex, filteredRows };
};

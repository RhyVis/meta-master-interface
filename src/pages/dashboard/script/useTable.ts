import type { QSelectOption } from 'quasar';

import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';

import { columns } from '@/pages/dashboard/define.ts';
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { get } from '@vueuse/core';

export const useTable = () => {
  const { data } = storeToRefs(useLibraryStore());

  const visibleColumns = ref(['title', 'alias', 'tags', 'time_created', 'time_updated']);
  const searchTag = ref('');

  const filteredRows = computed(() => {
    if (searchTag.value) {
      return data.value.filter(
        (row) =>
          row.title.includes(searchTag.value) ||
          row.alias.some((alias) => alias.includes(searchTag.value)) ||
          row.tags.some((tag) => tag.includes(searchTag.value)),
      );
    } else {
      return data.value;
    }
  });

  const paginationOptions = ref<QSelectOption[]>(
    columns.map((col) => ({ value: col.name, label: col.label })),
  );
  const paginationSort = ref('title');
  const pagination = computed(() => ({
    sortBy: get(paginationSort),
    rowsPerPage: 6,
  }));

  return { visibleColumns, searchTag, filteredRows, pagination, paginationSort, paginationOptions };
};

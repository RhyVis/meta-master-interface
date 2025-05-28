import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';

import { useLibraryStore } from '@/pages/dashboard/store.ts';

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

  return { visibleColumns, searchTag, filteredRows };
};

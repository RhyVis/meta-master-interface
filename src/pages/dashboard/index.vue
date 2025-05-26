<script lang="ts" setup>
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { columns } from '@/pages/dashboard/define.ts';
import { useToggle } from '@vueuse/core';
import { useTable } from '@/pages/dashboard/script/useTable.ts';
import { computed, ref } from 'vue';
import DashboardUpdate from '@/pages/dashboard/comp/DashboardUpdate.vue';

const dev = computed(() => import.meta.env.DEV);
const library = useLibraryStore();
const { reload, remove } = library;
const { visibleColumns, searchTag, filteredRows } = useTable();

const [updateState, toggleUpdateState] = useToggle(false);

const editIdx = ref(-1);
const handleUpdate = (index?: number) => {
  if (index != undefined && index >= 0) {
    console.log(`Using edit mode: ${index}`);
    editIdx.value = index;
  } else {
    console.log('Úsing new entry mode');
    editIdx.value = -1;
  }
  toggleUpdateState(true);
};
const handleExit = () => {
  console.log('Exiting update mode');
  toggleUpdateState(false);
  editIdx.value = -1;
};
const handleRemove = (id: string) => {
  handleExit();
  remove(id);
};
</script>

<template>
  <div class="q-pa-md">
    <div class="col q-gutter-md">
      <!-- 数据表 -->
      <q-table
        :columns="columns"
        :rows="filteredRows"
        :visible-columns="visibleColumns"
        grid
        row-key="id"
      >
        <template #top-left>
          <q-btn-group>
            <q-btn icon="refresh" label="刷新" @click="reload" />
            <q-btn icon="add" label="添加" @click="handleUpdate()" />
          </q-btn-group>
        </template>
        <template #top-right>
          <q-chip class="q-mr-sm" v-if="dev">Edit Index: {{ editIdx }}</q-chip>
          <q-input class="q-mr-xs" v-model="searchTag" dense outlined placeholder="搜索">
            <template #append>
              <q-icon name="delete" v-if="searchTag" @click="searchTag = ''" />
              <q-icon name="search" v-else />
            </template>
          </q-input>
          <q-select
            v-model="visibleColumns"
            :options="columns.filter((col) => col.name != 'actions' && col.name != 'title')"
            dense
            display-value="显示内容"
            emit-value
            map-options
            multiple
            option-value="name"
            options-dense
            outlined
          />
        </template>

        <template #item="props">
          <div class="q-pa-xs col-xs-12 col-sm-6 col-md-4 flex flex-col">
            <q-card class="full-height full-width column" bordered flat>
              <q-list class="q-my-sm" dense>
                <q-item v-for="col in props.cols" :key="col.id">
                  <!-- 描述 -->
                  <q-item-section>
                    <template v-if="col.name === 'description'">
                      <div class="r-no-sel text-weight-medium">{{ col.label }}</div>
                      <div
                        class="text-grey-9 text-body2 q-ml-xs"
                        v-for="(line, lineIdx) in col.value"
                        :key="lineIdx"
                      >
                        {{ line }}
                      </div>
                    </template>
                    <!-- 普通Field -->
                    <template v-else>
                      <div class="r-no-sel text-weight-medium">{{ col.label }}</div>
                      <div class="text-body2 text-grey-9 q-ml-xs">{{ col.value }}</div>
                    </template>
                  </q-item-section>
                </q-item>
              </q-list>
              <q-space />
              <q-separator inset />
              <q-card-actions class="q-mt-auto" align="right">
                <q-btn-group flat>
                  <q-btn flat icon="edit" size="sm" @click="handleUpdate(props.rowIndex)" />
                  <q-btn
                    color="negative"
                    flat
                    icon="delete"
                    size="sm"
                    @click="handleRemove(props.row.id)"
                  />
                </q-btn-group>
              </q-card-actions>
            </q-card>
          </div>
        </template>
      </q-table>
    </div>
  </div>

  <q-drawer v-model="updateState" :width="600" no-swipe-close side="right" @hide="handleExit">
    <DashboardUpdate v-model="editIdx" :key="editIdx" @exit="handleExit" />
  </q-drawer>
</template>

<script lang="ts" setup>
import { useLibraryStore } from '@/pages/dashboard/store.ts';
import { columns } from '@/pages/dashboard/define.ts';
import { set, useToggle } from '@vueuse/core';
import { useTable } from '@/pages/dashboard/script/useTable.ts';
import { computed, ref } from 'vue';
import DashboardUpdate from '@/pages/dashboard/comp/DashboardUpdate.vue';
import { openSelectFolder } from '@/api/file-dialog.ts';
import { useQuasar } from 'quasar';
import { ArchiveType, DeployType, type Metadata } from '@/api/types.ts';
import { useGlobalStore } from '@/stores/global.ts';

const dev = computed(() => import.meta.env.DEV || useGlobalStore().develop);
const library = useLibraryStore();
const { reload, remove, deploy, deployOff } = library;
const { visibleColumns, searchTag, filteredRows, pagination, paginationSort, paginationOptions } =
  useTable();
const { notify } = useQuasar();

const [updateState, toggleUpdateState] = useToggle(false);

const editIdx = ref(-1);
const handleUpdate = (index?: number) => {
  if (index != undefined && index >= 0) {
    console.log(`Using edit mode: ${index}`);
    set(editIdx, index);
  } else {
    console.log('Using new entry mode');
    set(editIdx, -1);
  }
  toggleUpdateState(true);
};
const handleExit = () => {
  console.log('Exiting update mode');
  toggleUpdateState(false);
  // Avoid keeping data when creating a new entry continuously
  set(editIdx, -2);
  set(editIdx, -1);
};
const handleRemove = (id: string) => {
  handleExit();
  remove(id);
};
const handleDeploy = async (id: string) => {
  const path = await openSelectFolder();
  if (path) {
    await deploy(id, path);
  } else {
    notify({
      type: 'negative',
      message: '请选择一个有效的目录进行部署',
      color: 'negative',
      position: 'top',
    });
  }
};
</script>

<template>
  <q-page padding>
    <div class="col q-gutter-md">
      <!-- 数据表 -->
      <q-table
        v-model:pagination="pagination"
        :columns="columns"
        :rows="filteredRows"
        :rows-per-page-options="[6, 12, 18, 24, 30, 0]"
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
          <div class="row q-gutter-sm items-center">
            <q-chip v-if="dev">Edit Index: {{ editIdx }}</q-chip>
            <q-chip v-if="dev">Sort By: {{ pagination }}</q-chip>
            <q-input v-model="searchTag" dense outlined placeholder="搜索">
              <template #append>
                <q-icon name="delete" v-if="searchTag" @click="searchTag = ''" />
                <q-icon name="search" v-else />
              </template>
            </q-input>
            <q-select
              v-model="paginationSort"
              :options="paginationOptions"
              dense
              display-value="排序"
              emit-value
              map-options
              options-dense
              outlined
            />
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
          </div>
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
                  <q-btn
                    v-if="
                      (props.row as Metadata).archive_info != ArchiveType.Unset &&
                      (props.row as Metadata).deploy_info == DeployType.Unset
                    "
                    color="primary"
                    flat
                    icon="create_new_folder"
                    size="sm"
                    @click="handleDeploy((props.row as Metadata).id)"
                  >
                    <q-tooltip> 部署到指定目录 </q-tooltip>
                  </q-btn>
                  <q-btn
                    v-if="(props.row as Metadata).deploy_info != DeployType.Unset"
                    color="primary"
                    flat
                    icon="folder_off"
                    size="sm"
                  >
                    <q-tooltip> 取消部署 </q-tooltip>
                    <q-popup-proxy>
                      <q-card>
                        <q-card-section>
                          <div class="r-no-sel text-subtitle2">确定要取消部署吗</div>
                        </q-card-section>
                        <q-separator />
                        <q-card-actions align="right">
                          <q-btn-group flat>
                            <q-btn v-close-popup flat icon="close" size="sm" />
                            <q-btn
                              v-close-popup
                              flat
                              icon="check"
                              size="sm"
                              @click="deployOff((props.row as Metadata).id)"
                            />
                          </q-btn-group>
                        </q-card-actions>
                      </q-card>
                    </q-popup-proxy>
                  </q-btn>
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
  </q-page>

  <q-drawer
    v-model="updateState"
    :width="600"
    bordered
    no-swipe-close
    overlay
    side="right"
    @hide="handleExit"
  >
    <DashboardUpdate v-model="editIdx" :key="editIdx" @exit="handleExit" />
  </q-drawer>
</template>

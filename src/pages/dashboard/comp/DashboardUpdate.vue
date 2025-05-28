<script lang="ts" setup>
import { useUpdate } from '@/pages/dashboard/script/useUpdate.ts';
import { computed, type Ref, ref } from 'vue';
import type { QForm } from 'quasar';
import {
  ArchiveType,
  ArchiveTypeOptions,
  ContentTypeOptions,
  PlatformOptions,
  PlatformType,
} from '@/api/types.ts';
import { set } from '@vueuse/core';
import { openPathTo, openSelectFile, openSelectFolder } from '@/api/file-dialog.ts';
import { generateRandomPassword } from '@/api/util.ts';
import { useGlobalStore } from '@/stores/global.ts';

const dev = computed(() => import.meta.env.DEV || useGlobalStore().develop);
const value = defineModel({
  type: Number,
  required: true,
});
const formRef = ref<QForm>();
const {
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
} = useUpdate(value, formRef as Ref<QForm>);

const emit = defineEmits<{
  (e: 'exit'): void;
}>();

const handleUpdate = async () => {
  if (await doUpdate()) {
    emit('exit');
  }
};
const handleSelectPath = async (fileMode: boolean) => {
  if (fileMode) {
    const path = await openSelectFile();
    if (path) {
      set(cArchivePath, path);
    }
  } else {
    const path = await openSelectFolder();
    if (path) {
      set(cArchivePath, path);
    }
  }
};
const handlePassword = () => {
  if (cArchivePassword.value) {
    set(cArchivePassword, generateRandomPassword());
  } else {
    set(cArchivePassword, 'META');
  }
};
</script>

<template>
  <q-card class="full-height full-width" flat>
    <q-card-section class="text-h6 r-no-sel">
      <div v-if="mode">
        <q-icon class="q-mr-sm" name="edit" />
        <span>编辑 {{ edit.title }}</span>
      </div>
      <div v-else>
        <q-icon class="q-mr-sm" name="add" />
        <span>创建新条目</span>
      </div>
    </q-card-section>
    <q-separator />

    <q-card-section>
      <q-form ref="formRef">
        <!-- Title -->
        <q-input
          v-model="edit.title"
          :rules="[(val) => !!val || '标题不能为空']"
          dense
          hint="通用标题，必须填写"
          label="标题"
          lazy-rules
        />

        <!-- Description -->
        <q-input
          v-model="edit.description"
          autogrow
          dense
          hint="描述性内容"
          label="描述"
          type="textarea"
        />

        <!-- Alias -->
        <q-input
          v-model="cAlias"
          dense
          hint="如外文名等别名"
          label="别名"
          placeholder="回车以添加别名"
          @keyup.enter="addAlias"
        />
        <div class="q-mt-sm q-gutter-xs">
          <q-chip
            class="q-mr-sm"
            v-for="(alias, index) in edit.alias"
            :key="index"
            removable
            size="sm"
            @remove="delAlias(index)"
          >
            <span class="q-pr-xs">{{ alias }}</span>
          </q-chip>
        </div>

        <!-- Tag -->
        <q-input
          v-model="cTag"
          dense
          hint="额外限定，可用于搜索"
          label="标签"
          placeholder="回车以添加别名"
          @keyup.enter="addTag"
        />
        <div class="q-mt-sm q-gutter-xs">
          <q-chip
            class="q-mr-sm"
            v-for="(tag, index) in edit.tags"
            :key="index"
            removable
            size="sm"
            @remove="delTag(index)"
          >
            <span class="q-pr-xs">{{ tag }}</span>
          </q-chip>
        </div>

        <!-- Content Type -->
        <q-select
          v-model="edit.content_type"
          :options="ContentTypeOptions"
          dense
          emit-value
          hint="资源的内容类型"
          label="内容类型"
          map-options
          options-dense
        />

        <!-- Distribution Platform -->
        <q-select
          v-model="cPlatformType"
          :options="PlatformOptions"
          dense
          emit-value
          hint="平台"
          label="平台"
          map-options
          options-dense
        />
        <div class="q-mt-sm" v-if="cPlatformType != PlatformType.Unknown">
          <q-input
            v-if="cPlatformType == PlatformType.Steam"
            v-model="cPlatformID"
            :rules="[(val) => !!val || '必须提供']"
            dense
            hint="平台 ID"
            label="ID"
            lazy-rules
          />
          <q-input
            v-else-if="cPlatformType == PlatformType.DLSite"
            v-model="cPlatformID"
            :rules="[(val) => !!val || '必须提供']"
            dense
            hint="平台 ID"
            label="ID"
            lazy-rules
          />
          <div v-else-if="cPlatformType == PlatformType.Other">
            <q-input
              v-model="cPlatformName"
              :rules="[(val) => !!val || '必须提供']"
              dense
              hint="平台名称"
              label="平台名称"
              lazy-rules
            />
            <q-input v-model="cPlatformID" dense hint="平台 ID" label="ID" />
          </div>
        </div>

        <!-- Developer -->
        <q-input v-model="edit.developer" dense hint="开发者 [Developer]" label="开发者" />

        <!-- Publisher -->
        <q-input v-model="edit.publisher" dense hint="发行商 [Publisher]" label="发行商" />

        <!-- Version -->
        <q-input v-model="edit.version" dense hint="版本 [Version]" label="版本" />

        <!-- Archive Info -->
        <q-select
          class="full-width"
          v-model="cArchiveType"
          :options="ArchiveTypeOptions"
          dense
          emit-value
          hint="存储类型，如压缩包、文件等"
          label="存储类型"
          map-options
          options-dense
        />
        <div class="q-mt-sm" v-if="cArchiveType != ArchiveType.Unset">
          <div v-if="cArchiveType == ArchiveType.CommonFile">
            <q-field
              :rules="[() => !!cArchivePath || '必须提供路径']"
              dense
              label="文件路径"
              lazy-rules
              stack-label
            >
              <template #control>
                <div class="self-center full-width no-outline" @click="openPathTo(cArchivePath)">
                  {{ cArchivePath ?? '未选择文件' }}
                </div>
              </template>
              <template #after>
                <q-btn dense flat icon="file_open" @click="handleSelectPath(true)" />
              </template>
            </q-field>
          </div>
          <div v-else-if="cArchiveType == ArchiveType.Directory">
            <q-field
              :rules="[() => !!cArchivePath || '必须提供路径']"
              dense
              label="文件夹路径"
              lazy-rules
              stack-label
            >
              <template #control>
                <div class="self-center full-width no-outline" @click="openPathTo(cArchivePath)">
                  {{ cArchivePath ?? '未选择文件夹' }}
                </div>
              </template>
              <template #after>
                <q-btn dense flat icon="folder" @click="handleSelectPath(false)" />
              </template>
            </q-field>
          </div>
          <div v-else-if="cArchiveType == ArchiveType.ArchiveFile">
            <q-field
              :hint="mode ? '更新为新的压缩包路径' : '选择被压缩的文件夹'"
              :label="mode ? '存档文件路径' : '源路径'"
              :rules="[() => !!cArchivePath || '必须提供路径']"
              dense
              lazy-rules
              stack-label
            >
              <template #control>
                <div class="self-center full-width no-outline" @click="openPathTo(cArchivePath)">
                  {{ cArchivePath ?? '路径缺失' }}
                </div>
              </template>
              <template #after>
                <q-btn
                  :icon="mode ? 'folder_zip' : 'folder'"
                  dense
                  flat
                  @click="handleSelectPath(mode)"
                />
              </template>
            </q-field>
            <q-input v-model="cArchivePassword" dense hint="存档密码" label="密码">
              <template #after>
                <q-btn dense flat icon="password" @click="handlePassword">
                  <q-tooltip> 随机生成密码 </q-tooltip>
                </q-btn>
              </template>
            </q-input>
          </div>
          <div v-else>Unexpected Archive Type: {{ cArchiveType }}</div>
        </div>
      </q-form>
    </q-card-section>

    <template v-if="dev">
      <q-separator />
      <q-card-section>
        <div class="text-bold">Mode：</div>
        <div>{{ mode ? `Edit ${edit.id}` : 'New' }}</div>
        <br />
        <div class="text-bold">Edit Values</div>
        <div>cAlias: {{ cAlias }}</div>
        <div>cTag: {{ cTag }}</div>
        <br />
        <div class="text-bold">Platform Values</div>
        <div>cPlatformType: {{ cPlatformType }}</div>
        <div>cPlatformID: {{ cPlatformID }}</div>
        <div>cPlatformName: {{ cPlatformName }}</div>
        <br />
        <div class="text-bold">Archive Values</div>
        <div>cArchiveType: {{ cArchiveType }}</div>
        <div>cArchivePath: {{ cArchivePath }}</div>
        <div>cArchivePassword: {{ cArchivePassword }}</div>
        <br />
        <div class="text-bold">Full Content</div>
        <pre>{{ JSON.stringify(edit, null, 2) }}</pre>
      </q-card-section>
    </template>

    <q-separator />
    <q-card-actions align="right">
      <q-btn flat icon="close" label="退出" @click="$emit('exit')" />
      <q-btn
        :icon="mode ? 'edit' : 'add'"
        :label="mode ? '更新' : '创建'"
        color="primary"
        flat
        @click="handleUpdate"
      />
    </q-card-actions>
  </q-card>
</template>

import type { QTableColumn } from 'quasar';
import type { Metadata } from '@/api/types.ts';

import { date } from 'quasar';

import { PlatformType } from '@/api/types.ts';
import { formatBytes } from '@/api/util.ts';

export const columns: QTableColumn[] = [
  {
    name: 'title',
    label: '标题',
    required: true,
    sortable: true,
    align: 'left',
    classes: 'r-no-sel',
    field: 'title',
  },
  {
    name: 'alias',
    label: '别名',
    classes: 'r-no-sel',
    field: (row: Metadata) => (row.alias.length > 0 ? row.alias.join(', ') : '未提供'),
  },
  {
    name: 'tags',
    label: '标签',
    classes: 'r-no-sel',
    field: (row: Metadata) => (row.tags.length > 0 ? row.tags.join(', ') : '未提供'),
  },
  {
    name: 'content_type',
    label: '内容类型',
    classes: 'r-no-sel',
    field: (row: Metadata) => {
      switch (row.content_type) {
        case 'Game':
          return '游戏';
        case 'Novel':
          return '小说';
        case 'Comic':
          return '漫画';
        case 'Music':
          return '音乐';
        case 'Anime':
          return '动画';
        case 'Movie':
          return '电影';
        case 'Software':
          return '软件';
        case 'Other':
          return '其他';
        default:
          return '未明确';
      }
    },
  },
  {
    name: 'platform',
    label: '平台',
    classes: 'r-no-sel',
    field: (row: Metadata) => {
      if (row.platform == PlatformType.Unknown) {
        return '未知';
      }
      if (PlatformType.Steam in row.platform) {
        return `Steam (${row.platform.Steam.id})`;
      }
      if (PlatformType.DLSite in row.platform) {
        return `DLSite (${row.platform.DLSite.id})`;
      }
      if (PlatformType.Other in row.platform) {
        return `其他 (${row.platform.Other.name}${row.platform.Other.id ? `, ID: ${row.platform.Other.id}` : ''})`;
      }
      return '未明确';
    },
  },
  {
    name: 'description',
    label: '描述',
    field: (row: Metadata) => (row.description ? row.description.split('\n') : ['未提供']),
  },
  {
    name: 'developer',
    label: '开发者',
    field: (row: Metadata) => row.developer ?? '未提供',
  },
  {
    name: 'publisher',
    label: '发行商',
    field: (row: Metadata) => row.publisher ?? '未提供',
  },
  {
    name: 'version',
    label: '版本',
    field: (row: Metadata) => row.version ?? '未提供',
  },
  {
    name: 'archive_size',
    label: '归档大小',
    field: (row: Metadata) => formatBytes(row.archive_size),
  },
  {
    name: 'time_created',
    label: '创建时间',
    field: (row: Metadata) => date.formatDate(row.time_created, 'YYYY-MM-DD'),
  },
  {
    name: 'time_updated',
    label: '更新时间',
    field: (row: Metadata) => date.formatDate(row.time_updated, 'YYYY-MM-DD'),
  },
];

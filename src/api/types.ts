import type { QSelectOption } from 'quasar';

export const enum PlatformType {
  Unknown = 'Unknown',
  Steam = 'Steam',
  DLSite = 'DLSite',
  Other = 'Other',
}

export const PlatformOptions: QSelectOption[] = [
  { label: '未知', value: PlatformType.Unknown },
  { label: 'Steam', value: PlatformType.Steam },
  { label: 'DLSite', value: PlatformType.DLSite },
  { label: '其他', value: PlatformType.Other },
];

export type DistributionPlatform =
  | 'Unknown'
  | {
      Steam: {
        id: string;
      };
    }
  | {
      DLSite: {
        id: string;
      };
    }
  | {
      Other: {
        name: string;
        id?: string;
      };
    };

export const enum ArchiveType {
  Unset = 'Unset',
  ArchiveFile = 'ArchiveFile',
  CommonFile = 'CommonFile',
  Directory = 'Directory',
}

export const ArchiveTypeOptions: QSelectOption[] = [
  { label: '未设置', value: ArchiveType.Unset },
  { label: '压缩包', value: ArchiveType.ArchiveFile },
  { label: '通用文件', value: ArchiveType.CommonFile },
  { label: '目录', value: ArchiveType.Directory },
];

export type ArchiveInfo =
  | 'Unset'
  | {
      ArchiveFile: {
        path: string;
        password?: string;
      };
    }
  | {
      CommonFile: {
        path: string;
      };
    }
  | {
      Directory: {
        path: string;
      };
    };

export type Metadata = {
  id: string;
  title: string;
  alias: string[];
  tags: string[];
  platform: DistributionPlatform;
  description?: string;
  developer?: string;
  publisher?: string;
  version?: string;
  archive_info: ArchiveInfo;
  time_created: string;
  time_updated: string;
};

type RequireFields<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export type MetadataOptional = RequireFields<
  Partial<Metadata>,
  'title' | 'alias' | 'tags' | 'platform' | 'archive_info'
>;

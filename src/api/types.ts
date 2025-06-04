import type { QSelectOption } from 'quasar';

export const enum ContentType {
  Other = 'Other',
  Game = 'Game',
  Novel = 'Novel',
  Comic = 'Comic',
  Anime = 'Anime',
  Music = 'Music',
  Movie = 'Movie',
  Software = 'Software',
}

export const ContentTypeOptions: QSelectOption[] = [
  { label: '其他', value: ContentType.Other },
  { label: '游戏', value: ContentType.Game },
  { label: '小说', value: ContentType.Novel },
  { label: '漫画', value: ContentType.Comic },
  { label: '动画', value: ContentType.Anime },
  { label: '音乐', value: ContentType.Music },
  { label: '电影', value: ContentType.Movie },
  { label: '软件', value: ContentType.Software },
];

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
  | PlatformType.Unknown
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
  | ArchiveType.Unset
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

export const enum DeployType {
  Unset = 'Unset',
  File = 'File',
  Directory = 'Directory',
}

export type DeployInfo =
  | DeployType.Unset
  | {
      File: {
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
  content_type: ContentType;
  platform: DistributionPlatform;
  description?: string;
  developer?: string;
  publisher?: string;
  version?: string;
  archive_info: ArchiveInfo;
  archive_size: number;
  deploy_info: DeployInfo;
  time_created: string;
  time_updated: string;
};

type RequireFields<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export type MetadataOptional = RequireFields<
  Partial<Metadata>,
  'title' | 'alias' | 'tags' | 'platform' | 'archive_info'
>;

// API

export type DLSiteInfo = {
  title: string;
  circle: string;
  scenario: string[];
  illustration: string[];
  category: string[];
  tags: string[];
  description: string[];
};

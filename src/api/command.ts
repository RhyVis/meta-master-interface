import type { Metadata, MetadataOptional } from '@/api/types.ts';

import { invoke } from '@tauri-apps/api/core';

export function command_metadata_get_all(): Promise<Metadata[]> {
  return invoke('metadata_get_all');
}

export function command_metadata_get(key: string): Promise<Metadata> {
  return invoke('metadata_get', { key });
}

export function command_metadata_update(opt: MetadataOptional): Promise<string> {
  return invoke('metadata_update', { opt });
}

export function command_metadata_remove(key: string): Promise<void> {
  return invoke('metadata_remove', { key });
}

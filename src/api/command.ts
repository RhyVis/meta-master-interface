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

export function command_metadata_deploy(key: string, target: string): Promise<void> {
  return invoke('metadata_deploy', { key, target });
}

export function command_metadata_deploy_off(key: string): Promise<void> {
  return invoke('metadata_deploy_off', { key });
}

export function command_library_clear(): Promise<void> {
  return invoke('library_clear');
}

export function command_library_export(): Promise<void> {
  return invoke('library_export');
}

export function command_library_import(): Promise<void> {
  return invoke('library_import');
}

export function command_util_resolve_absolute(path: string): Promise<string> {
  return invoke('util_resolve_absolute', { path });
}

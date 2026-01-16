import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import type { ChangedFile, FileDiff, RepoInfo } from './types';

export async function getRepoInfo(repoPath: string): Promise<RepoInfo> {
  return invoke<RepoInfo>('get_repo_info', { repoPath });
}

export async function getChangedFiles(repoPath: string, baseBranch: string): Promise<ChangedFile[]> {
  return invoke<ChangedFile[]>('get_changed_files', { repoPath, baseBranch });
}

export async function getFileDiff(repoPath: string, filePath: string, baseBranch: string): Promise<FileDiff> {
  return invoke<FileDiff>('get_file_diff', { repoPath, filePath, baseBranch });
}

export async function openInEditor(repoPath: string, filePath: string): Promise<void> {
  return invoke<void>('open_in_editor', { repoPath, filePath });
}

export async function selectFolder(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Git Repository'
  });
  return selected as string | null;
}

export async function watchRepo(repoPath: string): Promise<void> {
  return invoke<void>('watch_repo', { repoPath });
}

export async function stopWatching(): Promise<void> {
  return invoke<void>('stop_watching');
}

export interface GitChangeEvent {
  change_type: 'branch' | 'index' | 'refs';
}

export async function onGitChange(callback: (event: GitChangeEvent) => void): Promise<UnlistenFn> {
  return listen<GitChangeEvent>('git-changed', (event) => {
    callback(event.payload);
  });
}

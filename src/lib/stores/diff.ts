import { writable, derived } from 'svelte/store';
import type { ChangedFile, FileDiff, RepoInfo, FileTreeNode } from '../types';

export const repoPath = writable<string>('');
export const repoInfo = writable<RepoInfo | null>(null);
export const baseBranch = writable<string>('main');
export const changedFiles = writable<ChangedFile[]>([]);
export const selectedFile = writable<string | null>(null);
export const currentDiff = writable<FileDiff | null>(null);
export const isLoading = writable<boolean>(false);
export const error = writable<string | null>(null);
export const viewMode = writable<'unified' | 'split'>('unified');

function buildFileTree(files: ChangedFile[]): FileTreeNode[] {
  const root: FileTreeNode[] = [];

  for (const file of files) {
    const parts = file.path.split('/');
    let currentLevel = root;

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const isLast = i === parts.length - 1;
      const currentPath = parts.slice(0, i + 1).join('/');

      let existing = currentLevel.find((n) => n.name === part);

      if (!existing) {
        existing = {
          name: part,
          path: currentPath,
          isDirectory: !isLast,
          children: [],
          file: isLast ? file : undefined,
          expanded: true
        };
        currentLevel.push(existing);
      }

      if (!isLast) {
        currentLevel = existing.children;
      }
    }
  }

  const sortNodes = (nodes: FileTreeNode[]): FileTreeNode[] => {
    return nodes.sort((a, b) => {
      if (a.isDirectory && !b.isDirectory) return -1;
      if (!a.isDirectory && b.isDirectory) return 1;
      return a.name.localeCompare(b.name);
    }).map((node) => ({
      ...node,
      children: sortNodes(node.children)
    }));
  };

  return sortNodes(root);
}

export const fileTree = derived(changedFiles, ($files) => buildFileTree($files));

export const summary = derived(changedFiles, ($files) => {
  const totalAdditions = $files.reduce((sum, f) => sum + f.additions, 0);
  const totalDeletions = $files.reduce((sum, f) => sum + f.deletions, 0);
  return {
    fileCount: $files.length,
    additions: totalAdditions,
    deletions: totalDeletions
  };
});

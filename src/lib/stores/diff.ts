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

// File tree filters
export const fileSearch = writable<string>('');
export const showAdded = writable<boolean>(true);
export const showModified = writable<boolean>(true);
export const showDeleted = writable<boolean>(true);
export const showCosmetic = writable<boolean>(true);
export const allCollapsed = writable<boolean>(false);

// Filtered files based on search and status toggles
export const filteredFiles = derived(
  [changedFiles, fileSearch, showAdded, showModified, showDeleted, showCosmetic],
  ([$files, $search, $showAdded, $showModified, $showDeleted, $showCosmetic]) => {
    return $files.filter((file) => {
      // Cosmetic filter - hide cosmetic-only changes when disabled
      if (!$showCosmetic && file.is_cosmetic) return false;

      // Status filter
      const status = file.status;
      if (status === 'added' && !$showAdded) return false;
      if (status === 'modified' && !$showModified) return false;
      if (status === 'deleted' && !$showDeleted) return false;

      // Search filter (case-insensitive fuzzy match on path)
      if ($search) {
        const searchLower = $search.toLowerCase();
        const pathLower = file.path.toLowerCase();

        // Simple fuzzy: check if all characters appear in order
        let searchIdx = 0;
        for (let i = 0; i < pathLower.length && searchIdx < searchLower.length; i++) {
          if (pathLower[i] === searchLower[searchIdx]) {
            searchIdx++;
          }
        }
        if (searchIdx < searchLower.length) return false;
      }

      return true;
    });
  }
);

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

export const fileTree = derived(filteredFiles, ($files) => buildFileTree($files));

// Summary of all files (not filtered)
export const summary = derived(changedFiles, ($files) => {
  const totalAdditions = $files.reduce((sum, f) => sum + f.additions, 0);
  const totalDeletions = $files.reduce((sum, f) => sum + f.deletions, 0);
  const added = $files.filter(f => f.status === 'added').length;
  const modified = $files.filter(f => f.status === 'modified').length;
  const deleted = $files.filter(f => f.status === 'deleted').length;
  const cosmetic = $files.filter(f => f.is_cosmetic).length;
  return {
    fileCount: $files.length,
    additions: totalAdditions,
    deletions: totalDeletions,
    added,
    modified,
    deleted,
    cosmetic
  };
});

// Summary of filtered files
export const filteredSummary = derived(filteredFiles, ($files) => {
  const totalAdditions = $files.reduce((sum, f) => sum + f.additions, 0);
  const totalDeletions = $files.reduce((sum, f) => sum + f.deletions, 0);
  return {
    fileCount: $files.length,
    additions: totalAdditions,
    deletions: totalDeletions
  };
});

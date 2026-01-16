import { get } from 'svelte/store';
import {
  selectedFile, filteredFiles, repoPath, baseBranch, repoInfo,
  currentDiff, isLoading, viewMode, fileSearch, changedFiles,
  showAdded, showModified, showDeleted, showCosmetic, allCollapsed
} from './stores';
import { getFileDiff, selectFolder, getRepoInfo, getChangedFiles } from './tauri';
import type { ChangedFile } from './types';

let searchInput: HTMLInputElement | null = null;

export function setSearchInput(el: HTMLInputElement | null) {
  searchInput = el;
}

export function handleKeyboard(e: KeyboardEvent) {
  // Skip if typing in input (except Escape)
  if (e.target instanceof HTMLInputElement) {
    if (e.key === 'Escape') {
      (e.target as HTMLInputElement).blur();
      fileSearch.set('');
    }
    return;
  }

  const files = get(filteredFiles);
  const current = get(selectedFile);
  const idx = current ? files.findIndex(f => f.path === current) : -1;

  switch (e.key) {
    // Navigation
    case 'ArrowDown':
    case 'j':
      e.preventDefault();
      selectFile(files, idx + 1);
      break;
    case 'ArrowUp':
    case 'k':
      e.preventDefault();
      selectFile(files, idx - 1);
      break;
    case 'Enter':
      if (idx === -1 && files.length > 0) selectFile(files, 0);
      break;
    case 'Escape':
      selectedFile.set(null);
      currentDiff.set(null);
      break;

    // Actions
    case 'o':
      openRepo();
      break;
    case 'r':
      if (get(repoInfo)) refresh();
      break;
    case '/':
      e.preventDefault();
      searchInput?.focus();
      break;
    case 'v':
      viewMode.update(m => m === 'unified' ? 'split' : 'unified');
      break;

    // Filters
    case '1':
      showAdded.update(v => !v);
      handleFilterChange();
      break;
    case '2':
      showModified.update(v => !v);
      handleFilterChange();
      break;
    case '3':
      showDeleted.update(v => !v);
      handleFilterChange();
      break;
    case '4':
      showCosmetic.update(v => !v);
      handleFilterChange();
      break;

    // Tree
    case '[':
      allCollapsed.set(true);
      setTimeout(() => allCollapsed.set(false), 50);
      break;
    case ']':
      expandAll();
      break;
  }
}

async function selectFile(files: ChangedFile[], idx: number) {
  if (files.length === 0) return;
  const clamped = Math.max(0, Math.min(files.length - 1, idx));
  const file = files[clamped];

  selectedFile.set(file.path);
  isLoading.set(true);
  try {
    const diff = await getFileDiff(get(repoPath), file.path, get(baseBranch));
    currentDiff.set(diff);

    // Scroll file into view in tree
    setTimeout(() => {
      const el = document.querySelector(`[data-file-path="${file.path}"]`);
      el?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }, 10);

    // Reset diff scroll to top
    setTimeout(() => {
      document.querySelector('[data-diff-scroll]')?.scrollTo(0, 0);
    }, 10);
  } finally {
    isLoading.set(false);
  }
}

function handleFilterChange() {
  // After filter toggle, check if selected file is still visible
  setTimeout(() => {
    const files = get(filteredFiles);
    const current = get(selectedFile);

    if (current && !files.find(f => f.path === current)) {
      // Selected file was filtered out
      if (files.length > 0) {
        selectFile(files, 0);
      } else {
        selectedFile.set(null);
        currentDiff.set(null);
      }
    }
  }, 0);
}

async function openRepo() {
  const folder = await selectFolder();
  if (!folder) return;

  repoPath.set(folder);
  isLoading.set(true);
  try {
    const info = await getRepoInfo(folder);
    repoInfo.set(info);
    baseBranch.set(info.default_base);
    await refresh();
  } finally {
    isLoading.set(false);
  }
}

async function refresh() {
  const path = get(repoPath);
  const base = get(baseBranch);
  if (!path || !base) return;

  isLoading.set(true);
  try {
    const files = await getChangedFiles(path, base);
    changedFiles.set(files);
    selectedFile.set(null);
    currentDiff.set(null);
  } finally {
    isLoading.set(false);
  }
}

function expandAll() {
  const current = get(fileSearch);
  fileSearch.set(current + ' ');
  setTimeout(() => fileSearch.set(current), 10);
}

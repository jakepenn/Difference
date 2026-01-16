export interface ChangedFile {
  path: string;
  status: 'added' | 'deleted' | 'modified' | 'renamed' | 'copied' | 'typechange' | 'unknown';
  additions: number;
  deletions: number;
  is_cosmetic: boolean;
}

export interface DiffLine {
  content: string;
  line_type: 'add' | 'delete' | 'context';
  old_lineno: number | null;
  new_lineno: number | null;
}

export interface DiffHunk {
  old_start: number;
  old_lines: number;
  new_start: number;
  new_lines: number;
  lines: DiffLine[];
  is_cosmetic: boolean;
}

export interface FileDiff {
  path: string;
  hunks: DiffHunk[];
  is_binary: boolean;
  is_cosmetic: boolean;
}

export interface BranchInfo {
  name: string;
  is_current: boolean;
  is_remote: boolean;
}

export interface RepoInfo {
  path: string;
  current_branch: string;
  branches: BranchInfo[];
  default_base: string;
}

export interface FileTreeNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children: FileTreeNode[];
  file?: ChangedFile;
  expanded?: boolean;
}

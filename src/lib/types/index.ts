export interface FileInfo {
  path: string;
  name: string;
  paragraphs: string[];
}

export type AppState = 'upload' | 'loading' | 'diff';
export type ViewMode = 'side-by-side' | 'line-by-line';

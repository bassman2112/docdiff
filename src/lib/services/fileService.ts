import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export async function extractText(filePath: string): Promise<string[]> {
  return invoke<string[]>('extract_text', { filePath });
}

export async function pickDocFile(): Promise<string | null> {
  const result = await open({
    multiple: false,
    filters: [
      { name: 'Documents', extensions: ['docx', 'txt', 'md'] }
    ]
  });
  if (typeof result === 'string') return result;
  return null;
}

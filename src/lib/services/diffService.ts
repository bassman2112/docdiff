import { createTwoFilesPatch } from 'diff';
import { html, type Diff2HtmlConfig } from 'diff2html';
import type { ViewMode } from '$lib/types';

export function computeDiffHtml(
  beforeName: string,
  afterName: string,
  beforeParagraphs: string[],
  afterParagraphs: string[],
  viewMode: ViewMode,
  fullFile: boolean
): string {
  const beforeText = beforeParagraphs.join('\n');
  const afterText = afterParagraphs.join('\n');

  const patch = createTwoFilesPatch(
    beforeName,
    afterName,
    beforeText,
    afterText,
    '',
    '',
    { context: fullFile ? 999999 : 3 }
  );

  const config: Diff2HtmlConfig = {
    outputFormat: viewMode === 'side-by-side' ? 'side-by-side' : 'line-by-line',
    matching: 'words',
    drawFileList: false
  };

  return html(patch, config);
}

export function areIdentical(before: string[], after: string[]): boolean {
  return before.join('\n') === after.join('\n');
}

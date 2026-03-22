<script lang="ts">
  let { size = 40 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
  <!-- Back doc (slightly offset) -->
  <rect x="14" y="6" width="36" height="46" rx="4" fill="var(--dd-logo-back)" stroke="var(--dd-logo-back-stroke)" stroke-width="1.5"/>
  <!-- Front doc -->
  <rect x="10" y="10" width="36" height="46" rx="4" fill="var(--dd-logo-front)" stroke="var(--dd-logo-front-stroke)" stroke-width="1.5"/>
  <!-- Diff lines -->
  <rect x="16" y="18" width="18" height="3" rx="1.5" fill="var(--dd-red)" opacity="0.7"/>
  <rect x="16" y="25" width="22" height="3" rx="1.5" fill="var(--dd-green)" opacity="0.7"/>
  <rect x="16" y="32" width="14" height="3" rx="1.5" fill="var(--dd-logo-ctx)" opacity="0.4"/>
  <rect x="16" y="39" width="20" height="3" rx="1.5" fill="var(--dd-red)" opacity="0.7"/>
  <rect x="16" y="46" width="16" height="3" rx="1.5" fill="var(--dd-green)" opacity="0.7"/>
  <!-- Magnifying glass -->
  <circle cx="46" cy="46" r="10" fill="var(--dd-logo-glass)" stroke="var(--dd-logo-glass-stroke)" stroke-width="2.5"/>
  <line x1="53" y1="53" x2="60" y2="60" stroke="var(--dd-logo-glass-stroke)" stroke-width="2.5" stroke-linecap="round"/>
  <!-- +/- in magnifying glass -->
  <text x="42" y="45" font-size="7" font-weight="bold" fill="var(--dd-red)" font-family="monospace">-</text>
  <text x="47" y="50" font-size="7" font-weight="bold" fill="var(--dd-green)" font-family="monospace">+</text>
</svg>

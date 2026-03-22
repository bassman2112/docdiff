let dark = $state(true);

export function isDark() {
  return dark;
}

export function toggle() {
  dark = !dark;
  if (dark) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}

export function init() {
  if (dark) {
    document.documentElement.classList.add('dark');
  }
}

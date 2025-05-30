// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function removeEmptyStrings(obj: any): any {
  if (Array.isArray(obj)) {
    return obj.map(removeEmptyStrings);
  } else if (obj && typeof obj === 'object') {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result: any = {};
    for (const [key, value] of Object.entries(obj)) {
      const cleaned = removeEmptyStrings(value);
      if (cleaned === '' || cleaned === undefined) continue;
      result[key] = cleaned;
    }
    return result;
  } else if (typeof obj === 'string') {
    const trimmed = obj.trim();
    return trimmed === '' ? undefined : trimmed;
  }
  return obj;
}

export function generateRandomPassword(length = 8) {
  const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+';
  let password = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * charset.length);
    password += charset[randomIndex];
  }
  return password;
}

export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = bytes / Math.pow(k, i);
  return `${size.toFixed(2)}${units[i]}`;
}

export function truncateString(str: string, maxLength = 40): string {
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - 3) + '...';
}

export async function checkForUpdate(): Promise<null> {
  return null;
}

export interface UpdateInfo {
  version: string;
  currentVersion: string;
}

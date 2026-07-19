import React, { createContext, useContext, useCallback } from "react";

interface UpdateContextValue {
  hasUpdate: boolean;
  updateInfo: null;
  isChecking: boolean;
  error: string | null;
  isDismissed: boolean;
  dismissUpdate: () => void;
  checkUpdate: () => Promise<boolean>;
  resetDismiss: () => void;
}

const UpdateContext = createContext<UpdateContextValue | undefined>(undefined);

export function UpdateProvider({ children }: { children: React.ReactNode }) {
  const value: UpdateContextValue = {
    hasUpdate: false,
    updateInfo: null,
    isChecking: false,
    error: null,
    isDismissed: true,
    dismissUpdate: useCallback(() => {}, []),
    checkUpdate: useCallback(async () => false, []),
    resetDismiss: useCallback(() => {}, []),
  };

  return (
    <UpdateContext.Provider value={value}>{children}</UpdateContext.Provider>
  );
}

export function useUpdate() {
  const context = useContext(UpdateContext);
  if (!context) {
    throw new Error("useUpdate must be used within UpdateProvider");
  }
  return context;
}

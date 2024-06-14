
export type {}  

declare global {
  interface Window {
    __TAURI__: {
      tauri: {
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        invoke: (command: string, args?: any) => Promise<any>;
      }
    };
  }
}
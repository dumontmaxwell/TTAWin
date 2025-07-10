import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    macrosEnabled: false,
    hotkey: 'Ctrl+Shift+M',
    transparency: 50,
    alwaysOnTop: true,
    // Add more settings as needed
  }),
  actions: {
    setMacrosEnabled(val: boolean) { this.macrosEnabled = val },
    setHotkey(val: string) { this.hotkey = val },
    setTransparency(val: number) { this.transparency = val },
    setAlwaysOnTop(val: boolean) { this.alwaysOnTop = val },
    // Add more setters as needed
  }
}) 
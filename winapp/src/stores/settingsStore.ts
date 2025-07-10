import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    macrosEnabled: false,
    hotkey: 'Ctrl+Shift+M',
    transparency: 50,
    alwaysOnTop: true,
    // New fields for condensed settings modal
    audioTranscription: false,
    echotap: false,
    modelType: 'local', // 'local' or 'online'
    subscription: 'max', // 'max', 'edge', 'go'
  }),
  actions: {
    setMacrosEnabled(val: boolean) { this.macrosEnabled = val },
    setHotkey(val: string) { this.hotkey = val },
    setTransparency(val: number) { this.transparency = val },
    setAlwaysOnTop(val: boolean) { this.alwaysOnTop = val },
    // New setters
    setAudioTranscription(val: boolean) { this.audioTranscription = val },
    setEchotap(val: boolean) { this.echotap = val },
    setModelType(val: string) { this.modelType = val },
    setSubscription(val: string) { this.subscription = val },
  }
}) 
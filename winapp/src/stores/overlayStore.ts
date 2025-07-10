import { defineStore } from 'pinia'

export const useOverlayStore = defineStore('overlay', {
  state: () => ({
    micEnabled: true,
    monitors: [] as string[],
    currentMonitorIndex: 0,
  }),
  actions: {
    setMicEnabled(val: boolean) { this.micEnabled = val },
    setMonitors(monitors: string[]) { this.monitors = monitors },
    setCurrentMonitorIndex(idx: number) { this.currentMonitorIndex = idx },
  }
}) 
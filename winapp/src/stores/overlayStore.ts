import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useOverlayStore = defineStore('overlay', () => {
  const micEnabled = ref(false)
  const monitors = ref<string[]>([])
  const currentMonitorIndex = ref(0)
  const overlayVisible = ref(false)

  const toggleMic = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      if (micEnabled.value) {
        await invoke('stop_audio_stream')
      } else {
        await invoke('start_audio_stream')
      }
      micEnabled.value = !micEnabled.value
    } catch (error) {
      console.error('Failed to toggle mic:', error)
    }
  }

  const switchMonitor = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const nextIndex = await invoke('switch_monitor', { currentIndex: currentMonitorIndex.value })
      currentMonitorIndex.value = nextIndex as number
    } catch (error) {
      console.error('Failed to switch monitor:', error)
    }
  }

  const setOverlayHidden = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_overlay_hidden')
      overlayVisible.value = false
      console.log('Overlay set to hidden state')
    } catch (error) {
      console.error('Failed to set overlay hidden:', error)
    }
  }

  const setOverlayVisible = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_overlay_visible')
      overlayVisible.value = true
      console.log('Overlay set to visible state')
    } catch (error) {
      console.error('Failed to set overlay visible:', error)
    }
  }

  const toggleOverlay = async () => {
    if (overlayVisible.value) {
      await setOverlayHidden()
    } else {
      await setOverlayVisible()
    }
  }

  const loadMonitors = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const monitorList = await invoke('get_monitors')
      monitors.value = monitorList as string[]
    } catch (error) {
      console.error('Failed to load monitors:', error)
    }
  }

  return {
    micEnabled,
    monitors,
    currentMonitorIndex,
    overlayVisible,
    toggleMic,
    switchMonitor,
    setOverlayHidden,
    setOverlayVisible,
    toggleOverlay,
    loadMonitors
  }
}) 
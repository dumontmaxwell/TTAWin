import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useOverlayStore } from '../stores/overlayStore'

export function useOverlayControls() {
  const store = useOverlayStore()

  const fetchMonitors = async () => {
    const res = await invoke('get_monitors')
    if (res && typeof res === 'object' && 'data' in res) {
      store.setMonitors(res.data as string[])
    }
  }

  const switchMonitor = async () => {
    const res = await invoke('switch_monitor', { current_index: store.currentMonitorIndex })
    if (res && typeof res === 'object' && 'data' in res) {
      store.setCurrentMonitorIndex(res.data as number)
    }
  }

  const fetchMicState = async () => {
    const res = await invoke('get_mic_state')
    if (res && typeof res === 'object' && 'data' in res) {
      store.setMicEnabled(res.data as boolean)
    }
  }

  const toggleMic = async () => {
    const res = await invoke('toggle_mic', { current: store.micEnabled })
    if (res && typeof res === 'object' && 'data' in res) {
      store.setMicEnabled(res.data as boolean)
    }
  }

  const startAudioStream = async () => {
    const res = await invoke('start_audio_stream')
    if (res && typeof res === 'object' && 'data' in res) {
      console.log('Audio stream started')
    }
  }

  const stopAudioStream = async () => {
    const res = await invoke('stop_audio_stream')
    if (res && typeof res === 'object' && 'data' in res) {
      console.log('Audio stream stopped')
    }
  }

  onMounted(() => {
    fetchMonitors()
    fetchMicState()
  })

  return {
    store,
    fetchMonitors,
    switchMonitor,
    fetchMicState,
    toggleMic,
    startAudioStream,
    stopAudioStream,
  }
} 
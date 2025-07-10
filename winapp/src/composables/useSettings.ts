import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../stores/settingsStore'

export function useSettings() {
  const store = useSettingsStore()

  // Example: Fetch settings from backend (stub)
  const fetchSettings = async () => {
    // You can implement a Tauri command like 'get_settings' to fetch persisted settings
    // For now, just a stub
    // const res = await invoke('get_settings')
    // if (res && typeof res === 'object' && 'data' in res) {
    //   // Update store with backend data
    // }
  }

  // Example: Save settings to backend (stub)
  const saveSettings = async () => {
    // You can implement a Tauri command like 'set_settings' to persist settings
    // await invoke('set_settings', { ...store.$state })
  }

  onMounted(() => {
    fetchSettings()
  })

  return {
    store,
    fetchSettings,
    saveSettings,
  }
} 
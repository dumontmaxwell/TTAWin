<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from "@tauri-apps/api/core"
import { listen } from '@tauri-apps/api/event'
// Pinia and composable imports
import { useOverlayControls } from './composables/useOverlayControls'
import { useSettings } from './composables/useSettings'
import { storeToRefs } from 'pinia'

interface OverlayState {
  enabled: boolean
  window_handle: number | null
  topmost: boolean
  transparent: boolean
}

const overlayState = ref<OverlayState>({
  enabled: false,
  window_handle: null,
  topmost: false,
  transparent: false
})

const showSettings = ref(false)

let statusInterval: number | null = null
let unlistenFns: (() => void)[] = []

// Initialize overlay system
const initOverlay = async () => {
  try {
    const response = await invoke('init_overlay')
    console.log('Overlay initialized:', response)
  } catch (error) {
    console.error('Failed to initialize overlay:', error)
  }
}

// Get overlay state
const getOverlayState = async () => {
  try {
    const response = await invoke('get_overlay_state')
    console.log('Overlay state:', response)
    // Update state based on response
    if (response && typeof response === 'object' && 'data' in response) {
      overlayState.value = response.data as OverlayState
    }
  } catch (error) {
    console.error('Failed to get overlay state:', error)
  }
}

// Toggle overlay
const toggleOverlay = async () => {
  try {
    const response = await invoke('toggle_overlay')
    console.log('Toggle overlay response:', response)
    if (response && typeof response === 'object' && 'data' in response) {
      overlayState.value = response.data as OverlayState
    }
  } catch (error) {
    console.error('Failed to toggle overlay:', error)
  }
}

// Close overlay
const closeOverlay = async () => {
  try {
    await invoke('cleanup_overlay', { with_exit: true })
    // Close the application
    window.close()
  } catch (error) {
    console.error('Failed to close overlay:', error)
  }
}

// Settings functions
const openSettings = () => {
  showSettings.value = true
}

const closeSettings = () => {
  showSettings.value = false
}

const {
  store,
  switchMonitor,
  toggleMic,
} = useOverlayControls()

const { micEnabled, monitors, currentMonitorIndex } = storeToRefs(store)

const { store: settingsStore } = useSettings()

// Shortcut keys for each action
const SHORTCUTS = {
  mic: 'Ctrl+Shift+M',
  monitor: 'Ctrl+Shift+N',
  settings: 'Ctrl+Shift+S',
  quit: 'Ctrl+Shift+Q',
}

// Handle hotkey events from Rust
const handleHotkeyEvent = async (event: any) => {
  const { action } = event.payload
  
  console.log('Hotkey triggered:', action)
  
  switch (action) {
    case 'toggle_mic':
      await toggleMic()
      break
    case 'switch_monitor':
      if (monitors.value.length > 1) {
        await switchMonitor()
      }
      break
    case 'open_settings':
      openSettings()
      break
    case 'quit':
      await closeOverlay()
      break
    default:
      console.log('Unknown hotkey action:', action)
  }
}

// Setup hotkey listeners
const setupHotkeyListeners = async () => {
  try {
    const unlisten = await listen('hotkey-triggered', handleHotkeyEvent)
    unlistenFns.push(unlisten)
    console.log('Hotkey listeners setup complete')
  } catch (error) {
    console.error('Failed to setup hotkey listeners:', error)
  }
}

// Test hotkey function for debugging
const testHotkey = async (action: string) => {
  try {
    await invoke('test_hotkey', { action })
    console.log('Test hotkey triggered:', action)
  } catch (error) {
    console.error('Failed to test hotkey:', error)
  }
}

// Lifecycle
onMounted(async () => {
  await initOverlay()
  await getOverlayState()
  await setupHotkeyListeners()
  
  // Poll for state changes
  statusInterval = setInterval(getOverlayState, 1000)
})

onUnmounted(() => {
  if (statusInterval) {
    clearInterval(statusInterval)
  }
  
  // Cleanup event listeners
  unlistenFns.forEach(unlisten => unlisten())
})
</script>

<template>
  <div id="app" class="overlay-container">
    <!-- Header with close and settings buttons -->
    <div class="overlay-header">
      <div class="header-buttons">
        <!-- Microphone Toggle Button -->
        <button
          @click="toggleMic"
          class="header-btn mic-btn"
          :class="micEnabled ? 'mic-on' : 'mic-off'"
          :title="micEnabled ? 'Microphone On' : 'Microphone Off'"
        >
          <div class="btn-content-row">
            <span class="btn-action-text">{{ micEnabled ? 'Mute' : 'Unmute' }}</span>
            <i class="fas" :class="micEnabled ? 'fa-microphone' : 'fa-microphone-slash'"></i>
          </div>
          <div class="btn-shortcut">{{ SHORTCUTS.mic }}</div>
        </button>
        <!-- Monitor Switch Button (only if more than one monitor) -->
        <button
          v-if="monitors.length > 1"
          @click="switchMonitor"
          class="header-btn monitor-btn"
          title="Switch Monitor"
        >
          <div class="btn-content-row">
            <span class="btn-action-text">Monitor</span>
            <i class="fas fa-desktop"></i>
            <span class="monitor-number">{{ currentMonitorIndex + 1 }}</span>
          </div>
          <div class="btn-shortcut">{{ SHORTCUTS.monitor }}</div>
        </button>
        <button @click="openSettings" class="header-btn settings-btn" title="Settings">
          <div class="btn-content-row">
            <span class="btn-action-text">Settings</span>
            <i class="fas fa-cog"></i>
          </div>
          <div class="btn-shortcut">{{ SHORTCUTS.settings }}</div>
        </button>
        <button @click="closeOverlay" class="header-btn close-btn" title="Close">
          <div class="btn-content-row">
            <span class="btn-action-text">Quit</span>
            <i class="fas fa-times"></i>
          </div>
          <div class="btn-shortcut">{{ SHORTCUTS.quit }}</div>
        </button>
      </div>
    </div>

    <!-- Main overlay content -->
    <div class="overlay-content">
      <div class="overlay-status">
        <h2>Overlay Status</h2>
        <div class="status-indicator" :class="{ active: overlayState.enabled }">
          <i class="fas" :class="overlayState.enabled ? 'fa-check-circle' : 'fa-times-circle'"></i>
          <span>{{ overlayState.enabled ? 'Active' : 'Inactive' }}</span>
        </div>
        
        <div class="overlay-controls">
          <button @click="toggleOverlay" class="control-btn primary">
            <i class="fas" :class="overlayState.enabled ? 'fa-pause' : 'fa-play'"></i>
            {{ overlayState.enabled ? 'Disable' : 'Enable' }} Overlay
          </button>
          
          <!-- Test buttons for hotkeys -->
          <div class="test-hotkeys" style="margin-top: 20px; padding: 15px; background: rgba(0,0,0,0.05); border-radius: 8px;">
            <h4 style="margin: 0 0 10px 0; color: #666;">Test Hotkeys:</h4>
            <div style="display: flex; gap: 10px; flex-wrap: wrap;">
              <button @click="testHotkey('toggle_mic')" class="control-btn secondary">Test Mic Toggle</button>
              <button @click="testHotkey('switch_monitor')" class="control-btn secondary">Test Monitor Switch</button>
              <button @click="testHotkey('open_settings')" class="control-btn secondary">Test Settings</button>
              <button @click="testHotkey('quit')" class="control-btn secondary">Test Quit</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Settings Modal -->
    <div v-if="showSettings" class="settings-modal" @click="closeSettings">
      <div class="settings-content" @click.stop>
        <div class="settings-header">
          <h3>Settings</h3>
          <button @click="closeSettings" class="close-btn">
            <i class="fas fa-times"></i>
          </button>
        </div>
        
        <div class="settings-body">
          <div class="setting-group">
            <h4>User Profile</h4>
            <div class="setting-item">
              <label>Account Status:</label>
              <span class="status-badge trial">Trial Mode</span>
            </div>
            <button class="btn-secondary">Login / Create Account</button>
          </div>
          
          <div class="setting-group">
            <h4>Macro Commands</h4>
            <div class="setting-item">
              <label>Enable Macros:</label>
              <input type="checkbox" v-model="settingsStore.macrosEnabled" />
            </div>
            <div class="setting-item">
              <label>Hotkey:</label>
              <input type="text" v-model="settingsStore.hotkey" placeholder="Ctrl+Shift+M" />
            </div>
          </div>
          
          <div class="setting-group">
            <h4>Overlay Settings</h4>
            <div class="setting-item">
              <label>Transparency:</label>
              <input type="range" v-model="settingsStore.transparency" min="0" max="100" />
              <span>{{ settingsStore.transparency }}%</span>
            </div>
            <div class="setting-item">
              <label>Always on Top:</label>
              <input type="checkbox" v-model="settingsStore.alwaysOnTop" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(2px);
  z-index: 9999;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.overlay-header {
  position: absolute;
  top: 0;
  right: 0;
  padding: 20px;
  z-index: 10000;
}

.header-buttons {
  display: flex;
  gap: 10px;
}

.header-btn {
  border: none;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.9);
  color: #333;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  min-width: 100px;
  min-height: 60px;
  padding: 8px 12px;
  box-sizing: border-box;
}

.header-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.settings-btn:hover {
  background: rgba(25, 118, 210, 0.9);
  color: white;
}

.close-btn {
  background: rgba(193, 0, 21, 0.95);
  color: white;
  border: 2px solid #c10015;
  font-weight: bold;
  transition: all 0.2s;
}
.close-btn:hover {
  background: #fff;
  color: #c10015;
  border: 2px solid #c10015;
  box-shadow: 0 0 0 2px #c1001533;
}

.overlay-content {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  padding: 20px;
}

.overlay-status {
  background: rgba(255, 255, 255, 0.95);
  padding: 40px;
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(20px);
  text-align: center;
  min-width: 300px;
}

.overlay-status h2 {
  margin: 0 0 20px 0;
  color: #333;
  font-size: 24px;
}

.status-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin: 20px 0;
  padding: 15px;
  border-radius: 8px;
  background: rgba(193, 0, 21, 0.1);
  color: #c10015;
  font-weight: 600;
}

.status-indicator.active {
  background: rgba(33, 186, 69, 0.1);
  color: #21ba45;
}

.status-indicator i {
  font-size: 20px;
}

.overlay-controls {
  margin-top: 30px;
}

.control-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 auto;
}

.control-btn.primary {
  background: #1976d2;
  color: white;
}

.control-btn.primary:hover {
  background: #1565c0;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(25, 118, 210, 0.3);
}

.control-btn.secondary {
  background: #f5f5f5;
  color: #333;
  border: 1px solid #ddd;
}

.control-btn.secondary:hover {
  background: #e0e0e0;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* Settings Modal */
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10001;
}

.settings-content {
  background: white;
  border-radius: 16px;
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.2);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #eee;
}

.settings-header h3 {
  margin: 0;
  color: #333;
}

.settings-body {
  padding: 24px;
}

.setting-group {
  margin-bottom: 30px;
}

.setting-group h4 {
  margin: 0 0 15px 0;
  color: #333;
  font-size: 18px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 15px;
  padding: 10px 0;
}

.setting-item label {
  font-weight: 500;
  color: #555;
}

.setting-item input[type="text"] {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
}

.setting-item input[type="range"] {
  width: 100px;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
}

.status-badge.trial {
  background: rgba(242, 192, 55, 0.2);
  color: #f2c037;
}

.btn-secondary {
  padding: 10px 20px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  color: #333;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 14px;
}

.btn-secondary:hover {
  background: #f5f5f5;
  border-color: #ccc;
}

.mic-btn {
  position: relative;
}
.mic-on {
  background: rgba(33, 186, 69, 0.9);
  color: white;
}
.mic-off {
  background: rgba(193, 0, 21, 0.9);
  color: white;
}
.monitor-btn {
  background: white;
  color: #1976d2;
  position: relative;
}
.monitor-btn .monitor-number {
  margin-left: 6px;
  position: static;
  background: #1976d2;
  color: white;
  border-radius: 50%;
  font-size: 12px;
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.btn-content-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
}
.btn-action-text {
  font-size: 14px;
  font-weight: 600;
}
.btn-shortcut {
  display: block;
  font-size: 10px;
  color: #888;
  font-family: 'Fira Mono', 'Consolas', monospace;
  margin-top: 4px;
  text-align: center;
  width: 100%;
}
</style>
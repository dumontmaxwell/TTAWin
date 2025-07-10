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
const clickThroughEnabled = ref(true)

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
    await invoke('quit_app')
  } catch (error) {
    console.error('Failed to quit application:', error)
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

// Unified action handler - used by both hotkeys and button clicks
const executeAction = async (action: string) => {
  console.log('Executing action:', action)
  
  // Send action to backend to trigger the event system
  try {
    await invoke('trigger_action', { action })
  } catch (error) {
    console.error('Failed to trigger action:', error)
    // Fallback to direct execution if event system fails
    await executeActionDirect(action)
  }
}

// Direct action execution (fallback)
const executeActionDirect = async (action: string) => {
  console.log('Executing action directly:', action)
  
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
    case 'toggle_overlay':
      await toggleOverlay()
      break
    default:
      console.log('Unknown action:', action)
  }
}

// Handle hotkey events from Rust
const handleHotkeyEvent = async (event: any) => {
  const { action, timestamp, source } = event.payload
  console.log('Hotkey triggered:', { action, timestamp, source })
  
  // Add visual feedback for hotkey triggers
  if (source === 'hotkey') {
    showHotkeyFeedback(action)
  }
  
  // Execute the action directly since it came from the event system
  await executeActionDirect(action)
}

// Visual feedback for hotkey triggers
const showHotkeyFeedback = (action: string) => {
  // Create a temporary visual indicator
  const feedback = document.createElement('div')
  feedback.className = 'hotkey-feedback'
  feedback.textContent = `Hotkey: ${action}`
  feedback.style.cssText = `
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: rgba(25, 118, 210, 0.9);
    color: white;
    padding: 10px 20px;
    border-radius: 8px;
    font-weight: bold;
    z-index: 10002;
    animation: fadeInOut 1.5s ease-in-out;
  `
  
  // Add animation styles
  const style = document.createElement('style')
  style.textContent = `
    @keyframes fadeInOut {
      0% { opacity: 0; transform: translate(-50%, -50%) scale(0.8); }
      20% { opacity: 1; transform: translate(-50%, -50%) scale(1.1); }
      80% { opacity: 1; transform: translate(-50%, -50%) scale(1); }
      100% { opacity: 0; transform: translate(-50%, -50%) scale(0.8); }
    }
  `
  document.head.appendChild(style)
  document.body.appendChild(feedback)
  
  // Remove after animation
  setTimeout(() => {
    document.body.removeChild(feedback)
  }, 1500)
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

// Test hotkey function for debugging (commented out to avoid unused variable warning)
// const testHotkey = async (action: string) => {
//   try {
//     await invoke('test_hotkey', { action })
//     console.log('Test hotkey triggered:', action)
//   } catch (error) {
//     console.error('Failed to test hotkey:', error)
//   }
// }

// Toggle click-through functionality
const toggleClickThrough = async () => {
  try {
    await invoke('set_click_through', { enabled: clickThroughEnabled.value })
    console.log('Click-through toggled:', clickThroughEnabled.value)
  } catch (error) {
    console.error('Failed to toggle click-through:', error)
  }
}

// Initialize click-through state
const initClickThrough = async () => {
  try {
    const response = await invoke('get_click_through')
    if (response && typeof response === 'object' && 'data' in response) {
      clickThroughEnabled.value = response.data as boolean
    }
  } catch (error) {
    console.error('Failed to get click-through state:', error)
  }
}

function onModelTypeToggle(event: Event) {
  const target = event.target as HTMLInputElement | null;
  if (!target) return;
  if (settingsStore.subscription === 'max') {
    settingsStore.modelType = target.checked ? 'online' : 'local';
  }
}

// Lifecycle
onMounted(async () => {
  await initOverlay()
  await getOverlayState()
  await setupHotkeyListeners()
  await initClickThrough()
  
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
          @click="executeAction('toggle_mic')"
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
          @click="executeAction('switch_monitor')"
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
        <button @click="executeAction('open_settings')" class="header-btn settings-btn" title="Settings">
          <div class="btn-content-row">
            <span class="btn-action-text">Settings</span>
            <i class="fas fa-cog"></i>
          </div>
          <div class="btn-shortcut">{{ SHORTCUTS.settings }}</div>
        </button>
        <button @click="executeAction('quit')" class="header-btn close-btn" title="Close">
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
          <button @click="executeAction('toggle_overlay')" class="control-btn primary">
            <i class="fas" :class="overlayState.enabled ? 'fa-pause' : 'fa-play'"></i>
            {{ overlayState.enabled ? 'Disable' : 'Enable' }} Overlay
          </button>
          
          <!-- Developer Quit Button -->
          <button @click="executeAction('quit')" class="control-btn quit-btn" style="margin-top: 15px;">
            <i class="fas fa-power-off"></i>
            Quit Application (Dev)
          </button>
          
          <!-- Test buttons for hotkeys -->
          <div class="test-hotkeys" style="margin-top: 20px; padding: 15px; background: rgba(0,0,0,0.05); border-radius: 8px;">
            <h4 style="margin: 0 0 10px 0; color: #666;">Test Hotkeys:</h4>
            <div style="display: flex; gap: 10px; flex-wrap: wrap;">
              <button @click="executeAction('toggle_mic')" class="control-btn secondary">Test Mic Toggle</button>
              <button @click="executeAction('switch_monitor')" class="control-btn secondary">Test Monitor Switch</button>
              <button @click="executeAction('open_settings')" class="control-btn secondary">Test Settings</button>
              <button @click="executeAction('quit')" class="control-btn secondary">Test Quit</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Settings Modal -->
    <div v-if="showSettings" class="settings-modal" @click="closeSettings">
      <div class="settings-content" @click.stop>
        <!-- Header with user profile -->
        <div class="settings-header">
          <div class="header-left">
            <h3>Settings</h3>
          </div>
          <div class="header-right">
            <div class="user-profile">
              <div class="profile-avatar">
                <i class="fas fa-user"></i>
              </div>
              <div class="profile-info">
                <span class="profile-name">TTAWin User</span>
                <span class="profile-status">
                  <span class="subscription-badge max">Max</span>
                  <!-- For demo: change to 'edge' or 'go' for other tiers -->
                </span>
              </div>
            </div>
            <button @click="closeSettings" class="close-settings-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>
        </div>

        <!-- Settings Content -->
        <div class="settings-body">
          <div class="settings-grid">
            <!-- Audio Transcription Toggle -->
            <div class="setting-card">
              <div class="setting-header">
                <i class="fas fa-wave-square"></i>
                <div class="setting-info">
                  <h5>Audio Transcription</h5>
                  <p>Enable or disable real-time audio transcription</p>
                </div>
              </div>
              <div class="setting-control">
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settingsStore.audioTranscription" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>

            <!-- EchoTap Toggle -->
            <div class="setting-card">
              <div class="setting-header">
                <i class="fas fa-broadcast-tower"></i>
                <div class="setting-info">
                  <h5>EchoTap</h5>
                  <p>Allow users on your local network to listen in (EchoTap mode)</p>
                </div>
              </div>
              <div class="setting-control">
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settingsStore.echotap" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>

            <!-- Model Type Toggle (Online/Local) -->
            <div class="setting-card">
              <div class="setting-header">
                <i class="fas fa-microchip"></i>
                <div class="setting-info">
                  <h5>Model Type</h5>
                  <p>
                    <span v-if="settingsStore.modelType === 'online'">Currently using Online mode.</span>
                    <span v-else>Currently using Local mode.</span>
                    <span v-if="settingsStore.subscription !== 'max'" class="model-lock"> (Max only for Online)</span>
                  </p>
                </div>
              </div>
              <div class="setting-control">
                <label class="toggle-switch">
                  <input
                    type="checkbox"
                    :checked="settingsStore.modelType === 'online'"
                    @change="onModelTypeToggle($event)"
                    :disabled="settingsStore.subscription !== 'max'"
                  />
                  <span class="toggle-slider"></span>
                </label>
              </div>
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
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(4px);
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
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  background: rgba(54, 57, 63, 0.95);
  color: #E8E8E8;
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
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.header-btn:hover {
  transform: scale(1.05);
  background: rgba(64, 68, 75, 0.95);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
  border-color: rgba(255, 255, 255, 0.15);
  color: #FFFFFF;
}

.settings-btn:hover {
  background: rgba(64, 68, 75, 0.95);
  color: #7289DA;
}

.close-btn {
  background: rgba(54, 57, 63, 0.95);
  color: #F04747;
  border: 1px solid rgba(255, 255, 255, 0.08);
  font-weight: bold;
  transition: all 0.2s;
}
.close-btn:hover {
  background: rgba(64, 68, 75, 0.95);
  color: #F04747;
  border-color: rgba(255, 255, 255, 0.15);
  box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1);
}

.overlay-content {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  padding: 20px;
}

.overlay-status {
  background: rgba(54, 57, 63, 0.95);
  padding: 40px;
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(20px);
  text-align: center;
  min-width: 300px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.overlay-status h2 {
  margin: 0 0 20px 0;
  color: #E8E8E8;
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
  background: rgba(54, 57, 63, 0.8);
  color: #F04747;
  font-weight: 600;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.status-indicator.active {
  background: rgba(64, 68, 75, 0.8);
  color: #43B581;
  border: 1px solid rgba(255, 255, 255, 0.15);
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
  background: rgba(54, 57, 63, 0.9);
  color: #5865F2;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.control-btn.primary:hover {
  background: rgba(64, 68, 75, 0.9);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  border-color: rgba(255, 255, 255, 0.15);
  color: #7289DA;
}

.control-btn.secondary {
  background: rgba(54, 57, 63, 0.9);
  color: #C8C8C8;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.control-btn.secondary:hover {
  background: rgba(64, 68, 75, 0.9);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  border-color: rgba(255, 255, 255, 0.15);
  color: #E8E8E8;
}

.control-btn.quit-btn {
  background: rgba(54, 57, 63, 0.9);
  color: #F04747;
  border: 1px solid rgba(255, 255, 255, 0.08);
  font-weight: bold;
}

.control-btn.quit-btn:hover {
  background: rgba(64, 68, 75, 0.9);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  border-color: rgba(255, 255, 255, 0.15);
  color: #F04747;
}

/* Settings Modal - Modern Design */
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10001;
}

.settings-content {
  background: rgba(54, 57, 63, 0.98);
  border-radius: 20px;
  min-width: 340px;
  max-width: 420px;
  min-height: 0;
  max-height: 90vh;
  width: auto;
  height: auto;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
  padding: 0;
}

/* Header with User Profile */
.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px 12px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(64, 68, 75, 0.3);
}

.header-left h3 {
  margin: 0 0 4px 0;
  color: #FFFFFF;
  font-size: 24px;
  font-weight: 700;
}

.header-subtitle {
  color: #B9BBBE;
  font-size: 14px;
  font-weight: 400;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-profile {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.profile-avatar {
  width: 32px;
  height: 32px;
  background: #5865F2;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 14px;
}

.profile-info {
  display: flex;
  flex-direction: column;
}

.profile-name {
  color: #FFFFFF;
  font-size: 14px;
  font-weight: 600;
}

.profile-status {
  color: #43B581;
  font-size: 12px;
  font-weight: 500;
}

.close-settings-btn {
  width: 40px;
  height: 40px;
  border: none;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 50%;
  color: #B9BBBE;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.close-settings-btn:hover {
  background: rgba(240, 71, 71, 0.1);
  color: #F04747;
  border-color: rgba(240, 71, 71, 0.3);
}



/* Settings Body */
.settings-body {
  flex: 1;
  padding: 20px 24px 24px 24px;
  overflow-y: auto;
  width: 100%;
  box-sizing: border-box;
}



.section-header {
  margin-bottom: 32px;
}

.section-header h4 {
  margin: 0 0 8px 0;
  color: #FFFFFF;
  font-size: 20px;
  font-weight: 700;
}

.section-header p {
  margin: 0;
  color: #B9BBBE;
  font-size: 14px;
  line-height: 1.5;
}

/* Settings Grid */
.settings-grid {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.setting-card {
  background: rgba(64, 68, 75, 0.3);
  border-radius: 12px;
  padding: 18px 18px 14px 18px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.setting-card:hover {
  background: rgba(64, 68, 75, 0.4);
  border-color: rgba(255, 255, 255, 0.12);
}

.setting-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.setting-header i {
  color: #5865F2;
  font-size: 18px;
  margin-top: 2px;
}

.setting-info h5 {
  margin: 0 0 4px 0;
  color: #FFFFFF;
  font-size: 16px;
  font-weight: 600;
}

.setting-info p {
  margin: 0;
  color: #B9BBBE;
  font-size: 13px;
  line-height: 1.4;
}

.setting-control {
  display: flex;
  align-items: center;
  justify-content: space-between;
}



/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.1);
  transition: 0.3s;
  border-radius: 24px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 2px;
  bottom: 2px;
  background-color: #B9BBBE;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #5865F2;
  border-color: #5865F2;
}

input:checked + .toggle-slider:before {
  transform: translateX(24px);
  background-color: white;
}

/* Status Badge */
.status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
}

.status-badge.active {
  background: rgba(67, 181, 129, 0.2);
  color: #43B581;
  border: 1px solid rgba(67, 181, 129, 0.3);
}

.status-badge.inactive {
  background: rgba(240, 71, 71, 0.2);
  color: #F04747;
  border: 1px solid rgba(240, 71, 71, 0.3);
}

/* Audio Meter */
.audio-meter {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.meter-bar {
  flex: 1;
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
}

.meter-fill {
  height: 100%;
  background: linear-gradient(90deg, #43B581, #5865F2);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.meter-value {
  color: #FFFFFF;
  font-weight: 600;
  font-size: 14px;
  min-width: 40px;
  text-align: right;
}

/* Hotkey Display */
.hotkey-display {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 8px 12px;
  color: #FFFFFF;
  font-family: 'Fira Mono', 'Consolas', monospace;
  font-size: 12px;
  font-weight: 500;
  letter-spacing: 0.5px;
}



.mic-btn {
  position: relative;
}
.mic-on {
  background: rgba(54, 57, 63, 0.95);
  color: #43B581;
}
.mic-off {
  background: rgba(54, 57, 63, 0.95);
  color: #F04747;
}
.monitor-btn {
  background: rgba(54, 57, 63, 0.95);
  color: #5865F2;
  position: relative;
}
.monitor-btn .monitor-number {
  margin-left: 6px;
  position: static;
  background: #5865F2;
  color: #FFFFFF;
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
  color: #A8A8A8;
  font-family: 'Fira Mono', 'Consolas', monospace;
  margin-top: 4px;
  text-align: center;
  width: 100%;
  font-weight: 500;
}
.model-type-options {
  display: flex;
  gap: 20px;
  border-radius: 12px;
  overflow: visible;
  border: none;
  background: none;
  margin: 24px 0 12px 0;
}
.model-type-option {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 18px 0;
  background: rgba(255,255,255,0.04);
  border-radius: 10px;
  border: 2px solid transparent;
  color: #B9BBBE;
  font-size: 18px;
  font-weight: 700;
  cursor: pointer;
  transition: background 0.2s, color 0.2s, border-color 0.2s;
  outline: none;
  position: relative;
  min-width: 120px;
  min-height: 56px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.06);
}
.model-type-option input[type="radio"] {
  display: none;
}
.model-type-option.active {
  background: #5865F2;
  color: #fff;
  border-color: #5865F2;
  z-index: 1;
}
.model-type-option:not(.active):hover:not(.disabled) {
  background: rgba(88,101,242,0.10);
  color: #7289DA;
  border-color: #7289DA;
}
.model-type-option.disabled,
.model-type-option:disabled {
  background: rgba(255,255,255,0.02);
  color: #888;
  cursor: not-allowed;
  opacity: 0.6;
  border-color: transparent;
}
.option-label {
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 0.5px;
}
.model-lock {
  font-size: 11px;
  color: #F04747;
  margin-left: 10px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
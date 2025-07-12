<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useOverlayStore } from './stores/overlayStore'
import { useSettings } from './composables/useSettings'
import { storeToRefs } from 'pinia'
import { listen } from '@tauri-apps/api/event'

const overlayStore = useOverlayStore()
const { micEnabled, monitors, currentMonitorIndex, overlayVisible } = storeToRefs(overlayStore)
const { store: settingsStore } = useSettings()

const showSettings = ref(false)

// Toggle OS-level click-through
const setClickThrough = async (enabled: boolean) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_click_through', { enabled })
  } catch (error) {
    console.error('Failed to set click-through:', error)
  }
}

// Set overlay to hidden state (only controls visible, middle click-through)
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

// Set overlay to visible state (full overlay, no click-through for security)
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

// Watch for modal open/close to toggle click-through
watch(showSettings, (val) => {
  if (val) {
    // Modal is open - disable click-through for security
    setClickThrough(false)
  } else {
    // Modal is closed - restore click-through based on overlay state
    if (overlayVisible.value) {
      setClickThrough(false) // Full overlay mode
    } else {
      setClickThrough(true) // Hidden mode with click-through
    }
  }
})

const openSettings = () => {
  console.log('Opening settings...')
  showSettings.value = true
}

const closeSettings = () => {
  console.log('Closing settings...')
  showSettings.value = false
}

const quitApp = async () => {
  console.log('Quitting app...')
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('quit_app')
  } catch (error) {
    console.error('Failed to quit app:', error)
  }
}

const onModelTypeToggle = (event: Event) => {
  const target = event.target as HTMLInputElement | null;
  if (!target) return;
  if (settingsStore.subscription === 'max') {
    settingsStore.modelType = target.checked ? 'online' : 'local';
  }
}

// Handle hotkey events
const handleHotkeyEvent = async (event: any) => {
  const { action } = event.payload;
  console.log('Hotkey triggered:', action);
  
  switch (action) {
    case 'toggle_overlay':
      if (overlayVisible.value) {
        await setOverlayHidden();
      } else {
        await setOverlayVisible();
      }
      break;
    case 'toggle_mic':
      await overlayStore.toggleMic();
      break;
    case 'switch_monitor':
      await overlayStore.switchMonitor();
      break;
    case 'open_settings':
      openSettings();
      break;
    case 'quit':
      await quitApp();
      break;
  }
}

// Prevent clicks from reaching background when overlay is active
const preventBackgroundClicks = (event: Event) => {
  if (overlayVisible.value || showSettings.value) {
    event.preventDefault();
    event.stopPropagation();
  }
}

let unlistenFn: (() => void) | null = null;

onMounted(async () => {
  // Initialize overlay as hidden and load monitors
  await setOverlayHidden()
  await overlayStore.loadMonitors()
  setClickThrough(true)
  unlistenFn = await listen('hotkey-triggered', handleHotkeyEvent)
  
  // Add global click prevention for security
  document.addEventListener('click', preventBackgroundClicks, true)
  document.addEventListener('mousedown', preventBackgroundClicks, true)
  document.addEventListener('mouseup', preventBackgroundClicks, true)
})

onUnmounted(() => {
  if (unlistenFn) unlistenFn()
  document.removeEventListener('click', preventBackgroundClicks, true)
  document.removeEventListener('mousedown', preventBackgroundClicks, true)
  document.removeEventListener('mouseup', preventBackgroundClicks, true)
})
</script>

<template>
  <div id="overlay" :class="{ 
    'overlay-container': true,
    'interactive': showSettings, 
    'visible': overlayVisible,
    'hidden-mode': !overlayVisible && !showSettings
  }">
    
    <!-- Developer Controls Bar (Top Right) - Always visible in hidden mode -->
    <div class="developer-controls" :class="{ 'always-visible': !overlayVisible }">
      <button @click="overlayStore.toggleMic()" class="dev-btn mic-btn" :class="micEnabled ? 'mic-on' : 'mic-off'" :title="micEnabled ? 'Microphone On' : 'Microphone Off'">
        <i class="fas" :class="micEnabled ? 'fa-microphone' : 'fa-microphone-slash'"></i>
        <span class="dev-btn-text">{{ micEnabled ? 'Mute' : 'Unmute' }}</span>
      </button>
      
      <button @click="openSettings()" class="dev-btn settings-btn" title="Settings">
        <i class="fas fa-cog"></i>
        <span class="dev-btn-text">Settings</span>
      </button>
      
      <button @click="quitApp()" class="dev-btn quit-btn" title="Quit Application">
        <i class="fas fa-times"></i>
        <span class="dev-btn-text">Quit</span>
      </button>
    </div>

    <!-- Developer Info Panel (Center) - Only visible in full overlay mode -->
    <div v-if="overlayVisible" class="developer-panel">
      <div class="dev-panel-content">
        <h2>TTAWin Developer Mode</h2>
        <div class="dev-status">
          <div class="status-item">
            <span class="status-label">Microphone:</span>
            <span class="status-value" :class="micEnabled ? 'status-on' : 'status-off'">
              {{ micEnabled ? 'ON' : 'OFF' }}
            </span>
          </div>
          <div class="status-item">
            <span class="status-label">Monitors:</span>
            <span class="status-value">{{ monitors.length }}</span>
          </div>
          <div class="status-item">
            <span class="status-label">Current Monitor:</span>
            <span class="status-value">{{ currentMonitorIndex + 1 }}</span>
          </div>
        </div>
        <div class="dev-actions">
          <button @click="overlayStore.toggleMic()" class="dev-action-btn">
            {{ micEnabled ? 'Stop Recording' : 'Start Recording' }}
          </button>
          <button @click="overlayStore.switchMonitor()" class="dev-action-btn" v-if="monitors.length > 1">
            Switch Monitor
          </button>
          <button @click="openSettings()" class="dev-action-btn">
            Open Settings
          </button>
          <button @click="quitApp()" class="dev-action-btn quit">
            Quit Application
          </button>
        </div>
      </div>
    </div>

    <!-- Click-through zone (middle area) - Only active in hidden mode -->
    <div v-if="!overlayVisible && !showSettings" class="click-through-zone"></div>

    <!-- Settings Modal -->
    <div v-if="showSettings" class="settings-modal" @click="closeSettings">
      <div class="settings-content" @click.stop>
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
                </span>
              </div>
            </div>
            <button @click="closeSettings" class="close-settings-btn">
              <i class="fas fa-times"></i>
            </button>
          </div>
        </div>
        <div class="settings-body">
          <div class="settings-grid">
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
  background: transparent !important;
  pointer-events: none;
  z-index: 9999;
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.overlay-container.visible {
  opacity: 1;
  pointer-events: auto;
}

.overlay-container.hidden-mode {
  opacity: 1;
  pointer-events: none;
}

.overlay-container.interactive {
  pointer-events: auto;
}

/* Click-through zone for hidden mode */
.click-through-zone {
  position: absolute;
  top: 80px; /* Below the controls */
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none; /* Allow clicks to pass through */
  z-index: -1;
}

/* Developer Controls (Top Right) */
.developer-controls {
  position: absolute;
  top: 20px;
  right: 20px;
  display: flex;
  gap: 12px;
  z-index: 10000;
  pointer-events: auto;
  transition: opacity 0.3s ease;
}

.developer-controls.always-visible {
  opacity: 1;
  background: rgba(0, 0, 0, 0.3);
  padding: 8px;
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

.dev-btn {
  background: rgba(54, 57, 63, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 8px 12px;
  color: #FFFFFF;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
  display: flex;
  align-items: center;
  gap: 6px;
  font-family: 'Inter', sans-serif;
  font-size: 13px;
  font-weight: 500;
  min-width: 80px;
  justify-content: center;
}

.dev-btn:hover {
  background: rgba(64, 68, 75, 0.95);
  border-color: rgba(255, 255, 255, 0.2);
  transform: translateY(-1px);
}

.dev-btn i {
  font-size: 14px;
}

.dev-btn-text {
  font-weight: 500;
}

/* Developer Panel (Center) */
.developer-panel {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 9999;
  pointer-events: auto;
}

.dev-panel-content {
  background: rgba(44, 47, 51, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  padding: 32px;
  backdrop-filter: blur(20px);
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
  min-width: 400px;
  text-align: center;
}

.dev-panel-content h2 {
  color: #FFFFFF;
  margin: 0 0 24px 0;
  font-family: 'Inter', sans-serif;
  font-weight: 600;
  font-size: 24px;
}

.dev-status {
  margin-bottom: 24px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.status-item:last-child {
  border-bottom: none;
}

.status-label {
  color: rgba(255, 255, 255, 0.7);
  font-family: 'Inter', sans-serif;
  font-size: 14px;
}

.status-value {
  color: #FFFFFF;
  font-family: 'Inter', sans-serif;
  font-weight: 600;
  font-size: 14px;
}

.status-on {
  color: #43B581;
}

.status-off {
  color: #F04747;
}

.dev-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dev-action-btn {
  background: rgba(114, 137, 218, 0.2);
  border: 1px solid rgba(114, 137, 218, 0.3);
  border-radius: 8px;
  padding: 12px 16px;
  color: #FFFFFF;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: 'Inter', sans-serif;
  font-size: 14px;
  font-weight: 500;
}

.dev-action-btn:hover {
  background: rgba(114, 137, 218, 0.3);
  border-color: rgba(114, 137, 218, 0.4);
}

.dev-action-btn.quit {
  background: rgba(240, 71, 71, 0.2);
  border-color: rgba(240, 71, 71, 0.3);
}

.dev-action-btn.quit:hover {
  background: rgba(240, 71, 71, 0.3);
  border-color: rgba(240, 71, 71, 0.4);
}

/* Developer button states */
.dev-btn.mic-on {
  background: rgba(67, 181, 129, 0.2);
  border-color: rgba(67, 181, 129, 0.3);
}

.dev-btn.mic-on:hover {
  background: rgba(67, 181, 129, 0.3);
  border-color: rgba(67, 181, 129, 0.4);
}

.dev-btn.mic-off {
  background: rgba(240, 71, 71, 0.2);
  border-color: rgba(240, 71, 71, 0.3);
}

.dev-btn.mic-off:hover {
  background: rgba(240, 71, 71, 0.3);
  border-color: rgba(240, 71, 71, 0.4);
}

.dev-btn.settings-btn {
  background: rgba(114, 137, 218, 0.2);
  border-color: rgba(114, 137, 218, 0.3);
}

.dev-btn.settings-btn:hover {
  background: rgba(114, 137, 218, 0.3);
  border-color: rgba(114, 137, 218, 0.4);
}

.dev-btn.quit-btn {
  background: rgba(240, 71, 71, 0.2);
  border-color: rgba(240, 71, 71, 0.3);
}

.dev-btn.quit-btn:hover {
  background: rgba(240, 71, 71, 0.3);
  border-color: rgba(240, 71, 71, 0.4);
}

.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10001;
  pointer-events: auto;
}

.settings-content {
  background: #2C2F33;
  border-radius: 16px;
  min-width: 340px;
  max-width: 420px;
  padding: 0;
  box-shadow: 0 24px 64px rgba(0,0,0,0.6);
  border: 1px solid rgba(255,255,255,0.1);
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid rgba(255,255,255,0.1);
  background: #36393F;
}

.header-left h3 {
  margin: 0;
  color: #FFF;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.user-profile {
  display: flex;
  align-items: center;
  gap: 12px;
}

.profile-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #7289DA;
  display: flex;
  align-items: center;
  justify-content: center;
}

.profile-info {
  display: flex;
  flex-direction: column;
}

.profile-name {
  font-weight: 600;
  color: #FFF;
}

.profile-status {
  font-size: 12px;
  color: rgba(255,255,255,0.7);
}

.subscription-badge {
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
}

.subscription-badge.max {
  background: #FFD700;
  color: #000;
}

.close-settings-btn {
  background: none;
  border: none;
  color: #FFF;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-settings-btn:hover {
  background: rgba(255,255,255,0.1);
}

.settings-body {
  padding: 20px;
}

.settings-grid {
  display: grid;
  gap: 20px;
  max-width: 600px;
}

.setting-card {
  background: #36393F;
  border-radius: 8px;
  padding: 20px;
  border: 1px solid rgba(255,255,255,0.1);
}

.setting-header {
  display: flex;
  align-items: flex-start;
  gap: 15px;
  margin-bottom: 15px;
}

.setting-header i {
  font-size: 20px;
  color: #7289DA;
  margin-top: 2px;
}

.setting-info h5 {
  margin: 0 0 5px 0;
  color: #FFF;
}

.setting-info p {
  margin: 0;
  color: rgba(255,255,255,0.7);
  font-size: 14px;
}

.setting-control {
  display: flex;
  justify-content: flex-end;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 50px;
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
  background-color: #4F545C;
  transition: 0.3s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #7289DA;
}

input:checked + .toggle-slider:before {
  transform: translateX(26px);
}

.model-lock {
  color: #F04747;
  font-weight: 600;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .developer-controls {
    top: 10px;
    right: 10px;
    gap: 8px;
  }
  
  .dev-btn {
    min-width: 70px;
    padding: 6px 10px;
    font-size: 12px;
  }
}
</style>

<style>
body, html, #app, .overlay-container {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
</style>
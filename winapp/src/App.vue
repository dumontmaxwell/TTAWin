<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useOverlayControls } from './composables/useOverlayControls'
import { useSettings } from './composables/useSettings'
import { storeToRefs } from 'pinia'
import { listen } from '@tauri-apps/api/event'

const { store, toggleMic, switchMonitor } = useOverlayControls()
const { micEnabled, monitors, currentMonitorIndex } = storeToRefs(store)
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

// Watch for modal open/close to toggle click-through
watch(showSettings, (val) => {
  setClickThrough(!val)
})

const openSettings = () => {
  showSettings.value = true
}
const closeSettings = () => {
  showSettings.value = false
}

const quitApp = async () => {
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
  switch (action) {
    case 'toggle_mic':
      await toggleMic();
      break;
    case 'switch_monitor':
      await switchMonitor();
      break;
    case 'open_settings':
      openSettings();
      break;
    case 'quit':
      await quitApp();
      break;
  }
}

let unlistenFn: (() => void) | null = null;

onMounted(async () => {
  setClickThrough(true)
  unlistenFn = await listen('hotkey-triggered', handleHotkeyEvent)
})

onUnmounted(() => {
  if (unlistenFn) unlistenFn()
})
</script>

<template>
  <div id="overlay" :class="{ interactive: showSettings }" class="overlay-container">
    <!-- Action Bar -->
    <div class="action-bar">
      <button @click="toggleMic()" class="action-btn mic-btn" :class="micEnabled ? 'mic-on' : 'mic-off'" :title="micEnabled ? 'Microphone On' : 'Microphone Off'">
        <div class="btn-content-row">
          <span class="btn-action-text">{{ micEnabled ? 'Mute' : 'Unmute' }}</span>
          <i class="fas" :class="micEnabled ? 'fa-microphone' : 'fa-microphone-slash'"></i>
        </div>
        <div class="btn-shortcut">Ctrl+Shift+M</div>
      </button>
      <button v-if="monitors.length > 1" @click="switchMonitor()" class="action-btn monitor-btn" title="Switch Monitor">
        <div class="btn-content-row">
          <span class="btn-action-text">Monitor</span>
          <i class="fas fa-desktop"></i>
          <span class="monitor-number">{{ currentMonitorIndex + 1 }}</span>
        </div>
        <div class="btn-shortcut">Ctrl+Shift+N</div>
      </button>
      <button @click="openSettings()" class="action-btn settings-btn" title="Settings">
        <div class="btn-content-row">
          <span class="btn-action-text">Settings</span>
          <i class="fas fa-cog"></i>
        </div>
        <div class="btn-shortcut">Ctrl+Shift+S</div>
      </button>
      <button @click="quitApp()" class="action-btn quit-btn" title="Quit">
        <div class="btn-content-row">
          <span class="btn-action-text">Quit</span>
          <i class="fas fa-times"></i>
        </div>
        <div class="btn-shortcut">Ctrl+Shift+Q</div>
      </button>
    </div>

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
}
.overlay-container.interactive {
  pointer-events: auto;
}
.action-bar {
  position: absolute;
  top: 20px;
  right: 20px;
  display: flex;
  gap: 10px;
  z-index: 10000;
  pointer-events: auto;
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
</style>
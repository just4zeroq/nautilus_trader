<template>
  <div id="app">
    <header class="app-header">
      <div class="header-brand">
        <div class="logo">
          <svg viewBox="0 0 32 32" fill="none">
            <rect x="2" y="2" width="28" height="28" rx="6" stroke="currentColor" stroke-width="2"/>
            <path d="M8 16h16M16 8v16" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <circle cx="16" cy="16" r="4" fill="currentColor"/>
          </svg>
        </div>
        <div class="brand-text">
          <span class="brand-name">CryptoArb</span>
          <span class="brand-version">v1.0</span>
        </div>
      </div>

      <nav class="header-nav">
        <router-link to="/" class="nav-link" :class="{ active: $route.path === '/' }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7" rx="1"/>
            <rect x="14" y="3" width="7" height="7" rx="1"/>
            <rect x="3" y="14" width="7" height="7" rx="1"/>
            <rect x="14" y="14" width="7" height="7" rx="1"/>
          </svg>
          <span>Dashboard</span>
        </router-link>
        <router-link to="/strategies" class="nav-link" :class="{ active: $route.path === '/strategies' }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
          <span>Strategies</span>
        </router-link>
        <router-link to="/symbols" class="nav-link" :class="{ active: $route.path === '/symbols' }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 6v6l4 2"/>
          </svg>
          <span>Markets</span>
        </router-link>
        <router-link to="/positions" class="nav-link" :class="{ active: $route.path === '/positions' }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12V7a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h7"/>
            <path d="M16 19l2 2 4-4"/>
          </svg>
          <span>Positions</span>
        </router-link>
      </nav>

      <div class="header-status">
        <div class="status-item">
          <span class="status-dot online"></span>
          <span class="status-label">System Online</span>
        </div>
        <div class="time-display">{{ currentTime }}</div>
      </div>
    </header>

    <main class="app-main">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const currentTime = ref('')
let timeInterval: number

function updateTime() {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

onMounted(() => {
  updateTime()
  timeInterval = window.setInterval(updateTime, 1000)
})

onUnmounted(() => {
  clearInterval(timeInterval)
})
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

:root {
  --bg-primary: #0a0e14;
  --bg-secondary: #111822;
  --bg-tertiary: #1a2232;
  --border-color: rgba(56, 139, 253, 0.15);
  --text-primary: #e6edf3;
  --text-secondary: #7d8590;
  --text-tertiary: #484f58;
  --accent-cyan: #00d9ff;
  --accent-green: #00ff88;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  background: var(--bg-primary);
  color: var(--text-primary);
}

#app {
  font-family: 'Outfit', -apple-system, BlinkMacSystemFont, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background: var(--bg-primary);
  min-height: 100vh;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 40px;
  height: 64px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo {
  width: 36px;
  height: 36px;
  color: var(--accent-cyan);
}

.logo svg {
  width: 100%;
  height: 100%;
}

.brand-text {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.brand-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.5px;
}

.brand-version {
  font-size: 10px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-tertiary);
}

.header-nav {
  display: flex;
  align-items: center;
  gap: 4px;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 8px;
  text-decoration: none;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.nav-link svg {
  width: 18px;
  height: 18px;
}

.nav-link:hover {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.05);
}

.nav-link.active {
  color: var(--accent-cyan);
  background: rgba(0, 217, 255, 0.1);
}

.header-status {
  display: flex;
  align-items: center;
  gap: 20px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-green);
  box-shadow: 0 0 10px var(--accent-green);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.status-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.time-display {
  font-family: 'JetBrains Mono', monospace;
  font-size: 14px;
  color: var(--text-primary);
  background: var(--bg-tertiary);
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.app-main {
  min-height: calc(100vh - 64px);
}

/* Reset Element Plus styles that conflict */
.el-container {
  background: var(--bg-primary);
}

.el-header {
  display: none !important;
}

.el-main {
  padding: 0 !important;
}
</style>

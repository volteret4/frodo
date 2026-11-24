<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import Sidebar from '$lib/Sidebar.svelte';
  import { appState, loadServices } from '$lib/store.svelte';

  let unlisten: (() => void) | undefined;

  onMount(async () => {
    await loadServices();
    const win = getCurrentWindow();
    unlisten = await win.onResized(async () => {
      await invoke('update_webview_bounds');
    });
  });

  onDestroy(() => unlisten?.());
</script>

<div class="app">
  <Sidebar />

  {#if !appState.activeId}
    <div class="home">
      <div class="logo">🧙</div>
      <h1>Frodo</h1>
      <p>Select a service on the left, or add one with <strong>+</strong></p>
    </div>
  {/if}
</div>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(html, body) {
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: #111120;
    font-family: system-ui, -apple-system, sans-serif;
  }

  .app {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  .home {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    color: #60607a;
    user-select: none;
  }

  .logo {
    font-size: 64px;
    margin-bottom: 8px;
    filter: grayscale(0.3);
  }

  h1 {
    font-size: 28px;
    font-weight: 700;
    color: #9090b0;
    letter-spacing: -0.5px;
  }

  p {
    font-size: 14px;
    color: #50506a;
  }

  strong {
    color: #7070a0;
  }
</style>

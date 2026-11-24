<script lang="ts">
  import { appState, activateService, removeService } from './store.svelte';
  import { getIcon } from './presets';
  import AddModal from './AddModal.svelte';

  let showModal = $state(false);
  let contextMenu = $state<{ x: number; y: number; serviceId: string } | null>(null);
  let modalPreviousId = $state<string | null>(null);

  function toggle(id: string) {
    activateService(appState.activeId === id ? null : id);
  }

  async function openModal() {
    modalPreviousId = appState.activeId;
    if (appState.activeId !== null) await activateService(null);
    showModal = true;
  }

  async function closeModal() {
    showModal = false;
    if (modalPreviousId !== null) await activateService(modalPreviousId);
    modalPreviousId = null;
  }

  function onContextMenu(e: MouseEvent, id: string) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, serviceId: id };
  }

  function closeContext() {
    contextMenu = null;
  }

  async function deleteService(id: string) {
    contextMenu = null;
    if (appState.activeId === id) await activateService(null);
    await removeService(id);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<nav class="sidebar" onmousedown={closeContext}>
  {#each appState.services as service (service.id)}
    <button
      class="svc"
      class:active={appState.activeId === service.id}
      style="--c: {service.color}"
      title={service.name}
      onclick={() => toggle(service.id)}
      oncontextmenu={(e) => onContextMenu(e, service.id)}
    >
      {getIcon(service.preset)}
    </button>
  {/each}

  <div class="spacer"></div>

  <button class="add" title="Add service" onclick={openModal}>+</button>
</nav>

{#if contextMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px"
    onmousedown={(e) => e.stopPropagation()}
  >
    <button onclick={() => deleteService(contextMenu!.serviceId)}>Remove service</button>
  </div>
{/if}

{#if showModal}
  <AddModal onclose={closeModal} />
{/if}

<style>
  .sidebar {
    width: 64px;
    min-width: 64px;
    height: 100%;
    background: #0d0d1a;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px 0;
    gap: 4px;
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
    z-index: 50;
    border-right: 1px solid rgba(255, 255, 255, 0.06);
  }

  .svc {
    width: 46px;
    height: 46px;
    border-radius: 12px;
    border: none;
    background: rgba(255, 255, 255, 0.05);
    cursor: pointer;
    font-size: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s, border-radius 0.15s;
    flex-shrink: 0;
    position: relative;
  }

  .svc::before {
    content: '';
    position: absolute;
    left: -6px;
    top: 50%;
    transform: translateY(-50%) scaleY(0);
    width: 3px;
    height: 60%;
    background: var(--c);
    border-radius: 0 3px 3px 0;
    transition: transform 0.15s;
  }

  .svc:hover {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 16px;
  }

  .svc:hover::before {
    transform: translateY(-50%) scaleY(0.5);
  }

  .svc.active {
    background: color-mix(in srgb, var(--c) 25%, #1a1a2e);
    border-radius: 16px;
  }

  .svc.active::before {
    transform: translateY(-50%) scaleY(1);
  }

  .spacer {
    flex: 1;
  }

  .add {
    width: 46px;
    height: 46px;
    border-radius: 50%;
    border: 2px dashed rgba(255, 255, 255, 0.25);
    background: transparent;
    cursor: pointer;
    font-size: 22px;
    color: rgba(255, 255, 255, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
    margin-bottom: 6px;
    flex-shrink: 0;
    line-height: 1;
    padding: 0;
  }

  .add:hover {
    border-color: rgba(255, 255, 255, 0.6);
    color: rgba(255, 255, 255, 0.8);
  }

  .context-menu {
    position: fixed;
    background: #1e1e2e;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 4px;
    z-index: 2000;
    min-width: 140px;
  }

  .context-menu button {
    width: 100%;
    padding: 8px 12px;
    background: none;
    border: none;
    color: #f87171;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
    border-radius: 6px;
  }

  .context-menu button:hover {
    background: rgba(248, 113, 113, 0.1);
  }
</style>

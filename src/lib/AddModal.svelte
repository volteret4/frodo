<script lang="ts">
  import { PRESETS } from './presets';
  import { addService } from './store.svelte';
  import type { ServicePreset } from './types';

  let { onclose }: { onclose: () => void } = $props();

  let step = $state<'pick' | 'config'>('pick');
  let selected = $state<ServicePreset | null>(null);
  let name = $state('');
  let url = $state('');
  let error = $state('');

  function pickPreset(preset: ServicePreset) {
    selected = preset;
    name = preset.name;
    url = preset.defaultUrl;
    error = '';
    step = 'config';
  }

  function back() {
    step = 'pick';
    selected = null;
    name = '';
    url = '';
    error = '';
  }

  async function submit() {
    error = '';
    if (!name.trim()) { error = 'Name is required'; return; }
    if (!url.trim()) { error = 'URL is required'; return; }
    try { new URL(url); } catch { error = 'Enter a valid URL'; return; }

    await addService({
      id: crypto.randomUUID(),
      name: name.trim(),
      url: url.trim(),
      color: selected!.color,
      preset: selected!.id === 'custom' ? undefined : selected!.id,
    });
    onclose();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onmousedown={onclose}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal" onmousedown={(e) => e.stopPropagation()}>
    {#if step === 'pick'}
      <h2>Add service</h2>
      <div class="grid">
        {#each PRESETS as preset}
          <button
            class="preset-btn"
            style="--c: {preset.color}"
            onclick={() => pickPreset(preset)}
          >
            <span class="icon">{preset.icon}</span>
            <span class="label">{preset.name}</span>
          </button>
        {/each}
      </div>
    {:else if selected}
      <button class="back" onclick={back}>← Back</button>
      <h2>
        <span>{selected.icon}</span>
        {selected.name}
      </h2>

      <label>
        Name
        <input bind:value={name} placeholder="Display name" />
      </label>

      <label>
        URL
        <input
          bind:value={url}
          placeholder="https://..."
          readonly={!selected.needsUrl && url !== ''}
        />
      </label>

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <button class="submit" onclick={submit}>Add</button>
    {/if}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #1e1e2e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 14px;
    padding: 28px;
    width: 420px;
    max-height: 80vh;
    overflow-y: auto;
  }

  h2 {
    margin: 0 0 20px;
    color: #e0e0f0;
    font-size: 18px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }

  .preset-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 12px 6px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.04);
    cursor: pointer;
    transition: all 0.15s;
  }

  .preset-btn:hover {
    background: color-mix(in srgb, var(--c) 20%, transparent);
    border-color: color-mix(in srgb, var(--c) 50%, transparent);
  }

  .icon {
    font-size: 24px;
  }

  .label {
    font-size: 11px;
    color: #a0a0b8;
    text-align: center;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: #a0a0b8;
    font-size: 13px;
    margin-bottom: 14px;
  }

  input {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 9px 12px;
    color: #e0e0f0;
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s;
  }

  input:focus {
    border-color: rgba(255, 255, 255, 0.3);
  }

  .error {
    color: #f87171;
    font-size: 13px;
    margin: -6px 0 10px;
  }

  .submit {
    width: 100%;
    padding: 10px;
    border-radius: 8px;
    border: none;
    background: #5865f2;
    color: white;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s;
    margin-top: 4px;
  }

  .submit:hover {
    opacity: 0.85;
  }

  .back {
    background: none;
    border: none;
    color: #a0a0b8;
    cursor: pointer;
    font-size: 13px;
    padding: 0;
    margin-bottom: 14px;
  }

  .back:hover {
    color: #e0e0f0;
  }
</style>

import { invoke } from '@tauri-apps/api/core';
import type { Service } from './types';

export const appState = $state({
  services: [] as Service[],
  activeId: null as string | null,
});

export async function loadServices(): Promise<void> {
  appState.services = await invoke<Service[]>('get_services');
}

export async function addService(service: Service): Promise<void> {
  await invoke('add_service', { service });
  appState.services = [...appState.services, service];
}

export async function removeService(id: string): Promise<void> {
  await invoke('remove_service', { id });
  appState.services = appState.services.filter((s) => s.id !== id);
  if (appState.activeId === id) {
    appState.activeId = null;
  }
}

export async function activateService(id: string | null): Promise<void> {
  await invoke('activate_service', { id });
  appState.activeId = id;
}

export async function updateBounds(): Promise<void> {
  await invoke('update_webview_bounds');
}

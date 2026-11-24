import type { ServicePreset } from './types';

export const PRESETS: ServicePreset[] = [
  {
    id: 'whatsapp',
    name: 'WhatsApp',
    defaultUrl: 'https://web.whatsapp.com',
    color: '#25D366',
    needsUrl: false,
    icon: '💬',
  },
  {
    id: 'telegram',
    name: 'Telegram',
    defaultUrl: 'https://web.telegram.org',
    color: '#229ED9',
    needsUrl: false,
    icon: '✈️',
  },
  {
    id: 'discord',
    name: 'Discord',
    defaultUrl: 'https://discord.com/app',
    color: '#5865F2',
    needsUrl: false,
    icon: '🎮',
  },
  {
    id: 'freshrss',
    name: 'FreshRSS',
    defaultUrl: '',
    color: '#FF6600',
    needsUrl: true,
    icon: '📰',
  },
  {
    id: 'wallabag',
    name: 'Wallabag',
    defaultUrl: '',
    color: '#7B4E97',
    needsUrl: true,
    icon: '📚',
  },
  {
    id: 'audiobookshelf',
    name: 'Audiobookshelf',
    defaultUrl: '',
    color: '#C3762E',
    needsUrl: true,
    icon: '🎧',
  },
  {
    id: 'grafana',
    name: 'Grafana',
    defaultUrl: '',
    color: '#F46800',
    needsUrl: true,
    icon: '📊',
  },
  {
    id: 'custom',
    name: 'Custom',
    defaultUrl: '',
    color: '#6B7280',
    needsUrl: true,
    icon: '🌐',
  },
];

export function getPreset(id: string | undefined): ServicePreset | undefined {
  return PRESETS.find((p) => p.id === id);
}

export function getIcon(presetId: string | undefined): string {
  return getPreset(presetId)?.icon ?? '🌐';
}

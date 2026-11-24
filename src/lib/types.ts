export interface Service {
  id: string;
  name: string;
  url: string;
  color: string;
  preset?: string;
}

export interface ServicePreset {
  id: string;
  name: string;
  defaultUrl: string;
  color: string;
  needsUrl: boolean;
  icon: string;
}

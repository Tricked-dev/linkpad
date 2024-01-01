import { ws } from "./ws.svelte";

interface Icon {
  id: string;
  url: string;
}

let icons = $state<Icon[]>([]);

export function upsertIcon(id: string, url: string) {
  let oldIcons = [...icons.filter((x) => x.id != id)];
  icons.length = 0;
  icons.push(...oldIcons, { id, url });

  console.log(icons);
}

export const getIcon = (id: string) => {
  let icon = $derived(icons.find((x) => x.id == id));
  return icon;
};

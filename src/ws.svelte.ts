import type { Info } from "./types";
import { upsertIcon } from "./icons.svelte";

export type State = "OPEN" | "CLOSED";

// interface WsInfo {
//   instance?: WebSocket;
//   connect(): Promise<void>;
//   send(type: string, data: any, params?: any): Promise<void>;
//   get state(): State;
//   on: {
//     data: (data: any) => unknown;
//     iconData: (data: any) => unknown;
//   };
// }

const b64toBlob = (b64Data: string, contentType = "", sliceSize = 512) => {
  const byteCharacters = atob(b64Data);
  const byteArrays = [];

  for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
    const slice = byteCharacters.slice(offset, offset + sliceSize);

    const byteNumbers = new Array(slice.length);
    for (let i = 0; i < slice.length; i++) {
      byteNumbers[i] = slice.charCodeAt(i);
    }

    const byteArray = new Uint8Array(byteNumbers);
    byteArrays.push(byteArray);
  }

  const blob = new Blob(byteArrays, { type: contentType });
  return blob;
};

let state = $state<State>("CLOSED");
let actions = $state<{ name: string; id: string }[]>([]);
let info = $state<Info>([]);
let active = $state<string>("");

export const ws = {
  instance: undefined as WebSocket | undefined,
  actions: actions,
  get info() {
    return info;
  },
  get state() {
    return state;
  },
  get active() {
    return active;
  },
  async connect() {
    const socket = new WebSocket("ws://127.0.0.1:8000/connect");
    ws.instance = socket;
    socket.onmessage = (m) => {
      const res = JSON.parse(m.data);
      console.log(res.type);
      if (res.type == "Data") {
        ws.on.data(res);
      } else if (res.type == "IconData") {
        console.log("IconData");
        ws.on.iconData(res);
      } else if (res.type == "Actions") {
        ws.on.actions(res);
      }
    };
    socket.addEventListener("close", () => {
      state = "CLOSED";
    });

    ws.on.iconData = (event: any) => {
      const url = URL.createObjectURL(
        b64toBlob(event.data.data, event.data.content_type),
      );
      upsertIcon(event.data.id, url);
    };

    return new Promise<void>((res) => {
      socket.addEventListener("open", () => {
        state = "OPEN";
        res();
      });
    });
  },
  async send(type: string, data: any, params: any = {}) {
    if (this.instance?.readyState == WebSocket.CLOSED) await this.connect();
    ws.instance?.send(JSON.stringify({ type, data, ...params }));
  },
  async save() {
    ws.send("UpdatePanels", info);
  },
  on: {
    data: (data: any) => {
      info = data.data;
      // console.log("info", { ...info });
      active = data.data[0].id;
    },
    iconData: (data: any) => {},
    actions: (data: any) => {
      ws.actions = data.data;
    },
  },
};

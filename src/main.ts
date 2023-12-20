import "ato-ui/themes/ato.css";
import "virtual:uno.css";
import "@unocss/reset/tailwind.css";
import "@fontsource/lato";

import App from "./App.svelte";
import { createRoot } from "svelte";

const app = createRoot(App, { target: document.getElementById("app") });

export default app;

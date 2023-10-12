import "./styles.css";
import App from "./App.svelte";
import { attachConsole } from "tauri-plugin-log-api";

attachConsole();

const app = new App({
  target: document.getElementById("app"),
});

export default app;

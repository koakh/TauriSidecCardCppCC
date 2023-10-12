<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Command } from "@tauri-apps/api/shell";
  import { info } from "tauri-plugin-log-api";

  let name = "";
  let greetMsg = "";

  async function greet() {
    info("frontend: greet() -> invoke greet");
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function runSIdeCard() {
    info("frontend: greet() -> invoke binaries/app");

    // alternatively, use `window.__TAURI__.shell.Command`
    // `binaries/app` is the EXACT value specified on `tauri.conf.json > tauri > bundle > externalBin`
    const command = Command.sidecar("binaries/app");
    const output = await command.execute();
    console.log(`output: [${JSON.stringify(output, undefined, 2)}]`);
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>

    <button on:click={runSIdeCard}> SideCard </button>
  </div>
  <p>{greetMsg}</p>
</div>

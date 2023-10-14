<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Button from "./ui/Button.svelte";
  import Input from "./ui/Input.svelte";

  export let minecraftPath = "";
  export let valid: boolean | undefined;

  onMount(async () => {
    minecraftPath = await invoke("default_path");
  });

  async function openPathPicker() {
    const defaultPath = await invoke("default_path");
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: defaultPath as string,
    });

    minecraftPath = (selected as string | null) || ""; // since multiple = false, selected can't be an array
  }

  async function validatePath(path: string) {
    valid = await invoke("validate_path", { path });
  }

  $: validatePath(minecraftPath);
</script>

<div class="flex flex-row w-[80%] mx-auto">
  <Input bind:value={minecraftPath} />
  <Button variant="secondary" on:click={openPathPicker}>Choose location</Button>
</div>

{#if valid}
  <p class="text-green-500">OK path</p>
{:else}
  <p class="text-red-500">Invalid path</p>
{/if}

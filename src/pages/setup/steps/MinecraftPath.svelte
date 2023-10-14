<script lang="ts">
  import MinecraftPathInput from "../../../lib/components/MinecraftPathInput.svelte";
  import { stores } from "../../../lib/database/Store";
  export let enableNext: boolean | undefined;

  let minecraftPath: string | undefined;

  export function onNext() {
    if (enableNext && minecraftPath) {
      stores.settings.set("minecraftPath", minecraftPath);
    }
  }
</script>

<p>To setup the modpack manager, define where minecraft is located</p>
<MinecraftPathInput bind:valid={enableNext} bind:minecraftPath />

<script lang="ts">
  import MinecraftPathInput from "../../../lib/components/MinecraftPathInput.svelte";
  import { stores } from "../../../lib/database/Store";

  let input: MinecraftPathInput;
  export function onNext() {
    if (input.valid) {
      stores.settings.set("minecraftPath", input.minecraftPath);
    }
  }
</script>

<p>To setup the modpack manager, define where minecraft is located</p>
<MinecraftPathInput bind:this={input} />

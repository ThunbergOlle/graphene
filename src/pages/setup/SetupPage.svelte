<script lang="ts">
  import Button from "../../lib/components/ui/Button.svelte";
  import MinecraftPath from "./steps/MinecraftPath.svelte";
  import MinecraftProfile from "./steps/MinecraftProfile.svelte";

  let steps = [MinecraftPath, MinecraftProfile];

  let currentStep = 0;
  let currentStepComponent: { onNext: () => void };

  function onNextStep() {
    if (currentStep < steps.length) {
      currentStepComponent.onNext();
      currentStep++;
      return;
    }
    done();
  }
  function done() {
    console.log("done");
  }
</script>

<div class="flex flex-col justify-between min-h-full text-center">
  <h1 class="text-2xl mt-8">Graphene Modpack setup</h1>
  <div class="flex-1">
    {#each steps as Step, i}
      {#if i === currentStep}
        <div class="flex flex-col items-center">
          <Step bind:this={currentStepComponent} />
        </div>
      {/if}
    {/each}
  </div>
  <div class="m-4 flex flex-row justify-between">
    <Button
      disabled={currentStep === 0}
      variant="secondary"
      on:click={() => currentStep--}>Previous</Button
    >

    <Button variant="primary" on:click={onNextStep}
      >{currentStep === steps.length - 1 ? "Done" : "Next"}</Button
    >
  </div>
</div>

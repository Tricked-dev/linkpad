<script lang="ts">
  import { onMount } from "svelte";
  import Dialog from "./Dialog.svelte";
  import Button from "./Button.svelte";

  const css =
    "rounded-24px bg-surface-200/30 backdrop-blur-4 p-4 w-160px h-160px text-3xl font-bold text-white hover:bg-surface-200/50 duration-200";

  const edit = true;
  let open = $state(true);
  let checked = $state(false);
</script>

<main
  class="[font-family:'Lato'] [background-image:url('/bg3.png')] min-h-screen"
  data-theme="ato"
>
  <div class="bg-surface-900/20 backdrop-blur-50 p-4 text-surface-100 flex">
    <span class="text text-3xl">LINK PAD</span>
    {#if edit}<span class="ml-auto mt-auto">Edit mode</span>{/if}
  </div>

  <div class="p-4 flex flex-col gap-4">
    <div class="flex gap-4">
      {@render button("Button 1")}
      {@render button("Button 2")}
      {@render button("Button 3")}
      {@render button("Button 4")}
      {@render button("Button 5")}
    </div>
    <div class="flex gap-4">
      {@render button("Action 1")}
      {@render button("Cool BTN")}
      {@render button("Text")}
    </div>
    <div class="flex gap-4">
      {@render button("Action 1")}
      {@render button("Cool BTN")}
      {@render button("Cool BTN")}
      {@render button("Text")}
      {#if edit}<button class={`${css} border-tertiary-500 border-4`}>
          <div class="i-mdi-plus text-9xl"></div>
        </button>{/if}
    </div>
  </div>
  <Dialog bind:open title="Edit Button! ">
    <div class="h-110 w-100">
      <div class="flex gap-2">
        <span>ICON</span>
        <label class="relative inline-flex items-center cursor-pointer">
          <input type="checkbox" value="" class="sr-only peer" bind:checked />
          <div
            class="w-11 h-6 bg-surface-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-200 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-red-100 after:border-surface-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
          ></div>
        </label>
        <span>TEXT</span>
      </div>
      {#if checked}
        <div class="p-2">
          <input
            type="text"
            name="brand"
            id="brand"
            class="bg-surface-200 border border-surface-300 text-surface-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 placeholder-surface-400"
            placeholder="Product brand"
          />
        </div>
      {:else}
        <div class="p-2">
          <Button text="Select Icon" />
        </div>
      {/if}
      <span class="text-xl font-bold"> Select Action </span>
      <div class="p-2">
        <Button text="Browse" />
        <span> Increase Volume </span>
      </div>
      <div class="flex justify-center items-center mt-10">
        {@render button("Action 1")}
      </div>
    </div>
  </Dialog>
</main>

{#snippet button(text)}
  <button class={css}>
    {#if edit}<div
        onclick={() => (open = true)}
        class="i-mdi-gear text-2xl absolute top-2 right-2 rounded-lg hover:bg-primary-500"
      ></div>{/if}
    {text}
  </button>
{/snippet}

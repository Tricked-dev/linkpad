<script lang="ts">
  import type { Fragment } from "svelte/compiler";
  import { fade } from "svelte/transition";

  let {
    open,
    children,
    duration = 150,
    title,
  } = $props<{ open: boolean; duration?: number; children: any; title: any }>();

  function close() {
    open = false;
  }
</script>

{#if open}
  <dialog
    onclose={close}
    {open}
    class="flex justify-center"
    transition:fade={{ duration }}
  >
    <div
      class="absolute backdrop-blur-10 bg-surface-200/5 p-20 fixed inset-0 grid place-content-center"
      onclick={close}
    >
      <div
        class="w-full max-w-lg bg-surface-900/90 p-4 shadow-lg text-white rounded-lg"
        onclick={(e) => e.stopPropagation()}
      >
        <div class="flex mb-5 border-surface-300 border-b-3 pb-2">
          {#if typeof title == "object"}
            {@render title()}
          {:else if title}
            <h1 class="text-3xl font-bold">{title}</h1>
          {/if}

          <button
            class="ml-auto i-mdi-close text-2xl my-auto hover:bg-surface-300 p-2 duration-300"
            onclick={close}>Close</button
          >
        </div>
        {@render children()}
      </div>
    </div>
  </dialog>
{/if}

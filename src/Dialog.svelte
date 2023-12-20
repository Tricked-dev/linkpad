<script lang="ts">
  import { fade } from "svelte/transition";

  let { open, children, duration=150 } = $props<{ open: boolean, duration?: number, children:any}>();

  function close() {
    open = false;
  }
</script>

{#if open}
  <dialog onclose={close} {open} class="flex justify-center" transition:fade={{ duration }}>
    <div class="absolute backdrop-blur-10 bg-surface-200/5 p-20 fixed inset-0 grid place-content-center" onclick={close}>
        <div class="w-full max-w-lg bg-surface-500 p-4 shadow-lg text-white rounded-lg" onclick={(e) =>e.stopPropagation()}>
            {@render children()}
        </div>
    </div>
  </dialog>
{/if}

<script lang="ts">
  import Dialog from "./Dialog.svelte";

  const css = "rounded-24px bg-surface-200/30 backdrop-blur-4 p-4 w-160px h-160px text-3xl font-bold text-white hover:bg-surface-200/50 duration-200"

  const edit = true
  let open = $state(false);
</script>

<main
  class="[font-family:'Lato'] [background-image:url('/bg3.png')] min-h-screen"
  data-theme="ato"
>
  <div class="bg-surface-200/50 backdrop-blur-50 p-4 text-white flex">
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
            <div class="i-mdi-plus text-9xl">

            </div>
        </button>{/if}
    </div>
  </div>
<Dialog bind:open={open} >
    <div class="h-20 w-70">
    Editiing!

    </div>
  </Dialog>
</main>

{#snippet button(text)}
    <button class={css}>
      {#if edit}<div onclick={() => open = true} class="i-mdi-gear text-2xl absolute top-2 right-2 rounded-lg hover:bg-primary-500"></div>{/if}
        {text}
    </button>
{/snippet}
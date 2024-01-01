<script lang="ts">
  import { crossfade, fade, fly, blur, scale } from "svelte/transition";
  import { DisplayType } from "./types";
  import { getIcon } from "./icons.svelte";

  let {
    data,
    edit,
    onEdit,
    preview = false,
  } = $props<{
    data: any;
    edit: boolean;
    onEdit: () => void;
    preview?: boolean;
  }>();
  let container: HTMLButtonElement | undefined = $state();
  let txt: HTMLSpanElement | undefined = $state();

  function isOverflowing() {
    console.log(txt!.scrollWidth, txt!.clientWidth);
    return (
      txt!.scrollWidth > container!.clientWidth ||
      txt!.scrollHeight > container!.clientHeight
    );
  }

  function adjustFontSize() {
    if (!txt) return;
    let defaultFontSize = 30;

    do {
      console.log(defaultFontSize);
      defaultFontSize--;
      txt.style.fontSize = `${defaultFontSize}cqi`;
    } while (isOverflowing() && defaultFontSize > 5);
  }

  let text = $derived(data.text);

  $effect(() => {
    if (data.autoSize && text) {
      adjustFontSize();
    }
  });
</script>

<button
  bind:this={container}
  class="rounded-24px bg-surface-200/30 backdrop-blur-4 w-160px h-160px font-bold text-white hover:bg-surface-200/50 duration-200 text-center overflow-hidden flex [container-type:inline-size]"
  id="{data.id}-btn"
>
  {#if edit}<div
      onclick={onEdit}
      class="i-mdi-gear text-2xl absolute top-2 right-2 rounded-lg hover:bg-primary-500"
    ></div>{/if}

  {#if data.type === DisplayType.TEXT}
    {#key data.text}
      <div
        bind:this={txt}
        in:scale={{ duration: preview ? 0 : 500, delay: preview ? 0 : 200 }}
        class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
      >
        {#if data.autoSize}
          <span class="text-center m-auto w-full">
            {data.text}
          </span>
        {:else}
          <span
            style:font-size="{data.textSize}px"
            style:line-height="{data.textSize}px"
            class="text-center my-auto mx-auto"
          >
            {data.text}
          </span>
        {/if}
      </div>
    {/key}
  {:else if data.type === DisplayType.ICON}
    <img
      in:scale={{ duration: preview ? 0 : 500, delay: preview ? 0 : 200 }}
      src={getIcon(data.id)?.url}
      alt="No icon loaded"
      class="w-full h-full object-contain cursor-events-none"
    />
  {/if}
</button>

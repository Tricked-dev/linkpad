<script lang="ts">
  let { data, css, edit, open } = $props<{
    data: any;
    css: string;
    edit: boolean;
    open: boolean;
  }>();

  let txt: HTMLSpanElement | undefined = $state();

  function isOverflowing() {
    return (
      txt!.scrollWidth > txt!.clientWidth ||
      txt!.scrollHeight > txt!.clientHeight
    );
  }

  function adjustFontSize() {
    if (!txt) return;
    let defaultFontSize = 30;

    do {
      defaultFontSize--;
      txt.style.fontSize = `${defaultFontSize}cqi`;
    } while (isOverflowing() && defaultFontSize > 10);
  }

  let text = $derived(data.text);

  $effect(() => {
    if (data.autoSize && text) {
      adjustFontSize();
    }
  });
</script>

<button
  class="{css} overflow-hidden flex [container-type:inline-size]"
  id="{data.id}-btn"
>
  {#if edit}<div
      onclick={() => (open = true)}
      class="i-mdi-gear text-2xl absolute top-2 right-2 rounded-lg hover:bg-primary-500"
    ></div>{/if}
  {#if data.autoSize}
    <span bind:this={txt} class="text-center m-auto w-full" id="{data.id}-txt">
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
</button>

<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  let {
    class: clazz,
    icon,
    children,
    text = "",
    ...rest
  } = $props<HTMLButtonAttributes & { icon?: string; text?: string }>();
</script>

<button
  class="{clazz} btn btn-surface bg-surface-600/50 font-bold {icon
    ? 'i-mdi-' + icon
    : ''}"
  {...rest}
>
  {text}
  {#if children}{@render children()}{/if}
</button>

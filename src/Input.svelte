<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  let {
    value,
    class: clazz,
    ...props
  } = $props<HTMLInputAttributes & { value: string }>();
</script>

<input
  type="text"
  bind:value
  class="{clazz} bg-surface-500 border border-primary-300 text-surface-50 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 placeholder-primary-400"
  {...props}
/>

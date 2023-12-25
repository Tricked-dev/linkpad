<script lang="ts">
  let { checked } = $props<{ checked: boolean }>();
</script>

<label class="relative inline-flex items-center cursor-pointer">
  <input type="checkbox" value="" class="sr-only peer" bind:checked />
  <div
    class="w-11 h-6 bg-surface-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-200 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-red-100 after:border-surface-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
  ></div>
</label>

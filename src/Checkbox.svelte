<script lang="ts">
  let { checked, label, id } = $props<{
    checked: boolean;
    label: string;
    id: string;
  }>();
</script>

<div class="flex items-center mb-4 py-2">
  <input
    {id}
    type="checkbox"
    value=""
    bind:checked
    class="w-4 h-4 text-primary-600 bg-surface-600 border-surface-300 rounded focus:ring-primary-100 focus:ring-2"
  />
  <label for={id} class="ms-2 text-sm font-medium text-surface-50"
    >{label}</label
  >
</div>

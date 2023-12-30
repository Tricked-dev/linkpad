<script lang="ts">
  import Dialog from "./Dialog.svelte";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Toggle from "./Toggle.svelte";
  import Checkbox from "./Checkbox.svelte";
  import ActionButton from "./ActionButton.svelte";
  import IconPicker from "./IconPicker.svelte";
  import { DisplayType, type AnyButton } from "./types";
  import { untrack } from "svelte";

  interface Props {
    onSubmit: () => void;
    onCancel: () => void;
    open: boolean;
    data: AnyButton;
  }

  let { onSubmit, onCancel, data, open } = $props<Props>();

  let checked = $state(false);

  $effect(() => {
    if (checked) {
      data.type = DisplayType.ICON;
    } else {
      data.type = DisplayType.TEXT;
    }
  });

  $effect(() => {
    // open changes to false while the submitted state is false
    if (!open && !submitted) {
      onCancel();
    }
  });

  let submitted = $state(false);
</script>

<Dialog bind:open title="Edit Button! ">
  <div class="h-110 w-100">
    <div class="flex gap-2">
      <span>ICON</span>
      <Toggle bind:checked />
      <span>TEXT</span>
    </div>
    <IconPicker />
    {#if data.type === DisplayType.ICON}
      <div class="p-2 grid grid-cols-2 gap-2">
        <Input
          class="col-span-2"
          name="buttontext"
          placeholder="Button Text"
          bind:value={data.text}
        />
        <Checkbox id="autosize" bind:checked={data.autoSize} label="Autosize" />
        {#if !data.autoSize}
          <label>
            Text Size
            <input type="range" bind:value={data.textSize} min="10" max="200" />
          </label>
        {/if}
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
      <ActionButton {data} onEdit={() => {}} preview edit={false} />
    </div>

    <button
      on:click={() => {
        untrack(() => {
          submitted = true;
        });
        onSubmit();
        open = false;
        untrack(() => {
          submitted = false;
        });
      }}>SUBMIT</button
    >
  </div>
</Dialog>

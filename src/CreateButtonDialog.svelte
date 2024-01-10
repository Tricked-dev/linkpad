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
  import { ws } from "./ws.svelte";

  interface Props {
    onSubmit: () => void;
    onCancel: () => void;
    open: boolean;
    data: AnyButton;
  }

  let { onSubmit, onCancel, data, open } = $props<Props>();

  let checked = $state(false);

  let iconId = $derived(data?.id);

  $effect(() => {
    untrack(() => {
      checked = data.type === DisplayType.ICON;
    });
  });

  $effect(() => {
    if (checked) {
      data.type = DisplayType.TEXT;
    } else {
      data.type = DisplayType.ICON;
    }
  });

  $effect(() => {
    // open changes to false while the submitted state is false
    if (!open && !submitted) {
      onCancel();
    }
  });

  let submitted = $state(false);

  function readFile(accept: string): Promise<File> {
    return new Promise((resolve, reject) => {
      const input = document.createElement("input");
      input.type = "file";
      input.accept = accept;

      input.addEventListener("change", () => {
        const file = input.files![0];

        if (!file) {
          reject(new Error("No file selected"));
          return;
        }
        resolve(file);
      });

      // Trigger the file input click event
      input.click();
    });
  }

  function blobToBase64(blob: Blob | File): Promise<string> {
    return new Promise((resolve, _) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result as string);
      reader.readAsDataURL(blob);
    });
  }

  async function selectIcon() {
    const file = await readFile("image/*");

    data.icon = URL.createObjectURL(file);

    ws.send("UpsertIcon", {
      id: data.id,
      data: (await blobToBase64(file)).split(";base64,")[1],
      content_type: file.type,
    });
  }
</script>

<Dialog bind:open title="Edit Button! ">
  <div class="h-110 w-100">
    <div class="flex gap-2">
      <span>ICON</span>
      <Toggle bind:checked />
      <span>TEXT</span>
    </div>
    <IconPicker />
    {#if data.type === DisplayType.TEXT}
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
        <Button text="Select Icon" onclick={selectIcon} />
      </div>
    {/if}
    <span class="text-xl font-bold"> Select Action </span>
    <div class="p-2">
      <Button
        text="Browse"
        onclick={() => {
          console.log(ws.actions);
        }}
      />
      <span> Increase Volume </span>
    </div>
    <div class="flex justify-center items-center mt-10">
      <ActionButton {data} onEdit={() => {}} preview edit={false} />
    </div>
    <Button
      onclick={() => {
        untrack(() => {
          submitted = true;
        });
        onSubmit();
        open = false;
        untrack(() => {
          submitted = false;
        });
      }}
      text="submit"
    />
  </div>
</Dialog>

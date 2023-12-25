<script lang="ts">
  import { onMount } from "svelte";
  import Dialog from "./Dialog.svelte";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Toggle from "./Toggle.svelte";
  import Checkbox from "./Checkbox.svelte";
  import ActionButton from "./ActionButton.svelte";
  import IconPicker from "./IconPicker.svelte";

  const css =
    "rounded-24px bg-surface-200/30 backdrop-blur-4 p-4 w-160px h-160px font-bold text-white hover:bg-surface-200/50 duration-200 text-center";

  const edit = true;
  let open = $state(true);
  let checked = $state(true);

  const enum DisplayType {
    ICON = 0,
    TEXT = 1,
  }

  type TextButton = {
    type: DisplayType.TEXT;
    autoSize: boolean;
    text: string;
    textSize?: number;
  };
  type IconButton = {
    type: DisplayType.ICON;
    icon: string;
  };

  type AnyButton = Omit<Partial<TextButton> & Partial<IconButton>, "type"> & {
    type: DisplayType;
    id: string;
  };

  const buttonOption: AnyButton = $state({
    type: DisplayType.ICON,
    id: "123",
  });

  $effect(() => {
    if (checked) {
      buttonOption.type = DisplayType.ICON;
    } else {
      buttonOption.type = DisplayType.TEXT;
    }
  });

  function b(text: string): AnyButton {
    return {
      id: "123",
      type: DisplayType.TEXT,
      autoSize: true,
      text: text,
    };
  }

  const data: AnyButton[][] = [
    [b("btn 1"), b("MUTE"), b("Some cool action")],
    [b("Other"), b("Things"), b("DIE"), b("Download NOW")],
    [b("This row"), b("Is kinda empty")],
    [b("btn 4"), b("btn 5"), b("btn 6"), b("btn 7")],
    [
      b("Scrolley"),
      b("Scroll"),
      b("scroll"),
      b("scroll"),
      b("scroll"),
      b("scroll"),
      b("scroll"),
    ],
  ];
</script>

<main
  class="[font-family:'Lato'] [background-image:url('/bg3.png')] h-screen scrollbar scrollbar-w-0 scrollbar-rounded"
  data-theme="ato"
>
  <div
    class="bg-surface-900/20 backdrop-blur-50 p-4 text-surface-100 flex sticky"
  >
    <span class="text text-3xl">LINK PAD</span>
    {#if edit}<span class="ml-auto mt-auto">Edit mode</span>{/if}
  </div>

  <div class="p-4 flex flex-col gap-4 w-auto">
    {#each data as row}
      <div class="flex gap-4">
        {#each row as item}
          <ActionButton {css} data={item} bind:open {edit} />
        {/each}
        {#if edit}<button class={`${css} border-tertiary-500 border-4`}>
            <div class="i-mdi-plus text-9xl"></div>
          </button>{/if}
      </div>
    {/each}
  </div>
  <Dialog bind:open title="Edit Button! ">
    <div class="h-110 w-100">
      <div class="flex gap-2">
        <span>ICON</span>
        <Toggle bind:checked />
        <span>TEXT</span>
      </div>
      <IconPicker />
      {#if buttonOption.type === DisplayType.ICON}
        <div class="p-2 grid grid-cols-2 gap-2">
          <Input
            class="col-span-2"
            name="buttontext"
            placeholder="Button Text"
            bind:value={buttonOption.text}
          />
          <Checkbox
            id="autosize"
            bind:checked={buttonOption.autoSize}
            label="Autosize"
          />
          {#if !buttonOption.autoSize}
            <label>
              Text Size
              <input
                type="range"
                bind:value={buttonOption.textSize}
                min="10"
                max="200"
              />
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
        <ActionButton {css} data={buttonOption} {open} {edit} />
      </div>
    </div>
  </Dialog>
</main>

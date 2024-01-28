<script lang="ts">
  import { onMount } from "svelte";
  import Dialog from "./Dialog.svelte";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Toggle from "./Toggle.svelte";
  import Checkbox from "./Checkbox.svelte";
  import ActionButton from "./ActionButton.svelte";
  import IconPicker from "./IconPicker.svelte";
  import CreateButtonDialog from "./CreateButtonDialog.svelte";
  import { type AnyButton, DisplayType, type Info } from "./types";
  import { fly } from "svelte/transition";
  import { randomUUID } from "./utils";
  import { ws } from "./ws.svelte";

  const edit = true;

  let activeBoard = $derived(ws.info?.find((x) => x.id == ws.active));

  let open = $state(false);

  let onCancel: () => void = $state(() => {});
  let onSubmit: () => void = $state(() => {});

  let buttonOption: AnyButton = $state({
    type: DisplayType.ICON,
    action: "",
    id: "123",
  });

  onMount(() => {
    ws.connect();
  });

  $effect(() => {
    if (ws?.state !== "CLOSED" && ws.info) {
      ws?.save();
    }
  });
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
    {#each activeBoard?.buttons || [] as row, rowIdx}
      <div class="flex gap-4" in:fly={{ y: -100, duration: 500 }}>
        {#each row as item, itemIdx}
          <div in:fly={{ x: -100, duration: 500 }}>
            <ActionButton
              data={item}
              onEdit={() => {
                open = true;
                buttonOption = { ...item };
                onCancel = () => {};
                onSubmit = () => {
                  activeBoard!.buttons[rowIdx].splice(itemIdx, 1, buttonOption);
                };
              }}
              {edit}
            />
          </div>
        {/each}
        {#if edit}<button
            class={`rounded-24px bg-surface-200/30 backdrop-blur-4 p-4 w-160px h-160px font-bold text-white hover:bg-surface-200/50 duration-200 text-center border-tertiary-500 border-4`}
            onclick={() => {
              open = true;
              buttonOption = {
                type: DisplayType.ICON,
                action: "123",
                id: randomUUID(),
              };
              onCancel = () => {};
              onSubmit = () => {
                activeBoard?.buttons[rowIdx].push(buttonOption);
              };
            }}
          >
            <div class="i-mdi-plus text-9xl"></div>
          </button>{/if}
      </div>
    {/each}
    {#if edit}<button
        class={`text-white`}
        onclick={() => {
          activeBoard?.buttons.push([]);
        }}
      >
        <div class="i-mdi-plus text-2xl px-6 border-primary-500 border-2"></div>
      </button>{/if}
  </div>
  <CreateButtonDialog
    bind:open
    bind:data={buttonOption}
    {onCancel}
    {onSubmit}
  />
</main>

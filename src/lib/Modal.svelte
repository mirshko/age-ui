<script lang="ts">
  export let showModal: boolean;

  let dialog: HTMLDialogElement;

  $: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
  bind:this={dialog}
  on:close={() => (showModal = false)}
  on:click|self={() => dialog.close()}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="bg-white rounded-lg p-6 transition-all shadow-xl text-left overflow-hidden transform w-full"
    on:click|stopPropagation
  >
    <slot />
  </div>
</dialog>

<style>
  dialog {
    width: 100%;
    max-width: 24rem;
    border-radius: 0.2em;
    border: none;
    padding: 0;
    background-color: transparent;
  }
  dialog::backdrop {
    @apply bg-gray-500/75 transition-opacity;
  }

  dialog[open] {
    animation: modalEnter 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  @keyframes modalEnter {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  dialog[open]::backdrop {
    animation: backdropEnter 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  @keyframes backdropEnter {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>

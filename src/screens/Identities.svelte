<script lang="ts">
  import { dialog, invoke } from "@tauri-apps/api";
  import { useSWR } from "sswr";
  import IdentityForm from "../lib/IdentityForm.svelte";
  import Modal from "../lib/Modal.svelte";
  import { store } from "../lib/store";

  let showModal = false;

  const { data: identities, revalidate } = useSWR("identities", {
    fetcher: async () => await store.entries(),
  });

  function deleteIdentity(deleteIdentity: string) {
    return async () => {
      try {
        const confirmed = await dialog.confirm(
          "Are you sure you want to delete this identity?"
        );

        if (!confirmed) return;

        await store.delete(deleteIdentity);

        await store.save();

        await revalidate();
      } catch (error) {
        dialog.message(error.message, {
          type: "error",
        });
      }
    };
  }
</script>

<div class="border-b border-gray-200 pb-5 flex justify-between items-center">
  <h3 class="text-base font-semibold leading-6 text-gray-900 select-none">
    Identities
  </h3>

  <button
    on:click={() => (showModal = true)}
    type="button"
    class="inline-flex items-center select-none rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
  >
    Add new identity
  </button>
</div>

<Modal bind:showModal>
  <IdentityForm />
</Modal>

<ul class="divide-y divide-gray-200">
  {#if $identities}
    {#each $identities as [key, value]}
      <li class="flex items-center justify-between gap-x-6 py-5">
        <div class="min-w-0 flex-auto">
          <p class="text-sm font-semibold leading-6 text-gray-900">
            {key}
          </p>
          <p class="mt-1 truncate text-xs leading-5 text-gray-500">
            {value}
          </p>
        </div>

        <button
          on:click={deleteIdentity(key)}
          class="rounded-full bg-white px-2.5 py-1 text-xs font-semibold text-gray-900 select-none shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
        >
          Delete
        </button>
      </li>
    {/each}
  {/if}
</ul>

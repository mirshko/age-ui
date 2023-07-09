<script lang="ts">
  import { dialog } from "@tauri-apps/api";
  import IdentityForm from "../lib/IdentityForm.svelte";
  import { store } from "../lib/store";
  import { useSWR } from "sswr";

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
  <h3 class="text-base font-semibold leading-6 text-gray-900">Identities</h3>

  <button
    type="button"
    class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
  >
    Add new identity
  </button>
</div>

<!-- <IdentityForm /> -->

<div class="mt-4">
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
            class="rounded-full bg-white px-2.5 py-1 text-xs font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
          >
            Delete
          </button>
        </li>
      {/each}
    {/if}
  </ul>

  <!-- <button
          class="flex w-full items-center justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus-visible:outline-offset-0"
        >
          View all
        </button> -->
</div>

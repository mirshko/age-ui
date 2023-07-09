<script lang="ts">
  import IdentityForm from "./lib/IdentityForm.svelte";
  import { store } from "./lib/store";
  import { Tabs, TabList, TabPanel, Tab } from "./lib/tabs";
  import { dialog } from "@tauri-apps/api";
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

<main class="p-5">
  <Tabs>
    <TabList>
      <Tab>Identities</Tab>
      <Tab>Encrypt</Tab>
      <Tab>Decrypt</Tab>
    </TabList>

    <TabPanel>
      <IdentityForm />

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
    </TabPanel>

    <TabPanel>
      <div>
        <label
          for="identity"
          class="block text-sm font-medium leading-6 text-gray-900"
        >
          Identity
        </label>

        <select
          id="identity"
          name="identity"
          class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
        >
          <option value="">Select</option>
          {#if $identities}
            {#each $identities as [_, value]}
              <option {value}>{value}</option>
            {/each}
          {/if}
        </select>
      </div>
    </TabPanel>

    <TabPanel>
      <h2>Third panel</h2>
    </TabPanel>
  </Tabs>
</main>

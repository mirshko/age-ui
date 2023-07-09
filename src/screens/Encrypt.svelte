<script lang="ts">
  import { useSWR } from "sswr";
  import { store } from "../lib/store";

  const { data: identities } = useSWR("identities", {
    fetcher: async () => await store.entries(),
  });
</script>

<div class="border-b border-gray-200 pb-5 flex justify-between items-center">
  <h3 class="text-base font-semibold leading-6 text-gray-900">Encrypt</h3>

  <select
    name="identity"
    class="max-w-xs block rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
  >
    <option value="">Select Identity</option>
    {#if $identities}
      {#each $identities as [key, value]}
        <option {value}>{key}</option>
      {/each}
    {/if}
  </select>

  <!-- <button
    type="button"
    class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
  >
    Add new identity
  </button> -->
</div>

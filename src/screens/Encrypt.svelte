<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { useSWR } from "sswr";
  import { store } from "../lib/store";

  const { data: identities } = useSWR("identities", {
    fetcher: () => store.entries(),
  });

  let publicKey = "";
  let data = "";

  let encryptedResult = "";

  async function encrypt(publicKey: string, data: string) {
    try {
      const res = await invoke<number[]>("encrypt_with_x25519", {
        publicKey,
        data,
      });

      encryptedResult = new TextDecoder().decode(new Uint8Array(res));
    } catch (error) {
      console.error(error);
    }
  }

  async function handleSubmit() {
    await encrypt(publicKey, data);
  }
</script>

<div class="grid grid-rows-2 gap-4 flex-1">
  <form class="relative" on:submit|preventDefault={handleSubmit}>
    <textarea
      required
      name="data"
      id="data"
      placeholder="Enter data to encrypt"
      class="w-full h-full border-gray-400 bg-gray-300 resize-none rounded-md"
      bind:value={data}
    />

    <div class="absolute bottom-4 right-4 flex gap-4">
      <select
        bind:value={publicKey}
        name="identity"
        class="block rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6 select-none"
      >
        <option value="">Select...</option>
        {#if $identities}
          {#each $identities as [key, value]}
            <option {value}>{key}</option>
          {/each}
        {/if}
      </select>

      <button
        disabled={!publicKey || !data}
        class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-50"
        type="submit"
      >
        Encrypt
      </button>
    </div>
  </form>

  <div
    class="bg-gray-600 rounded-md border overflow-scroll border-gray-500 text-white px-4 py-3"
  >
    <div class="whitespace-pre-wrap">{encryptedResult}</div>
  </div>
</div>

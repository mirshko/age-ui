<script lang="ts">
  import { store } from "./store";
  import { nanoid } from "nanoid";

  let recipient = "";

  async function handleSubmit() {
    await store.set(nanoid(), recipient);

    await store.save();
  }
</script>

<div>
  <form on:submit|preventDefault={handleSubmit}>
    <div class="flex gap-2">
      <input
        placeholder="Enter an age recipient..."
        required
        bind:value={recipient}
        type="text"
        class="block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 flex-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
      />

      <button
        type="submit"
        class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
      >
        Save Recipient
      </button>
    </div>
  </form>
</div>

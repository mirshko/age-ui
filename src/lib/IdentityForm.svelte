<script lang="ts">
  import { store } from "./store";
  import { nanoid } from "nanoid";
  import { revalidate } from "sswr";

  let recipient = "";

  let error: string | undefined = undefined;

  const ageRegex = new RegExp(/^age[0-9a-z]{59}$/);

  async function handleSubmit() {
    error = undefined;

    recipient = recipient.trim();

    const validated = ageRegex.test(recipient);

    if (!validated) {
      error = "Invalid recipient";

      return;
    }

    await store.set(nanoid(), recipient);

    await store.save();

    await revalidate("identities");
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
        class={`block rounded-md border-0 py-1.5 shadow-sm ring-1 flex-1 ring-inset sm:text-sm sm:leading-6 focus:ring-2 focus:ring-inset ${
          typeof error === "string"
            ? "text-red-900 ring-red-300 placeholder:text-red-300 focus:ring-red-500"
            : "text-gray-900 ring-gray-300 placeholder:text-gray-400 focus:ring-indigo-600"
        }`}
        aria-invalid={typeof error === "string" ? "true" : undefined}
        aria-describedby={typeof error === "string"
          ? "recipient-error"
          : undefined}
        autocapitalize="off"
        autocomplete="off"
        autocorrect="off"
      />

      <button
        type="submit"
        class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
      >
        Save Recipient
      </button>
    </div>

    {#if error}
      <p class="mt-2 text-sm text-red-600" id="recipient-error">{error}</p>
    {/if}
  </form>
</div>

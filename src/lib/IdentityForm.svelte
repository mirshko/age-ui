<script lang="ts">
  import { store } from "./store";

  let label = "";
  let recipient = "";

  let error: string | undefined = undefined;

  const ageRegex = new RegExp(/^age[0-9a-z]{59}$/);

  async function handleSubmit() {
    error = undefined;

    label = label.trim();
    recipient = recipient.trim();

    const validated = ageRegex.test(recipient);

    if (!validated) {
      error = "Invalid recipient";

      return;
    }

    await store.set(label, recipient);

    await store.save();
  }
</script>

<div>
  <form class="flex flex-col gap-4" on:submit|preventDefault={handleSubmit}>
    <div>
      <input
        placeholder="Enter a label"
        required
        bind:value={label}
        type="text"
        class={`block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 flex-1 ring-inset sm:text-sm sm:leading-6 focus:ring-2 focus:ring-inset text-gray-900 ring-gray-300 placeholder:text-gray-400 focus:ring-indigo-600`}
      />
    </div>

    <div>
      <input
        placeholder="Enter an age recipient"
        required
        bind:value={recipient}
        type="text"
        class={`block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 flex-1 ring-inset sm:text-sm sm:leading-6 focus:ring-2 focus:ring-inset ${
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

      {#if error}
        <p class="mt-2 text-sm text-red-600" id="recipient-error">{error}</p>
      {/if}
    </div>

    <button
      type="submit"
      class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
    >
      Save Recipient
    </button>
  </form>
</div>
